use core::num;
use std::iter;

use image::{imageops::FilterType::Lanczos3, open, DynamicImage, GrayImage, RgbImage, GenericImage};

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


fn iterate_grid(num_dice_x: u32, num_dice_y: u32, dwidth: u32, dheight: u32, dice: &Vec<DynamicImage>, input: &DynamicImage) {
    let mut output = input.clone();
    for i in 0..num_dice_x {
        for j in 0..num_dice_y {
            let x = i * dwidth;
            let y = j * dheight;
            let die = dice[((i + j) % 6) as usize].to_rgb8();
            let die_rgba = DynamicImage::ImageRgb8(die).into_rgba8();
            output.copy_from(&die_rgba, x, y).unwrap();
        }
    }
}