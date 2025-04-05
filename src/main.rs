use core::num;
use std::iter;

use image::{buffer::Pixels, imageops::FilterType::Lanczos3, open, DynamicImage, GenericImage, GenericImageView, GrayImage, Pixel, Pixels, RgbImage};



enum DiceSides {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

struct PixelData {
    x: u32,
    y: u32,
    r: u8,
    g: u8,
    b: u8,
}


struct PixelMap {
    dice: DiceSides,
    x: u32,
    y: u32,
    pixel: Rgba<u8>
}
struct DiceMap {
    dice: DiceSides,
    x: u32,
    y: u32,
}

struct SetofDice {
    dice: Vec<DiceMap>, 
}

fn main() {
    // Load image
    let mut input: DynamicImage = load_image("images/falcons.jpg");
    let (iwidth, iheight) = input.to_luma8().dimensions();
    let dice: Vec<DynamicImage> = load_dice_images();
    let rgb_dice = dice.iter().map(|img| img.to_rgb8()).collect::<Vec<_>>();
    let (dwidth, dheight) = dice[0].to_luma8().dimensions();
    let num_dice_x = iwidth / dwidth;
    let num_dice_y = iheight / dheight;
    let dynamic_rgb_dice: Vec<DynamicImage> = rgb_dice.iter().map(|img| DynamicImage::ImageRgb8(img.clone())).collect();
    iterate_grid(num_dice_x, num_dice_y, dwidth, dheight, &dynamic_rgb_dice, &mut input);
    input.save("output/input.png").unwrap();

    // let output_width = num_dice_x * dwidth;
    // let output_height = num_dice_y * dheight;
    // let mut output: RgbImage = RgbImage::new(output_width, output_height);
    // Loop through the image and replace each square with a die
    

    
}


fn load_dice_images() -> Vec<DynamicImage> {
    //currently 500x500 dice
    let mut dice_images = Vec::new();
    for i in 1..=6 {
        let image_path = format!("dice/{}side.png", i);
        let image = open(image_path).unwrap().into_luma8();
        // Convert ImageBuffer to DynamicImage and push to the vector
        dice_images.push(DynamicImage::ImageLuma8(image));
    }
    dice_images
}

fn load_image(_input_image: &str) -> DynamicImage {
    // Load Image from file
    let input_image = open("images/falcons.jpg").unwrap().into_rgba8();
    // Convert to grayscale
    let grayscale_image = DynamicImage::ImageRgba8(input_image).into_luma8();
    let dynamic_image = DynamicImage::ImageLuma8(grayscale_image.clone());
    //getting the dimensions of the image
    let (width, height) = grayscale_image.dimensions();
    println!("Original Image size:\nWidth: {} Height: {}", width, height);
    // change image size to squared
    let resized = dynamic_image.resize(2500, 2000, image::imageops::FilterType::Lanczos3);
    resized
}


fn pixel_data_iter(img: DynamicImage) -> Vec<PixelData> {
    let ipxls = img.pixels();
    let mut opxls: Vec<PixelData> = vec![];
    for p in ipxls {
        let rgb = p.2;
        let pd: PixelData = PixelData {
            x: p.0,
            y: p.1,
            r: rgb[0],
            g: rgb[1],
            b: rgb[2]
        };
        opxls.push(pd);
    }
    opxls
}