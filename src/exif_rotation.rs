use std::fs::File;
use std::io;
use exif::{Error, Exif, In, Tag};
use image::{ImageBuffer, Rgba};
use image::imageops::{flip_horizontal, flip_vertical, rotate180, rotate270, rotate90};
use crate::image_buffer_conversions::read_image_to_buffer;

// vertical flips could also be user for number 4, but this keeps the logic simple, rotate if needed, then flip if needed
#[repr(u32)]
#[derive(PartialEq, Debug)]
pub enum ExifRotation {
    Upright = 1,
    FlippedHorizontal = 2,
    Rotated180 = 3,
    FlippedVertical = 4,
    Rotated90CWFlippedHorizontal = 5,
    Rotated90CCW = 6,
    Rotated90CCWFlippedHorizontal = 7,
    Rotated90CW = 8,
}

impl TryFrom<u32> for ExifRotation {
    type Error = &'static str;  // Simpler error type

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (1..=8).contains(&value) {
            Ok(unsafe { std::mem::transmute::<u32, ExifRotation>(value) })
        } else {
            Err("only values 1 to 8 are supported")
        }
    }
}

impl ExifRotation {
    pub fn apply(&self, image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        match self {
            ExifRotation::Upright => {image_buffer}
            ExifRotation::FlippedHorizontal => {flip_horizontal(&image_buffer)}
            ExifRotation::Rotated180 => {rotate180(&image_buffer)}
            ExifRotation::FlippedVertical => {flip_vertical(&image_buffer)}
            ExifRotation::Rotated90CWFlippedHorizontal => {rotate270(&flip_horizontal(&image_buffer))}
            ExifRotation::Rotated90CCW => {rotate90(&image_buffer)}
            ExifRotation::Rotated90CCWFlippedHorizontal => {rotate90(&flip_horizontal(&image_buffer))}
            ExifRotation::Rotated90CW => {rotate270(&image_buffer)}
        }
    }

    pub fn read_rotation_from_exif(exif_data: Exif) -> ExifRotation {
        if let Some(orientation) = exif_data.get_field(Tag::Orientation, In::PRIMARY) { 
            if let Some(v @ 1..=8) = orientation.value.get_uint(0) {
                let exif_rotation_result = ExifRotation::try_from(v);
                if let Ok(exif_rotation) = exif_rotation_result { exif_rotation } else {
                    eprintln!("Invalid exif rotation value, assuming correct orientation");
                    ExifRotation::Upright
                }
            } else {
                eprintln!("Orientation value is broken, assuming correct orientation");
                ExifRotation::Upright
            } 
        } else {
            eprintln!("reading orientation tag failed, assuming correct orientation");
            ExifRotation::Upright
        }
    }
}

pub fn fix_rotation(filename: &str) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let exif_result = read_exif_data_from_file(filename);

    match exif_result {
        Ok(exif_data) => {
            let rotation = ExifRotation::read_rotation_from_exif(exif_data);
            if rotation == ExifRotation::Upright {
                return None;
            }

            let image_buffer_option = read_image_to_buffer(filename);
            image_buffer_option.map(|image_buffer| rotation.apply(image_buffer))
        },
        Err(err) => {
            eprintln!("Reading exif data failed: {err} {filename}");
            None
        }
    }
}

fn read_exif_data_from_file(filename: &str) -> Result<Exif, Error> {
    let file = File::open(filename)?;
    let mut bufreader = io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    exif_reader.read_from_container(&mut bufreader)
}
