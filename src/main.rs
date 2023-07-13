use image::imageops::FilterType;
use image::{load_from_memory, DynamicImage, GenericImageView};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new("image.png")
    };

    let image_file = File::open(image_path).expect("Failed to open image file");
    let mut buf_reader = BufReader::new(image_file);
    let mut image_data = Vec::new();
    buf_reader
        .read_to_end(&mut image_data)
        .expect("Failed to read image");

    let image = load_from_memory(&image_data).expect("Failed to decode image");
    let (width, height) = image.dimensions();

    println!("Image dimensions: {}x{}", width, height);

    let image = if args.len() > 2 {
        let size = args[2].parse::<u32>().unwrap();
        image.resize(size, size, FilterType::Nearest)
    } else {
        image.resize(64, 64, FilterType::Nearest)
    };

    let ascii_art = convert_to_ascii(&image);
    println!("{}", ascii_art);
}

fn convert_to_ascii(image: &DynamicImage) -> String {
    let (width, height) = image.dimensions();
    let mut ascii_art = String::new(); // ""

    for y in 0..height {
        // if odd skip row, to keep aspect ratio
        if y % 2 == 1 {
            continue;
        }

        // println!("Processing row {}", y);

        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let luminance = (
                  0.2126 * pixel[0] as f32
                + 0.7152 * pixel[1] as f32
                + 0.0722 * pixel[2] as f32
            ) as u8;
            let ascii_char = if pixel[3] == 0 {
                ' '
            } else {
                match luminance { // 0 - 256
                    0..=15 => ' ',
                    16..=31 => '.',
                    32..=47 => ':',
                    48..=63 => '-',
                    64..=79 => '=',
                    80..=95 => '+',
                    96..=111 => '*',
                    112..=127 => '#',
                    128..=143 => '%',
                    144..=159 => '@',
                    160..=175 => 'a',
                    176..=191 => 'o',
                    192..=207 => 'e',
                    208..=223 => 'h',
                    224..=239 => 'u',
                    _ => '$',
                }
            };
            ascii_art.push(ascii_char);
        }
        ascii_art.push('\n');
    }

    ascii_art
}
