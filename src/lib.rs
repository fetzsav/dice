use ab_glyph::{FontVec, PxScale};
use image::{imageops, open, DynamicImage, GrayImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // Added derives for mapping
pub enum DiceSides {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, Clone)] // Added Clone
pub struct Dice {
    pub side: DiceSides, // Uses the simplified enum
    pub image: DynamicImage,
}

pub fn add_reference_text(
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


/// Loads dice images 1side.png to 6side.png from "dice/" folder.
/// Panics on load failure or inconsistent dimensions.
pub fn load_dice_images() -> [Dice; 6] {
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



/// Loads and returns a GrayImage. Input path is hardcoded for now.
pub fn load_image(_input_path: &str) -> GrayImage {
    // Path is currently hardcoded inside, consider passing _input_path through

   // let img = ImageReader::open("images/flag.jpeg").unwrap()
   //     .decode()
   //     .expect("Failed to decode image")
   //     .into_luma8();

   let img = open("images/falcons.jpg") // Use _input_path here if needed
       .expect("Failed to load input image")
       .into_luma8();

   // // conditional resize here later
   let dynamic_image = DynamicImage::ImageLuma8(img);
   let resized = dynamic_image.resize(1800,1800, image::imageops::FilterType::Lanczos3).into_luma8();
   return resized;

   img // Return original grayscale image if no resize
}


/// Loads dice images 1side.png to 6side.png from "dice/" folder.
/// Panics on load failure or inconsistent dimensions.
pub fn dice_images() -> [Dice; 6] {
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
pub fn map_intensity_to_dice_side(avg_intensity: u8) -> DiceSides {
    match avg_intensity {
         0..=42 => DiceSides::One,
         43..=85 => DiceSides::Two,
         86..=128 => DiceSides::Three,
        129..=171 => DiceSides::Four,
        172..=214 => DiceSides::Five,
        215..=255 => DiceSides::Six,
    }
}