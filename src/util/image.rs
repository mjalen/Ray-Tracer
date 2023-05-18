// use std::thread;
// use std::sync::mpsc;

use std::io::{self, Write};
// use std::thread;
// use std::sync::*;
// use std::thread::JoinHandle;

use crate::math::vector::Point;
use crate::util::camera::Camera;
use crate::util::hittable::*;

#[derive(Copy, Clone, Debug)]
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub samples_per_pixel: i32,
    pub fov: f32
}

pub struct DrawHeader<'a> {
    pub output_file: &'a str, 
    pub camera: &'a Camera,
    pub world: &'a World,
}

#[derive(Clone)]
pub struct RenderObject {
    pub coordinate: Point,
    pub image: Image,
    pub camera: Camera,
    pub world: World,
}

impl Image {
    pub fn new(width: i32, height: i32, fov: f32, samples: i32) -> Self {
        println!("Created a new {}x{} image!", width, height);
        Image { width, height, fov, samples_per_pixel: samples }
    }

    pub fn draw(&self, header: &DrawHeader) -> std::io::Result<()> {
        // let mut render_contents: String = format!("P3\n{} {}\n255\n", self.width, self.height);

        // render each pixel of this image, pixel-by-pixel. 
        print!("\nCreating framebuffer at {} samples/pixel...\n", self.samples_per_pixel);

        // let mut row_contents: Vec<JoinHandle<String>> = vec![];
        let mut render_object = RenderObject {
            coordinate: Point::origin(),
            image: self.to_owned(),
            camera: header.camera.to_owned(),
            world: header.world.to_owned()
        };

        // goofy mess that attempts to save memory 
        let image = 
            format!("P3\n{} {}\n255\n", self.width, self.height) // output head 
            + // plus the image contents
            (0..self.height).rev().map(|y| {
                let row = (0..self.width).map(|x| {
                    render_object.coordinate = Point::new(x as f32, y as f32, 0.0).to_owned();

                    let color_str = render_object.camera
                        .sample_pixel(&render_object)
                        .scalar_mul(255.0)
                        .to_pixel();

                    // render_contents.push_str(&color_str);
                    color_str
                }).collect::<String>();

                let percentage_complete: i32 = (100.0 * (1.0 - (y as f32 / self.height as f32))) as i32;
                print!("\rFramebuffer: {}%", percentage_complete);
                let _ = io::stdout().flush();

                row
            }).collect::<String>().as_str();

        print!("\n");

        use std::fs;
        fs::write(header.output_file, image)?;
        Ok(())
    }
}
