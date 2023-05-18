use image::{io::Reader as ImageReader, GenericImageView, DynamicImage};

use crate::Color;

#[derive(Clone, Debug)]
pub enum Texture {
    Solid(Color),
    Img(DynamicImage)
}

impl Texture {
    pub fn from_color(c: Color) -> Self {
        Texture::Solid(c)
    }

    pub fn from_img(file_name: String) -> Self {
        let img = ImageReader::open(file_name.as_str());

        // verify and decode image
        if let Result::Ok(image_result) = img {
            if let Result::Ok(data) = image_result.decode() {
                return Texture::Img(data);
            }
        }

        // return black solid if image reader failed
        Texture::Solid(Color::new(0.0, 0.0, 0.0))
    }

    pub fn get(&self, u: &f32, v: &f32) -> Color {
        match self {
            Self::Solid(c) => c.to_owned(),
            Self::Img(img) => {
                let (i, j) = Texture::uv_to_ij(img, u, v);

                let rgba = img.get_pixel(i, j);

                Color { 
                    a: rgba[0] as f32, 
                    b: rgba[1] as f32, 
                    c: rgba[2] as f32
                }.scalar_div(255.0)
            }
        }
    }

    pub fn uv_to_ij(img: &DynamicImage, u: &f32, v: &f32) -> (u32, u32) {
        let u_clamp = u.clamp(0.0, 1.0);
        let v_clamp = 1.0 - v.clamp(0.0, 1.0);

        let width = img.width();
        let height = img.height();

        let mut i = (u_clamp * width as f32) as u32;
        let mut j = (v_clamp * height as f32) as u32;

        if i >= width { i = width - 1 }
        if j >= height { j = height - 1 }

        (i, j)
    }
}