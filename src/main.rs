use std::env; // For reading command-line arguments
use std::path::Path; // For checking paths, creating directories
use image::{imageops, GrayImage, RgbaImage}; // Only necessary imports for image processing
mod dicelib;
use dicelib::{add_reference_text, map_intensity_to_dice_side, load_image, load_dice_images, Dice, DiceSides};

pub fn load_dice_images_static() -> [Dice; 6] {
    let dice_images: [&[u8]; 6] = [
        include_bytes!("../dice/1side.png").as_ref(),
        include_bytes!("../dice/2side.png").as_ref(),
        include_bytes!("../dice/3side.png").as_ref(),
        include_bytes!("../dice/4side.png").as_ref(),
        include_bytes!("../dice/5side.png").as_ref(),
        include_bytes!("../dice/6side.png").as_ref(),
    ];

    let target_width: u32 = 32;  // Desired width for resizing
    let target_height: u32 = 32; // Desired height for resizing

    let mut dice_array: [Option<Dice>; 6] = Default::default();

    for (i, &image_data) in dice_images.iter().enumerate() {
        let image = image::load_from_memory(image_data)
            .expect(&format!("Failed to load embedded dice image {}", i + 1));

        // Resize the image
        let resized_image = image.resize_exact(
            target_width,
            target_height,
            imageops::FilterType::Lanczos3, // High-quality resizing filter
        );

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

    // Convert [Option<Dice>; 6] to [Dice; 6]
    core::array::from_fn(|i| dice_array[i].clone().expect("Missing dice image"))
}

fn main() {
    // --- Get Input File Path ---
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: drag and drop an image file onto the executable.");
        return;
    }
    let input_path = &args[1];

    // --- Load Input ---
    let input: GrayImage = load_image(input_path); // Assuming load_image handles potential resize
    let (iwidth, iheight) = input.dimensions();

    // --- Load Dice ---
    let dicks: [Dice; 6] = load_dice_images_static(); // load_dice_images now returns [Dice; 6]
    if dicks.is_empty() || dicks[0].image.width() == 0 || dicks[0].image.height() == 0 {
        eprintln!("Error loading dice or dice have invalid dimensions.");
        return;
    }
    let dwidth = dicks[0].image.width();
    let dheight = dicks[0].image.height();

    // --- Calculate Grid ---
    let num_dice_x = iwidth / dwidth;
    let num_dice_y = iheight / dheight;

    if num_dice_x == 0 || num_dice_y == 0 {
        eprintln!("Input image too small for dice dimensions.");
        return;
    }

    // --- Prepare Output Image ---
    let output_width = num_dice_x * dwidth;
    let output_height = num_dice_y * dheight;
    let mut output_image = RgbaImage::new(output_width, output_height);

    // --- Map Blocks and Construct Output ---
    for grid_y in 0..num_dice_y {
        for grid_x in 0..num_dice_x {
            let block_start_x = grid_x * dwidth;
            let block_start_y = grid_y * dheight;

            let block_view = imageops::crop_imm(&input, block_start_x, block_start_y, dwidth, dheight);

            let mut total_intensity: u64 = 0;
            let num_pixels_in_block = (dwidth * dheight) as u64;
            for pixel in block_view.to_image().pixels() {
                total_intensity += pixel[0] as u64;
            }
            let avg_intensity = if num_pixels_in_block > 0 {
                (total_intensity / num_pixels_in_block) as u8
            } else {
                0
            };

            let target_side: DiceSides = map_intensity_to_dice_side(avg_intensity);

            let dice_to_draw = dicks.iter().find(|&d| d.side == target_side);

            if let Some(dice) = dice_to_draw {
                let paste_x = grid_x * dwidth;
                let paste_y = grid_y * dheight;
                let dice_rgba = dice.image.to_rgba8();
                imageops::overlay(&mut output_image, &dice_rgba, paste_x as i64, paste_y as i64);
            } else {
                eprintln!(
                    "Warning: Could not find dice for side {:?} at grid ({}, {})",
                    target_side, grid_x, grid_y
                );
            }
        }
    }

    add_reference_text(
        &mut output_image,
        (dwidth, dheight),
        num_dice_x * num_dice_y,
        (output_width, output_height),
    );

    // --- Save Output ---
    let output_path = "output/dice_output.png";
    if let Some(parent_dir) = Path::new(output_path).parent() {
        std::fs::create_dir_all(parent_dir).expect("Failed to create output directory");
    }
    output_image.save(output_path).unwrap_or_else(|err| {
        eprintln!("Error saving output image: {}", err);
    });
    println!("Dice size used: {}x{}", dwidth, dheight);
    println!("Total dice used: {}", num_dice_x * num_dice_y);
    println!("Full image size: {}x{}", output_width, output_height);
    println!("Output saved to {}", output_path);
}