use image::{DynamicImage, GenericImage, GenericImageView};

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

    pub(crate) fn flip_vertical(&mut self) {
        self.image = self.image.flipv();
    }

    pub(crate) fn flip_horizontal(&mut self) {
        self.image = self.image.fliph();
    }

    pub(crate) fn from_grid(
        path: &str,
        x: u32,
        y: u32,
        max_x: u32,
        max_y: u32,
        scale: Option<f32>,
    ) -> Self {
        let image = image::open(path).unwrap();
        let (width, height) = image.dimensions();

        let x_size = width / max_x;
        let y_size = height / max_y;

        let sub_image = image.view(x * x_size, y * y_size, x_size, y_size);
        let mut new_image = DynamicImage::new_rgba8(x_size, y_size);

        for x in 0..x_size {
            for y in 0..y_size {
                new_image.put_pixel(x, y, sub_image.get_pixel(x, y));
            }
        }

        Self {
            width: x_size,
            height: y_size,
            scale: scale.unwrap_or(4.0),
            image: new_image,
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
