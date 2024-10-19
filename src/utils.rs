use sdl2::image;
use sdl2::surface::Surface;
use std::env;
use std::fs::File;
use std::io::Read;

pub fn read_bin_file(file_path: &str) -> Vec<u8> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn print_hex(data: &[u8]) {
    for (i, byte) in data.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    println!();
}

pub fn get_file_path() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: String;
    // Check if a file path was provided
    if args.len() < 2 {
        // first arg is current executable file path
        #[cfg(not(debug_assertions))]
        {
            eprintln!("Usage: {} <file_path>", args[0]);
            panic!("No file path provided");
        }
        #[cfg(debug_assertions)]
        {
            file_path = String::from("roms/space_invaders/invaders");
        }
    } else {
        file_path = args[1].clone();
    }
    return file_path;
}

pub fn load_icon() -> Surface<'static> {
    // Initialize SDL2_image
    let _image_context = image::init(image::InitFlag::JPG).unwrap();

    // Load the icon image
    let icon_path = "src/assets/invaders_mascot.bmp";
    let icon = match Surface::load_bmp(icon_path) {
        Ok(icon) => icon,
        Err(error) => panic!("Problem loading icon: {:?}", error),
    };
    icon
}
