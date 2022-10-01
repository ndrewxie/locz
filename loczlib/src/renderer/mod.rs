mod fixedpoint;
mod line;
mod rasterizer;

use super::assets;

pub struct Renderer {
    framebuffer: Vec<u8>,
    pub raw_width: usize,
    pub raw_height: usize,
}
impl Renderer {
    pub fn new(raw_width: usize, raw_height: usize) -> Self {
        Self {
            framebuffer: vec![0; 4 * raw_width * raw_height],
            raw_width,
            raw_height,
        }
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8, a: u8) {
        let base = 4 * (y * self.raw_width + x);
        self.framebuffer[base] = r;
        self.framebuffer[base + 1] = g;
        self.framebuffer[base + 2] = b;
        self.framebuffer[base + 3] = a;
    }
    pub fn render(&mut self, mut time: u128) {
        let amogus = assets::AMONGUS;
        let dims = assets::DIMS_AMONGUS;
        let mut indx = 0;
        for y in 0..dims.1 {
            for x in 0..dims.0 {
                self.set_pixel(
                    x,
                    y,
                    amogus[indx],
                    amogus[indx + 1],
                    amogus[indx + 2],
                    amogus[indx + 3],
                );
                indx += 4;
            }
        }

        time /= 20_u128;
        let r = (3 * time % 256) as u8;
        let g = (4 * time % 256) as u8;
        let b = (5 * time % 256) as u8;

        for dx in 0..30 {
            let dx_f32 = dx as f32;
            let mut path = line::YSampledLine::new(
                &[100.0 + dx_f32, 100.0, 0.0, 1.0],
                &[500.0 + dx_f32, 400.0, 0.0, 1.0],
            );
            for y in path.start_y..path.end_y {
                self.set_pixel(path.x as usize, y as usize, r, g, b, 255);
                path.step();
            }
        }
    }
    pub fn framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }
}
