use std::env; // For reading command-line arguments
use std::path::Path; // For checking paths, creating directories
use image::{imageops, GrayImage, RgbaImage}; // Only necessary imports for image processing
mod dicelib;
use dicelib::{add_reference_text, map_intensity_to_dice_side, load_image, Dice, DiceSides};

struct Images {
    input: GrayImage,
    dice: [Dice; 6]
}


fn load_dice_images_d(dice_dir: &str) -> [Dice; 6] {
    // Read the directory and collect file paths
    let mut dice_image_paths: Vec<_> = std::fs::read_dir(dice_dir)
        .expect("Failed to read dice directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    // Ensure there are exactly 6 images
    if dice_image_paths.len() != 6 {
        panic!("The directory must contain exactly 6 dice images.");
    }

    // Sort the paths to ensure consistent order (e.g., by filename)
    dice_image_paths.sort();


    //Ask for Dice Size
    println!("Enter the desired dice size (e.g., 32 for 32x32 pixels):");
        let mut d_size_input = String::new();
        std::io::stdin().read_line(&mut d_size_input).unwrap();
        let d_size: u32 = match d_size_input.trim_end().parse() {
            Ok(size) if size > 0 => size,
            _ => {
                println!("Invalid dice size. Using default size of 32x32.");
                32
            }
        };
    // Load and resize the dice images
    let mut dice_array: [Option<Dice>; 6] = Default::default();
    for (i, image_path) in dice_image_paths.iter().enumerate() {
        let image_data = std::fs::read(image_path)
            .unwrap_or_else(|_| panic!("Failed to read dice image at {:?}", image_path));
        let image = image::load_from_memory(&image_data)
            .expect(&format!("Failed to load dice image {:?}", image_path));
        
    
        // Resize the image
        let resized_image = image.resize_exact(d_size, d_size, imageops::FilterType::Lanczos3);
        // let resized_image = image.resize_exact(
        //     target_width,
        //     target_height,
        //     imageops::FilterType::Lanczos3, // High-quality resizing filter
        // );

        dice_array[i] = Some(Dice {
            side: match i {
                0 => DiceSides::One,
                1 => DiceSides::Two,
                2 => DiceSides::Three,
                3 => DiceSides::Four,
                4 => DiceSides::Five,
                5 => DiceSides::Six,
                _ => unreachable!(),
            },
            image: resized_image,
        });
    }

    core::array::from_fn(|i| dice_array[i].clone().expect("Error loading dice images into array"))
}


fn load_images_dynamic() -> Images {

        let matches = clap::Command::new("Dice Image Processor")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Processes an image and converts it into a dice representation")
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

        let target_width: u32 = 32; // Default dice width
        let target_height: u32 = 32; // Default dice height
            //Load input
        let mut i: GrayImage = load_image(&input); // Assuming load_image handles potential resize
        let (original_width, original_height) = i.dimensions(); // Store original dimensions

        let mut dice = load_dice_images_d(&dice_dir);
        

        //Ask if the user wants to invert the image
        println!("Do you want to invert the input image? (y/n):");
        let mut invert_i = String::new();
        std::io::stdin().read_line(&mut invert_i).unwrap();
        if invert_i.trim().eq_ignore_ascii_case("y") {
            imageops::invert(&mut i);
            println!("The image has been inverted.");
        } else {
            println!("The image will not be inverted.");
        };

        // Resize the input image to the nearest square
        let square_size = original_width.min(original_height); // Use the smaller dimension
        i = imageops::crop_imm(&i, 0, 0, square_size, square_size).to_image();
         //Ask if the user wants to invert the dice colors
         println!("Do you want to invert the dice colors? (y/n):");
         let mut invert_dice_i = String::new();
         std::io::stdin().read_line(&mut invert_dice_i).unwrap();
         let invert_dice = invert_dice_i.trim().eq_ignore_ascii_case("y");
 
         if invert_dice {
             for d in &mut dice {
                 d.image.invert();
             }
             println!("The dice colors have been inverted.");
         } else {
             println!("The dice colors will not be inverted.");
         }

        Images {
            dice,
            input: i,
        }
}


fn validate_input(i: String) -> Option<String> {
        if Path::new(&i).exists() {
            return Some(i)
        } else {
            eprintln!("The provided file path does not exist: {}", i);
            println!("Press Enter to exit...");
            let mut i = String::new();
            std::io::stdin().read_line(&mut i).unwrap(); //i mean... really should just unwrap here... NOTE: LOOK INTO PANICK CONTROL
            return None;
        }
}
fn main() {
//  CLAP CL    // let matches = clap::Command::new("Dice Image Processor")
    //     .version("1.0")
    //     .author("Your Name <your.email@example.com>")
    //     .about("Processes an image and converts it into a dice representation")
    //     .arg(
    //         clap::Arg::new("input")
    //             .short('i')
    //             .long("input")
    //             .value_name("INPUT_FILE")
    //             .help("Path to the input image file")
    //             .required(true)
    //             .num_args(1),
    //     )
    //     .get_matches();

    // let input_file = matches
    //     .get_one::<String>("input")
    //     .expect("Input file is required").to_string();


    // // let args: Vec<String> = env::args().collect();
    // // let i = if args.len() < 2 {
    // //     println!("No file provided.");
    // //     println!("Please drag and drop an image file into this window or type the file path:");
    // //     let mut i = String::new();
    // //     std::io::stdin().read_line(&mut i).unwrap();
    // //     let t = i.trim(); // t = trimmed file path
    // //     if t.is_empty() {
    // //         println!("No file path provided. Exiting...");
    // //         return;
    // //     }
    // //     t.to_string()
    // // } else {
    // //     args[1].clone()
    // // };
    // let v = validate_input(input_file).expect("Could not validate input file");

    // //Ask for Dice Size
    // println!("Enter the desired dice size (e.g., 32 for 32x32 pixels):");
    // let mut d_size_input = String::new();
    // std::io::stdin().read_line(&mut d_size_input).unwrap();
    // let d_size: u32 = match d_size_input.trim().parse() {
    //     Ok(size) if size > 0 => size,
    //     _ => {
    //         println!("Invalid dice size. Using default size of 32x32.");
    //         32
    //     }
    // };


    //Load Dice
    let dicks: Images = load_images_dynamic();
    // let mut dicks: [Dice; 6] = load_dice_images_static(d_size); // Pass dice size to the function
    if dicks.dice.is_empty() || dicks.dice[0].image.width() == 0 || dicks.dice[0].image.height() == 0 {
        eprintln!("Error loading dice or dice have invalid dimensions.");
        return;
    }


    let (iwidth, iheight) = dicks.input.dimensions();
    println!("{} {}", iwidth, iheight);

    let d_size = dicks.dice[0].image.width();

    //Calculate Grid
    let dw = d_size; // Define dice width based on dice size
    let dh = d_size; // Define dice height based on dice size
    let num_dice_x = iwidth / dw;
    let num_dice_y = iheight / dh;

    if num_dice_x == 0 || num_dice_y == 0 {
        eprintln!("Input image too small for dice dimensions.");
        return;
    }

    //Prepare Output Image
    let mut ow = num_dice_x * dw;
    let mut oh = num_dice_y * dh;
    let mut oi = RgbaImage::new(ow, oh);

    //Map Blocks and Construct Output
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

            let target_side: DiceSides = map_intensity_to_dice_side(avg_intensity);

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
        
    };

    

        match resize_output(&oi) {
            Some(r) => {
                oi = r;
                let (now, noh) = oi.dimensions();
                ow = now;
                oh = noh;
            }
            None => {}
        }

    //Save Output
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

    //Keep CMD Window Open
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}



fn resize_output(oi: &RgbaImage) -> Option<RgbaImage> {
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
        let mut resized_oi = RgbaImage::new(output_width, output_height);

        // Calculate the aspect ratio of the original image
        let (oi_width, oi_height) = oi.dimensions();
        let aspect_ratio = oi_width as f32 / oi_height as f32;

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
        let scaled_oi = image::imageops::resize(
            oi,
            new_width,
            new_height,
            imageops::FilterType::Lanczos3, // High-quality resizing filter
        );

        // Calculate the offsets to center the scaled image
        let offset_x = ((output_width - new_width) / 2) as i64;
        let offset_y = ((output_height - new_height) / 2) as i64;

        // Overlay the scaled image onto the new blank image
        imageops::overlay(&mut resized_oi, &scaled_oi, offset_x, offset_y);

        Some(resized_oi)
    } else {
        None
    }
}
