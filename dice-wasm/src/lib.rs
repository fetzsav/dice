use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

use ab_glyph::{FontVec, PxScale};
use image::{imageops, DynamicImage, GrayImage, ImageBuffer, Rgba, RgbaImage};
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use std::io::Cursor;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum IntensityPreset {
    Default,
    HighContrast,
    LowContrast,
    Bright,
    Dark,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiceSides {
    One, Two, Three, Four, Five, Six,
}

#[derive(Clone)]
struct Dice {
    side: DiceSides,
    image: DynamicImage,
}

fn map_intensity_to_dice_side(avg: u8, preset: IntensityPreset) -> DiceSides {
    match preset {
        IntensityPreset::Default => match avg {
            0..=50 => DiceSides::One,
            51..=100 => DiceSides::Two,
            101..=150 => DiceSides::Three,
            151..=200 => DiceSides::Four,
            201..=230 => DiceSides::Five,
            _ => DiceSides::Six,
        },
        IntensityPreset::HighContrast => match avg {
            0..=42 => DiceSides::One,
            43..=85 => DiceSides::Two,
            86..=128 => DiceSides::Three,
            129..=171 => DiceSides::Four,
            172..=214 => DiceSides::Five,
            _ => DiceSides::Six,
        },
        IntensityPreset::LowContrast => match avg {
            0..=60 => DiceSides::One,
            61..=120 => DiceSides::Two,
            121..=180 => DiceSides::Three,
            181..=210 => DiceSides::Four,
            211..=240 => DiceSides::Five,
            _ => DiceSides::Six,
        },
        IntensityPreset::Bright => match avg {
            0..=30 => DiceSides::One,
            31..=80 => DiceSides::Two,
            81..=130 => DiceSides::Three,
            131..=180 => DiceSides::Four,
            181..=220 => DiceSides::Five,
            _ => DiceSides::Six,
        },
        IntensityPreset::Dark => match avg {
            0..=70 => DiceSides::One,
            71..=120 => DiceSides::Two,
            121..=160 => DiceSides::Three,
            161..=200 => DiceSides::Four,
            201..=240 => DiceSides::Five,
            _ => DiceSides::Six,
        },
    }
}

fn add_reference_text(
    image: &mut RgbaImage,
    dice_size: (u32, u32),
    total_dice: u32,
    full_image_size: (u32, u32),
) {
    let font_data = include_bytes!("../DejaVuSans-Bold.ttf");
    let font = FontVec::try_from_vec(font_data.to_vec()).expect("Failed to load font");
    let scale = PxScale::from(20.0);

    let text = format!(
        "Dice size: {}x{}, Total dice: {}, Image size: {}x{}",
        dice_size.0, dice_size.1, total_dice, full_image_size.0, full_image_size.1
    );

    // crude width estimate
    let text_width = (text.len() as u32).saturating_mul(12);
    let text_height = 28;

    let rect = Rect::at(0, 0).of_size(text_width.max(full_image_size.0), text_height);
    draw_filled_rect_mut(image, rect, Rgba([0, 0, 0, 200]));
    draw_text_mut(image, Rgba([255, 255, 255, 255]), 6, 4, scale, &font, &text);
}

fn load_from_bytes_gray(bytes: &[u8]) -> Result<GrayImage, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    Ok(img.into_luma8())
}
fn load_from_bytes_rgba(bytes: &[u8]) -> Result<DynamicImage, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    Ok(img)
}

#[wasm_bindgen]
pub struct DiceOptions {
    pub dice_size: u32,
    pub invert_input: bool,
    pub invert_dice: bool,
    pub preset: IntensityPreset,
    pub output_width: Option<u32>,
    pub output_height: Option<u32>,
    pub add_debug: bool,
}

#[wasm_bindgen]
impl DiceOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        dice_size: u32,
        invert_input: bool,
        invert_dice: bool,
        preset: IntensityPreset,
        output_width: Option<u32>,
        output_height: Option<u32>,
        add_debug: bool,
    ) -> DiceOptions {
        DiceOptions {
            dice_size: if dice_size == 0 { 32 } else { dice_size },
            invert_input,
            invert_dice,
            preset,
            output_width,
            output_height,
            add_debug,
        }
    }
}

/// input_bytes: original image (PNG/JPEG/etc)
/// dice_pngs:   JS Array of 6 Uint8Array dice images (faces 1..6, your order)
#[wasm_bindgen]
pub fn process_dice_image(
    input_bytes: Uint8Array,
    dice_pngs: js_sys::Array,
    opts: DiceOptions,
) -> Result<Uint8Array, JsValue> {
    console_error_panic_hook::set_once();

    // 1) Input
    let mut input_vec = vec![0u8; input_bytes.length() as usize];
    input_bytes.copy_to(&mut input_vec[..]);
    let mut input = load_from_bytes_gray(&input_vec)
        .map_err(|e| JsValue::from(js_sys::Error::new(&e)))?;
    if opts.invert_input {
        imageops::invert(&mut input);
    }

    // 2) Load 6 dice images
    if dice_pngs.length() != 6 {
        return Err(js_sys::Error::new("dice_pngs must contain exactly 6 images").into());
    }
    let mut dice_vec: Vec<Dice> = Vec::with_capacity(6);
    for (i, val) in dice_pngs.iter().enumerate() {
        let u8arr = js_sys::Uint8Array::new(&val);
        let mut buf = vec![0u8; u8arr.length() as usize];
        u8arr.copy_to(&mut buf[..]);

        let mut img = load_from_bytes_rgba(&buf)
            .map_err(|e| JsValue::from(js_sys::Error::new(&e)))?;
        if opts.invert_dice { img.invert(); }

        let resized = img.resize_exact(opts.dice_size, opts.dice_size, imageops::FilterType::Lanczos3);
        let side = match i {
            0 => DiceSides::One,
            1 => DiceSides::Two,
            2 => DiceSides::Three,
            3 => DiceSides::Four,
            4 => DiceSides::Five,
            5 => DiceSides::Six,
            _ => unreachable!(),
        };
        dice_vec.push(Dice { side, image: resized });
    }

    // 3) Optional output canvas sizing (center-fit)
    let (out_w, out_h) = match (opts.output_width, opts.output_height) {
        (Some(w), Some(h)) if w > 0 && h > 0 => (w, h),
        _ => input.dimensions(),
    };
    let (in_w, in_h) = input.dimensions();
    if (out_w, out_h) != (in_w, in_h) {
        let aspect = in_w as f32 / in_h as f32;
        let (nw, nh) = if (out_w as f32 / out_h as f32) > aspect {
            (((out_h as f32) * aspect).round() as u32, out_h)
        } else {
            (out_w, ((out_w as f32) / aspect).round() as u32)
        };
        let mut canvas = GrayImage::new(out_w, out_h);
        let scaled = imageops::resize(&input, nw, nh, imageops::FilterType::Lanczos3);
        let off_x = ((out_w - nw) / 2) as i64;
        let off_y = ((out_h - nh) / 2) as i64;
        imageops::overlay(&mut canvas, &scaled, off_x, off_y);
        input = canvas;
    }

    // 4) Crop square
    let (w, h) = input.dimensions();
    let sq = w.min(h);
    let input_sq = imageops::crop_imm(&input, 0, 0, sq, sq).to_image();

    // 5) Grid → dice render
    let dw = opts.dice_size;
    let dh = opts.dice_size;
    let num_x = sq / dw;
    let num_y = sq / dh;

    let mut out: RgbaImage = ImageBuffer::new(num_x * dw, num_y * dh);

    for gy in 0..num_y {
        for gx in 0..num_x {
            let sx = gx * dw;
            let sy = gy * dh;
            let block = imageops::crop_imm(&input_sq, sx, sy, dw, dh).to_image();

            let mut total: u64 = 0;
            let count = (dw * dh) as u64;
            for p in block.pixels() { total += p[0] as u64; }
            let avg = if count > 0 { (total / count) as u8 } else { 0 };

            let target = map_intensity_to_dice_side(avg, opts.preset);
            let dice = dice_vec.iter().find(|d| d.side == target).expect("missing dice face");
            let dice_rgba = dice.image.to_rgba8();
            imageops::overlay(&mut out, &dice_rgba, sx as i64, sy as i64);
        }
    }

    if opts.add_debug {
        let total_dice = num_x * num_y;
        // Avoid simultaneous mutable & immutable borrows:
        let dims = out.dimensions();
        add_reference_text(&mut out, (dw, dh), total_dice, dims);
    }

    // 6) Encode RGBA to PNG
    let width = out.width();
    let height = out.height();
    let mut bytes = Vec::<u8>::new();
    let mut cur = Cursor::new(&mut bytes);
    let enc = PngEncoder::new(&mut cur);
    enc.write_image(
        out.as_raw(),
        width,
        height,
        ColorType::Rgba8.into(), // -> ExtendedColorType
    ).map_err(|e| js_sys::Error::new(&e.to_string()))?;

    Ok(Uint8Array::from(bytes.as_slice()))
}
