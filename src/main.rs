mod cli;
use image::GenericImageView;
use rustcpp::*;

fn main() {
    let config = cli::parse_args();
    run(config);
}

fn run(config: cli::Config){
    let file = config.file;
    let img = rustcpp::read_image(&file).expect(&format!("Failed to open open file {}", file));
    let dims = img.dimensions();
    let width = dims.0;
    let height = dims.1;    
    

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();    

    //let bytes = bytes.as_slice();
    let rslt = unsafe { set_image_buffer(bytes.as_ptr(), bytes.len() as i32, width, height) };

    println!("result = {}", rslt);
}

