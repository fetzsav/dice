use image::{
    imageops, open, DynamicImage, GenericImage, GenericImageView, GrayImage, ImageReader, Luma, Pixel, RgbaImage // Use RGBA for output flexibility
};
use std::path::Path; // For checking paths, creating dirs
use image::{Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use ab_glyph::{FontVec, PxScale, ScaleFont, Font};

mod lib;


// Simplified DiceSides to not hold the image directly
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // Added derives for mapping
enum DiceSides {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, Clone)] // Added Clone
struct Dice {
    side: DiceSides, // Uses the simplified enum
    image: DynamicImage,
}

// Dices struct might not be needed if directly using the array
// #[derive(Debug)]
// struct Dices {
//     dices: [Dice; 6]
// }

// Other structs like PixelData, PixelMap, DiceMap, SetofDice removed for simplicity

fn main() {
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


/// Loads dice images 1side.png to 6side.png from "dice/" folder.
/// Panics on load failure or inconsistent dimensions.
fn load_dice_images() -> [Dice; 6] {
    // Target dimensions
    let target_width: u32 = 20;
    let target_height: u32 = 20;

    let mut dice_array: [Option<Dice>; 6] = Default::default();

    for i in 0..6 {
        let side_num = i + 1;
        let image_path = format!("dice/{}side.png", side_num);
        let image = open(&image_path).unwrap_or_else(|e| {
            panic!("Failed to load dice image {}: {}", image_path, e);
        });

         // Check if original image has valid dimensions before resizing
         if image.width() == 0 || image.height() == 0 {
             panic!("Original dice image {} has zero dimensions before resizing.", image_path);
         }


        // Resize the image
        let mut resized_image = image.resize_exact(
             target_width,
             target_height,
             imageops::FilterType::Lanczos3 // A good quality resizing filter
        );

        //INVERT        
        resized_image.invert();


        // Assign simplified DiceSides enum variant
        let side = match side_num {
            1 => DiceSides::One,
            2 => DiceSides::Two,
            3 => DiceSides::Three,
            4 => DiceSides::Four,
            5 => DiceSides::Five,
            6 => DiceSides::Six,
            _ => unreachable!(),
        };

        // Store the resized image in Option array
        dice_array[i] = Some(Dice { side, image: resized_image });
    }

    // Convert [Option<Dice>; 6] to [Dice; 6]
    core::array::from_fn(|i| {
        dice_array[i]
            .clone()
            .expect("Internal error: Dice image option was None after processing")
    })
}


// / Maps average grayscale intensity (0-255) to a DiceSides variant.
fn map_intensity_to_dice_side(avg_intensity: u8) -> DiceSides {
    match avg_intensity {
         0..=42 => DiceSides::One,
         43..=85 => DiceSides::Two,
         86..=128 => DiceSides::Three,
        129..=171 => DiceSides::Four,
        172..=214 => DiceSides::Five,
        215..=255 => DiceSides::Six,
    }
}




/// add some reference text (dice size, total dice, image size) onto the image
fn add_reference_text(
    image: &mut RgbaImage,
    dice_size: (u32, u32),
    total_dice: u32,
    full_image_size: (u32, u32),
) {
    // load a font
    let font_data = std::fs::read("/usr/share/fonts/TTF/DejaVuSans-Bold.ttf")
        .expect("Failed to load font file");
    let font = FontVec::try_from_vec(font_data.to_vec()).expect("font loading failed");

    // build the overlay text
    let text = format!(
        "dice size: {}x{}\ntotal dice: {}\nimage size: {}x{}",
        dice_size.0, dice_size.1, total_dice, full_image_size.0, full_image_size.1
    );

    // text style
    let scale = PxScale::from(40.0);
    let font = font; // Use FontVec directly without scaling
    let text_color = Rgba([255, 255, 255, 255]); // white text
    let background_color = Rgba([0, 0, 0, 200]); // semi-transparent black

    // estimate text box dimensions
    let text_width = 200;
    let text_height = 60;

    // draw a black rectangle behind the text
    let rect = Rect::at(5, 5) // Top-left corner
        .of_size(text_width, text_height);
    draw_filled_rect_mut(image, rect, background_color);

    // draw the text onto the image
    draw_text_mut(
        image,
        text_color,
        10, // x offset
        10, // y offset
        scale,
        &font,
        &text,
    );
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
