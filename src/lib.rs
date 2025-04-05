use ab_glyph::{FontVec, PxScale};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;


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