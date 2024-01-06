use image::{self, ImageBuffer, Rgba, Pixel};
use std::path::Path;

pub struct Assets {
    pub tex: Vec<[u8; 4]>,
    pub width: u32,
    pub height: u32,
}

impl Assets {
    pub fn load() -> Assets {
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open(Path::new("resources/texture.png"))
        .unwrap()
        .to_rgba8();
        let dims = image.dimensions();
        let mut tex = Vec::new();
        for j in 0..dims.1 {
            for i in 0..dims.0 {
                let p = image::ImageBuffer::get_pixel(&image, i, j).to_rgba();
                tex.push([p[0],p[1],p[2],p[3]])
            }
        }
        Assets {
            tex,
            width: dims.0,
            height: dims.1,
        }
    }
}