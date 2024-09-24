use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

pub trait ImageBufferConversions {
    fn to_image_buffer(&self) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>>;
}

impl ImageBufferConversions for DynamicImage {
    fn to_image_buffer(&self) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let (w,h) = self.dimensions();
        let rgba_image = self.to_rgba8();
        let image_bytes = rgba_image.as_raw();
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(w, h, image_bytes.clone()) // TODO: why is clone required here?
    }
}

pub fn read_image_to_buffer(filename: &str) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let dyn_img = image::open(filename);

    match dyn_img {
        Ok(img) => img.to_image_buffer(),
        _ => None
    }
}
