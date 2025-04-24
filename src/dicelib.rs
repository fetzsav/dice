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
    let font_data = include_bytes!("../DejaVuSans-Bold.ttf");
    let font = FontVec::try_from_vec(font_data.to_vec()).expect("Failed to load font");
    let scale = PxScale::from(20.0); // Font size

    let text = format!(
        "Dice size: {}x{}, Total dice: {}, Image size: {}x{}",
        dice_size.0, dice_size.1, total_dice, full_image_size.0, full_image_size.1
    );

    // Calculate text dimensions
    let text_width = text.len() as u32 * 12; // Approximate width per character
    let text_height = 24; // Approximate height of the text

    // Draw black background rectangle
    let rect = Rect::at(0, 0).of_size(text_width, text_height);
    draw_filled_rect_mut(image, rect, Rgba([0, 0, 0, 255])); // Black background

    // Draw the text
    draw_text_mut(
        image,
        Rgba([255, 255, 255, 255]), // White text
        5,                          // X offset
        5,                          // Y offset
        scale,
        &font,
        &text,
    );
}


/// Loads and returns a GrayImage
pub fn load_image(input_path: &str) -> GrayImage {
    // Path is currently hardcoded inside, consider passing _input_path through

   // let img = ImageReader::open("images/flag.jpeg").unwrap()
   //     .decode()
   //     .expect("Failed to decode image")
   //     .into_luma8();

   let img = open(input_path) // Use _input_path here if needed
       .expect("Failed to load input image")
       .into_luma8();

   // // conditional resize here later
//    let dynamic_image = DynamicImage::ImageLuma8(img);
//    let resized = dynamic_image.resize(2048,2048, image::imageops::FilterType::Lanczos3).into_luma8();
//    return resized;

   img // Return original grayscale image if no resize
}


pub enum IntensityPreset {
    Default,
    HighContrast,
    LowContrast,
    Bright,
    Dark
}

pub fn map_intensity_to_dice_side(avg_intensity: u8, preset: &IntensityPreset) -> DiceSides {
    match preset {
        IntensityPreset::Default => match avg_intensity {
            0..=50 => DiceSides::One,
            51..=100 => DiceSides::Two,
            101..=150 => DiceSides::Three,
            151..=200 => DiceSides::Four,
            201..=230 => DiceSides::Five,
            231..=255 => DiceSides::Six,
        },
        IntensityPreset::HighContrast => match avg_intensity {
            0..=42 => DiceSides::One,
            43..=85 => DiceSides::Two,
            86..=128 => DiceSides::Three,
            129..=171 => DiceSides::Four,
            172..=214 => DiceSides::Five,
            215..=255 => DiceSides::Six,
        },
        IntensityPreset::LowContrast => match avg_intensity {
            0..=60 => DiceSides::One,
            61..=120 => DiceSides::Two,
            121..=180 => DiceSides::Three,
            181..=210 => DiceSides::Four,
            211..=240 => DiceSides::Five,
            241..=255 => DiceSides::Six,
        },
        IntensityPreset::Bright => match avg_intensity {
            0..=30 => DiceSides::One,
            31..=80 => DiceSides::Two,
            81..=130 => DiceSides::Three,
            131..=180 => DiceSides::Four,
            181..=220 => DiceSides::Five,
            221..=255 => DiceSides::Six,
        },
        IntensityPreset::Dark => match avg_intensity {
            0..=70 => DiceSides::One,
            71..=120 => DiceSides::Two,
            121..=160 => DiceSides::Three,
            161..=200 => DiceSides::Four,
            201..=240 => DiceSides::Five,
            241..=255 => DiceSides::Six,
        }
    }
}