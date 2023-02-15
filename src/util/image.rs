use crate::util::vector::Vector3 as Point;
type Color = Point;

#[derive(Debug)]
pub struct Image {
    pub width: i32,
    pub height: i32,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        println!("Created a new {}x{} image!", width, height);
        Image { width, height }
    }

    pub fn draw(&self, output_file: &str, draw_call: fn(Point) -> Color) -> std::io::Result<()> {
        let mut render_contents: String = format!("P3\n{} {}\n255\n", self.width, self.height);

        let norm_plane: Vec<Point> = {
            let mut plane: Vec<Point> = vec![];
            for n in (0..self.height).rev() {
                for m in 0..self.width {
                    let nx = m  as f32 / (self.width-1) as f32;
                    let ny = n as f32 / (self.height-1) as f32;

                    plane.push(Point::new(nx, ny, 0.0)); 
                }
            }
            plane
        };
        
        let pixel_val: Vec<Color> = norm_plane
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
