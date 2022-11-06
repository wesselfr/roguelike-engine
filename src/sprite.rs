use image::{DynamicImage, GenericImageView};

pub(crate) struct Sprite {
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub image: DynamicImage,
    pub frame_num: u32,
}

impl Sprite {
    pub(crate) fn from_image(path: &str, scale: Option<f32>) -> Self {
        let image = image::open(path).unwrap();
        let (width, height) = image.dimensions();
        Self {
            width,
            height,
            scale: scale.unwrap_or(4.0),
            image,
            frame_num: 1,
        }
    }
    pub(crate) fn from_image_animated(path: &str, frame_count: u32, scale: Option<f32>) -> Self {
        let image = image::open(path).unwrap();
        let (width, height) = image.dimensions();
        Self {
            width,
            height,
            scale: scale.unwrap_or(4.0),
            image,
            frame_num: frame_count,
        }
    }
}
