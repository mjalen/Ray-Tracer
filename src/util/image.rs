use crate::math::vector::Vector3 as Point;
use crate::util::camera::Camera;

type Color = Point;

#[derive(Copy, Clone, Debug)]
pub struct Image {
    pub width: i32,
    pub height: i32,
}

pub struct RenderObject<'a> {
    pub coordinate: Point,
    pub image: &'a Image,
    pub camera: &'a Camera,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        println!("Created a new {}x{} image!", width, height);
        Image { width, height }
    }

    pub fn draw(&self, output_file: &str, camera: &Camera, draw_call: fn(RenderObject) -> Color) -> std::io::Result<()> {
        let mut render_contents: String = format!("P3\n{} {}\n255\n", self.width, self.height);

        let render_plane: Vec<RenderObject> = {
            let mut plane: Vec<RenderObject> = vec![];
            for n in (0..self.height).rev() {
                for m in 0..self.width {
                    let nx = m  as f32 / (self.width-1) as f32;
                    let ny = n as f32 / (self.height-1) as f32;

                    plane.push( RenderObject {
                        coordinate: Point::new(nx, ny, 0.0),
                        image: self,
                        camera
                    });
                }
            }
            plane
        };
        
        let pixel_val: Vec<Color> = render_plane
            .into_iter()
            .map(draw_call)
            .collect();

        for p in pixel_val {
            render_contents.push_str(&p.to_pixel());
        }

        use std::fs;
        fs::write(output_file, render_contents)?;
        Ok(())
    }
}
