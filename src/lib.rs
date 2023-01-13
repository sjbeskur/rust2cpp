#![allow(dead_code)]

use image::io::Reader as ImageReader;

#[link(name = "cppoxide")]
extern {
        
    pub fn proof_of_life(i: libc::c_int, j: libc::c_int) -> libc::c_float;

    pub fn set_image_buffer( buffer: *const u8, size: libc::c_int) -> libc::c_int;

    //fn set_image( buffer: *const u8, size: libc::c_int) -> libc::c_int;

}


pub fn read_image(file: &String) -> Result<image::DynamicImage, image::ImageError> {
    let img_reader = ImageReader::open(file)?; //
    let img = img_reader.decode()?; 
    Ok(img)
}


// experimental
#[repr(C)]
pub struct ByteBuffer {
    len: i64,
    data: *mut u8,
}