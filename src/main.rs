use image::{load_from_memory, DynamicImage, GenericImageView};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() {
    // Carrega a imagem
    let image_path = Path::new("image.png");
    let image_file = File::open(image_path).expect("Falha ao abrir o arquivo de imagem");
    let mut buf_reader = BufReader::new(image_file);
    let mut image_data = Vec::new();
    buf_reader
        .read_to_end(&mut image_data)
        .expect("Falha ao ler a imagem");

    // Decodifica a imagem
    let image = load_from_memory(&image_data).expect("Falha ao decodificar a imagem");

    // Exibe as dimensões da imagem
    let (width, height) = image.dimensions();
    println!("Dimensões da imagem: {}x{}", width, height);

    // Redimensiona para 64px
    let image = image.resize(64, 64, image::imageops::FilterType::Nearest);

    // Converte a imagem em arte ASCII
    let ascii_art = convert_to_ascii(&image);
    println!("{}", ascii_art);
}

fn convert_to_ascii(image: &DynamicImage) -> String {
    let (width, height) = image.dimensions();
    let mut ascii_art = String::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let luminance = (0.2126 * pixel[0] as f32
                + 0.7152 * pixel[1] as f32
                + 0.0722 * pixel[2] as f32) as u8;
            let ascii_char = if pixel[3] == 0 {
                ' '
            } else {
                match luminance {
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
