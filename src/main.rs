use core::num;
use std::iter;

use image::{buffer::Pixels, imageops::FilterType::Lanczos3, open, DynamicImage, GenericImage, GenericImageView, GrayImage, Pixel,RgbImage, Rgba};


#[derive(Debug)]
enum DiceSides {
    One(DynamicImage),
    Two(DynamicImage),
    Three(DynamicImage),
    Four(DynamicImage),
    Five(DynamicImage),
    Six(DynamicImage),
}

#[derive(Debug)]
struct Dice {
    side: DiceSides,
    image: DynamicImage,
}

#[derive(Debug)]
struct Dices {
    dices: [Dice; 6]
}

#[derive(Debug)]
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

// fn assign 

fn main() {
    // Load INPUT image, convert to luma8, and get dimensions.
    let mut input: GrayImage = load_image("images/falcons.jpg");
    let (iwidth, iheight) = input.dimensions();
    let pixel_data = pixel_data_iter(input.clone());


    //load array of our dice. vec not needed here ofc
    let dicks:[Dice; 6] = load_dice_images();
    let dwidth = dicks[0].image.width();
    let dheight = dicks[0].image.height();



// GOING TO NEED THIS LATER PROBABLY
    // let num_dice_x = iwidth / dwidth;
    // let num_dice_y = iheight / dheight;
//--end GOING TO NEED THIS LATER PROBABLY

//  \/ one liner for pixel_data_iter() \/ \/ \/ \/ \/
    // let dynamic_rgb_dice: Vec<  = rgb_dice.iter().map(|img| DynamicImage::ImageRgb8(img.clone())).collect();
    // input.save("output/input.png").unwrap();


    // for (i, dice) in dicks.iter().enumerate() {
    //     let name: String = i.to_string()+".png";
    //     input.save(name).unwrap_or_else(|err| println!("Error saving: {}", err));
    // }

    // let output_width = num_dice_x * dwidth;
    // let output_height = num_dice_y * dheight;
    // let mut output: RgbImage = RgbImage::new(output_width, output_height);
    // Loop through the image and replace each square with a die
    

    
}


fn load_dice_images() -> [Dice; 6] {
    // currently 500x500 dice

    //super fancy core library... gotta remember this one.
    let mut dices: [Dice; 6] = core::array::from_fn(|i: usize| {
        let side: DiceSides = match i {
            0 => DiceSides::One(DynamicImage::new_rgb8(500, 500)),
            1 => DiceSides::Two(DynamicImage::new_rgb8(500, 500)),
            2 => DiceSides::Three(DynamicImage::new_rgb8(500, 500)),
            3 => DiceSides::Four(DynamicImage::new_rgb8(500, 500)),
            4 => DiceSides::Five(DynamicImage::new_rgb8(500, 500)),
            5 => DiceSides::Six(DynamicImage::new_rgb8(500, 500)),
            _ => unreachable!(),
        };
        let image: DynamicImage = DynamicImage::new_rgb8(500, 500);
        Dice { side, image }
    });
    for i in 1..=6 {
        let image_path = format!("dice/{}side.png", i);
        let image = open(image_path).unwrap();
        
        let ds: Dice = Dice {
            side: match i {
                0 => DiceSides::One(image.clone()),
                1 => DiceSides::Two(image.clone()),
                2 => DiceSides::Three(image.clone()),
                3 => DiceSides::Four(image.clone()),
                4 => DiceSides::Five(image.clone()),
                5 => DiceSides::Six(image.clone()),
                _ => panic!("Invalid dice side"),
            },
            image: image.clone()
        };
        dices[i - 1] = ds;
        
    }
    dices
    
    //try to manually deconstruct the 
    // let dray: [DynamicImage; 6] = [
    //     dice_images.remove(0),
    //     dice_images.remove(0),
    //     dice_images.remove(0),
    //     dice_images.remove(0),
    //     dice_images.remove(0),
    //     dice_images.remove(0),
    // ];
    // 
}

fn load_image(_input_image: &str) -> GrayImage {
    // Load Image from file
    let input_image: GrayImage = open("images/falcons.jpg").unwrap().into_luma8();
    // Image is grayscaled here
    
    let dynamic_image= DynamicImage::ImageLuma8(input_image);
    //resized here, probs moving later ofc
    let resized = dynamic_image.resize(2500, 2000, image::imageops::FilterType::Lanczos3).into_luma8();
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







