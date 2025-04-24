// use std::env; // For grabbing command-line arguments... no need for clap really.
use std::path::Path; // Handy for working with file paths
use image::{imageops, GrayImage, RgbaImage}; // Just the essentials for image processing
mod dicelib;
use dicelib::{add_reference_text, map_intensity_to_dice_side, load_image, Dice, DiceSides, IntensityPreset};

struct Images {
    input: GrayImage,
    dice: [Dice; 6] // Six dice, one for each side. Simple and clean.
}

fn load_dice_images_d(dice_dir: &str) -> [Dice; 6] {
    // Read the dice directory and grab all the files
    let mut dice_image_paths: Vec<_> = std::fs::read_dir(dice_dir)
        .expect("Couldn't find the dice directory. Double-check your path!")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    // Make sure we have exactly 6 dice images. No more, no less.
    if dice_image_paths.len() != 6 {
        panic!("You need exactly 6 dice images. No shortcuts allowed!");
    }

    // Sort the paths to keep things consistent (alphabetical order is nice)
    dice_image_paths.sort();

    // Ask the user for the dice size
    println!("Enter the dice size you want (e.g., 32 for 32x32 pixels):");
    let mut d_size_input = String::new();
    std::io::stdin().read_line(&mut d_size_input).unwrap();
    let d_size: u32 = match d_size_input.trim_end().parse() {
        Ok(size) if size > 0 => size,
        _ => {
            println!("Invalid size. Defaulting to 32x32.");
            32
        }
    };

    // Load and resize the dice images
    let mut dice_array: [Option<Dice>; 6] = Default::default();
    for (i, image_path) in dice_image_paths.iter().enumerate() {
        let image_data = std::fs::read(image_path)
            .unwrap_or_else(|_| panic!("Couldn't read dice image at {:?}", image_path));
        let image = image::load_from_memory(&image_data)
            .expect(&format!("Couldn't load dice image {:?}", image_path));

        // Resize the dice image to the user-specified size
        let resized_image = image.resize_exact(d_size, d_size, imageops::FilterType::Lanczos3);

        dice_array[i] = Some(Dice {
            side: match i {
                0 => DiceSides::One,
                1 => DiceSides::Two,
                2 => DiceSides::Three,
                3 => DiceSides::Four,
                4 => DiceSides::Five,
                5 => DiceSides::Six,
                _ => unreachable!(), // This should never happen. If it does, something's seriously wrong.
            },
            image: resized_image,
        });
    }

    // Convert the Option array into a proper array. No room for None here.
    core::array::from_fn(|i| dice_array[i].clone().expect("Failed to load dice images."))
}

fn load_images_dynamic() -> Images {
    // Using clap for argument parsing. Because why not?
    let matches = clap::Command::new("Dice Image Processor")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Turns your images into dice art. Pretty cool, huh?")
        .arg(
            clap::Arg::new("input")
                .short('i')
                .long("input")
                .value_name("INPUT_FILE")
                .help("Path to the input image file")
                .required(true)
                .num_args(1),
        )
        .arg(
            clap::Arg::new("dice_dir")
                .short('d')
                .long("dice-dir")
                .value_name("DICE_DIRECTORY")
                .help("Path to the directory containing dice images (exactly 6 images)")
                .required(true)
                .num_args(1),
        )
        .get_matches();

    let input = matches
        .get_one::<String>("input")
        .expect("Input file is required")
        .to_string();

    let dice_dir = matches
        .get_one::<String>("dice_dir")
        .expect("Dice directory is required")
        .to_string();

    // Load the input image
    let mut i: GrayImage = load_image(&input);
    let (original_width, original_height) = i.dimensions(); // Save the original dimensions

    let mut dice = load_dice_images_d(&dice_dir);

    // Ask if the user wants to invert the input image
    println!("Invert the input image? (y/n):");
    let mut invert_i = String::new();
    std::io::stdin().read_line(&mut invert_i).unwrap();
    if invert_i.trim().eq_ignore_ascii_case("y") {
        imageops::invert(&mut i);
        println!("Image inverted.");
    } else {
        println!("Keeping it original. I like it.");
    }

    // Crop the input image to a square. Dice grids love squares.
    let square_size = original_width.min(original_height);
    i = imageops::crop_imm(&i, 0, 0, square_size, square_size).to_image();

    // Ask if the user wants to invert the dice colors
    println!("Invert the dice colors? (y/n):");
    let mut invert_dice_i = String::new();
    std::io::stdin().read_line(&mut invert_dice_i).unwrap();
    let invert_dice = invert_dice_i.trim().eq_ignore_ascii_case("y");

    if invert_dice {
        for d in &mut dice {
            d.image.invert();
        }
        println!("Dice colors inverted. Edgy.");
    } else {
        println!("Dice colors untouched. Classic.");
    }

    Images {
        dice,
        input: i,
    }
}

fn validate_input(i: String) -> Option<String> {
    // Check if the file exists. If not, yell at the user.
    if Path::new(&i).exists() {
        Some(i)
    } else {
        eprintln!("File not found: {}", i);
        println!("Press Enter to exit...");
        let mut i = String::new();
        std::io::stdin().read_line(&mut i).unwrap();
        None
    }
}

fn main() {
    // Load the dice and input image
    let mut dicks: Images = load_images_dynamic();
    if dicks.dice.is_empty() || dicks.dice[0].image.width() == 0 || dicks.dice[0].image.height() == 0 {
        eprintln!("Dice images are missing or invalid. Fix it.");
        return;
    }

    let dw = dicks.dice[0].image.width();
    let dh = dicks.dice[0].image.height();

    // Resize the input image if needed
    match resize_output(&dicks.input) {
        Some(resized_input) => {
            dicks.input = resized_input;
        }
        None => {}
    }

    // Recalculate grid dimensions based on the resized input image
    let (iwidth, iheight) = dicks.input.dimensions();
    let num_dice_x = iwidth / dw;
    let num_dice_y = iheight / dh;

    // Prepare the output image
    let mut ow = num_dice_x * dw;
    let mut oh = num_dice_y * dh;
    let mut oi = RgbaImage::new(ow, oh);

    // Ask the user for an intensity preset
    println!("Pick your intensity preset:");
    println!("1. Default");
    println!("2. High Contrast");
    println!("3. Low Contrast");
    println!("4. Bright");
    println!("5. Dark");

    let mut preset_input = String::new();
    std::io::stdin().read_line(&mut preset_input).unwrap();
    let preset = match preset_input.trim() {
        "1" => IntensityPreset::Default,
        "2" => IntensityPreset::HighContrast,
        "3" => IntensityPreset::LowContrast,
        "4" => IntensityPreset::Bright,
        "5" => IntensityPreset::Dark,
        _ => {
            println!("Invalid choice. Defaulting to Default preset.");
            IntensityPreset::Default
        }
    };

    // Map blocks to dice and construct the output
    for grid_y in 0..num_dice_y {
        for grid_x in 0..num_dice_x {
            let block_start_x = grid_x * dw;
            let block_start_y = grid_y * dh;

            let block_view = imageops::crop_imm(&dicks.input, block_start_x, block_start_y, dw, dh);

            let mut total_intensity: u64 = 0;
            let num_pixels_in_block = (dw * dh) as u64;
            for pixel in block_view.to_image().pixels() {
                total_intensity += pixel[0] as u64;
            }
            let avg_intensity = if num_pixels_in_block > 0 {
                (total_intensity / num_pixels_in_block) as u8
            } else {
                0
            };

            let target_side: DiceSides = map_intensity_to_dice_side(avg_intensity, &preset);

            let dice_to_draw = dicks.dice.iter().find(|&d| d.side == target_side);

            if let Some(dice) = dice_to_draw {
                let paste_x = grid_x * dw;
                let paste_y = grid_y * dh;
                let dice_rgba = dice.image.to_rgba8();
                imageops::overlay(&mut oi, &dice_rgba, paste_x as i64, paste_y as i64);
            } else {
                eprintln!(
                    "Warning: Could not find dice for side {:?} at grid ({}, {})",
                    target_side, grid_x, grid_y
                );
            }
        }
    }

    println!("Do you want to add debug info to output image? (y/n):");
    let mut invert_i = String::new();
    std::io::stdin().read_line(&mut invert_i).unwrap();
    if invert_i.trim().eq_ignore_ascii_case("y") {
        add_reference_text(
            &mut oi,
            (dw, dh),
            num_dice_x * num_dice_y,
            (ow, oh),
        );
        println!("Debug info added to image");
    } else {
        println!("No debug info added.");
    }

    // Save the output image
    let output_path = "output/dice_output.png";
    if let Some(parent_dir) = Path::new(output_path).parent() {
        std::fs::create_dir_all(parent_dir).expect("Failed to create output directory");
    }
    oi.save(output_path).unwrap_or_else(|err| {
        eprintln!("Error saving output image: {}", err);
    });
    println!("Original image size: {}x{}", iwidth, iheight);
    println!("Dice size used: {}x{}", dw, dh);
    println!("Total dice used: {}", num_dice_x * num_dice_y);
    println!("Output image size: {}x{}", ow, oh);
    println!("Output saved to {}", output_path);

    // Keep the window open
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn resize_output(input: &GrayImage) -> Option<GrayImage> {
    println!("Do you want to set a custom output image size? (y/n):");
    let mut custom_size_input = String::new();
    std::io::stdin().read_line(&mut custom_size_input).unwrap();
    if custom_size_input.trim().eq_ignore_ascii_case("y") {
        println!("Enter the desired output image width (e.g., 1920 for desktop wallpaper):");
        let mut output_width_input = String::new();
        std::io::stdin().read_line(&mut output_width_input).unwrap();
        let output_width: u32 = match output_width_input.trim().parse() {
            Ok(width) if width > 0 => width,
            _ => {
                println!("Invalid width. Using default width of 1920.");
                1920
            }
        };

        println!("Enter the desired output image height (e.g., 1080 for desktop wallpaper):");
        let mut output_height_input = String::new();
        std::io::stdin().read_line(&mut output_height_input).unwrap();
        let output_height: u32 = match output_height_input.trim().parse() {
            Ok(height) if height > 0 => height,
            _ => {
                println!("Invalid height. Using default height of 1080.");
                1080
            }
        };

        println!("Custom output size set to {}x{}", output_width, output_height);

        // Create a new blank image with the desired dimensions
        let mut resized_input = GrayImage::new(output_width, output_height);

        // Calculate the aspect ratio of the original image
        let (input_width, input_height) = input.dimensions();
        let aspect_ratio = input_width as f32 / input_height as f32;

        // Calculate the new dimensions for the original image while maintaining aspect ratio
        let (new_width, new_height) = if output_width as f32 / output_height as f32 > aspect_ratio {
            // Constrain by height
            let new_width = (output_height as f32 * aspect_ratio).round() as u32;
            (new_width, output_height)
        } else {
            // Constrain by width
            let new_height = (output_width as f32 / aspect_ratio).round() as u32;
            (output_width, new_height)
        };

        // Resize the original image to the new dimensions
        let scaled_input = image::imageops::resize(
            input,
            new_width,
            new_height,
            imageops::FilterType::Lanczos3, // High-quality resizing filter
        );

        // Calculate the offsets to center the scaled image
        let offset_x = ((output_width - new_width) / 2) as i64;
        let offset_y = ((output_height - new_height) / 2) as i64;

        // Overlay the scaled image onto the new blank image
        imageops::overlay(&mut resized_input, &scaled_input, offset_x, offset_y);

        Some(resized_input)
    } else {
        None
    }
}
