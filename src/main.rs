use image::{
    imageops, GrayImage, RgbaImage // Only necessary imports for image processing
};
use std::path::Path; // For checking paths, creating directories

use dice::*;

// Dices struct might not be needed if directly using the array
// #[derive(Debug)]
// struct Dices {
//     dices: [Dice; 6]
// }

// Other structs like PixelData, PixelMap, DiceMap, SetofDice removed for simplicity

fn main() {
    // getting read to put clap logic here for CLI






    // --- Load Input ---
    let input: GrayImage = load_image("images/pringles.png"); // Assuming load_image handles potential resize
    let (iwidth, iheight) = input.dimensions();

    // --- Load Dice ---
    let dicks: [Dice; 6] = load_dice_images(); // load_dice_images now returns [Dice; 6]
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
    // Create RGBA output image
    let mut output_image = RgbaImage::new(output_width, output_height);


    // --- Map Blocks and Construct Output ---
    for grid_y in 0..num_dice_y {
        for grid_x in 0..num_dice_x {
            let block_start_x = grid_x * dwidth;
            let block_start_y = grid_y * dheight;

            // Get view of current block
            let block_view = imageops::crop_imm(
                &input,
                block_start_x,
                block_start_y,
                dwidth,
                dheight,
            );

            // Calculate average intensity
            let mut total_intensity: u64 = 0;
            let num_pixels_in_block = (dwidth * dheight) as u64;
            for pixel in block_view.to_image().pixels() {
                total_intensity += pixel[0] as u64; // Luma<u8> pixel value
            }
            let avg_intensity = if num_pixels_in_block > 0 {
                (total_intensity / num_pixels_in_block) as u8
            } else {
                0
            };

            // Map intensity to dice side
            let target_side: DiceSides = map_intensity_to_dice_side(avg_intensity);

            // Find the matching dice image
            // Iterate through the 'dicks' array to find the Dice with the matching 'side'
            let dice_to_draw = dicks.iter().find(|&d| d.side == target_side);


            // Paste the dice image onto the output
            if let Some(dice) = dice_to_draw {
                let paste_x = grid_x * dwidth;
                let paste_y = grid_y * dheight;
                // Ensure dice image is RGBA for overlay
                let dice_rgba = dice.image.to_rgba8();
                 imageops::overlay(&mut output_image, &dice_rgba, paste_x as i64, paste_y as i64);
            } else {
                 // Should not happen if map_intensity_to_dice_side and dicks array are correct
                 eprintln!("Warning: Could not find dice for side {:?} at grid ({}, {})", target_side, grid_x, grid_y);
            }

        }
    }

    //adding debug text
    add_reference_text(
        &mut output_image,
        (dwidth, dheight),
        num_dice_x * num_dice_y,
        (output_width, output_height),
    );

    // --- Save Output ---
    let output_path = "output/dice_output.png";
    // Ensure output directory exists
     if let Some(parent_dir) = Path::new(output_path).parent() {
         std::fs::create_dir_all(parent_dir)
             .expect("Failed to create output directory");
     }
    output_image.save(output_path).unwrap_or_else(|err| {
        eprintln!("Error saving output image: {}", err);
    });
    println!("Dice size used: {}x{}", dwidth, dheight);
    println!("Total dice used: {}", num_dice_x * num_dice_y);
    println!("Full image size: {}x{}", output_width, output_height);
    println!("Output saved to {}", output_path);
}






// // / Maps average grayscale intensity (0-255) to a DiceSides variant.
// fn map_intensity_to_dice_side(avg_intensity: u8) -> DiceSides {
//     match avg_intensity {
//          0..=50 => DiceSides::One,
//          51..=100 => DiceSides::Two,
//          101..=150 => DiceSides::Three,
//          151..=200 => DiceSides::Four,
//          201..=230 => DiceSides::Five,
//          231..=255 => DiceSides::Six,
//     }
// }


// Removed grayscale_intensity_vec and pixel_data_iter as block processing is done directly in main
