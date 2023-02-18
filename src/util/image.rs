use crate::math::vector::Vector3 as Point;
use crate::util::camera::Camera;
use crate::util::hittable::*;

type Color = Point;

#[derive(Copy, Clone, Debug)]
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub samples_per_pixel: i32,
}

pub struct DrawHeader<'a> {
    pub output_file: &'a str, 
    pub camera: &'a Camera,
    pub world: &'a World,
    pub draw_call: fn(RenderObject) -> Color
}

pub struct RenderObject<'a> {
    pub coordinate: Point,
    pub image: &'a Image,
    pub camera: &'a Camera,
    pub world: &'a World
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        println!("Created a new {}x{} image!", width, height);
        Image { width, height, samples_per_pixel: 20 }
    }

    pub fn draw(&self, header: DrawHeader) -> std::io::Result<()> {
        let mut render_contents: String = format!("P3\n{} {}\n255\n", self.width, self.height);

        // generate a list of each point to render, as a vec container
        let render_plane: Vec<RenderObject> = {
            let mut plane: Vec<RenderObject> = vec![];
            for n in (0..self.height).rev() {
                for m in 0..self.width {
                    // get image coordinate x and y normalized
                    // let nx = m  as f32 / (self.width-1) as f32;
                    // let ny = n as f32 / (self.height-1) as f32;

                    // Using un-normalized world space coordinates
                    // so that anti-alias properly calculates sample
                    // coordinates.

                    plane.push( RenderObject {
                        coordinate: Point::new(m as f32, n as f32, 0.0),
                        image: self,
                        camera: header.camera,
                        world: header.world
                    });
                }
            }
            plane
        };
        
        let pixel_val: Vec<Color> = render_plane
            .into_iter()
            .map(header.draw_call)
            .collect();

        for p in pixel_val {
            render_contents.push_str(&p.to_pixel());
        }

        use std::fs;
        fs::write(header.output_file, render_contents)?;
        Ok(())
    }
}
