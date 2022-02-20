extern crate image;

use image::{GenericImageView, RgbaImage};

pub struct PhantomTank {
    fg: RgbaImage,
    bg: RgbaImage,
    height: u32,
    width: u32,
}

impl PhantomTank {
    pub fn new(path_fg: &str, path_bg: &str) -> Result<PhantomTank, image::ImageError> {
        // 取前景图的长宽
        let fg_raw = image::open(path_fg)?;
        let mut bg_raw = image::open(path_bg)?;

        let (width, height) = fg_raw.dimensions();
        bg_raw = bg_raw.resize(width, height, image::imageops::Nearest);

        Ok(PhantomTank {
            fg: fg_raw.to_rgba8(),
            bg: bg_raw.to_rgba8(),
            height,
            width,
        })
    }

    pub fn generate_grey(&self) -> RgbaImage {
        let fg_grey = self.to_gray(&self.fg, 1.0);
        let bg_grey = self.to_gray(&self.bg, 0.3);

        // fg_grey.save("fg_grey.png").unwrap();
        // bg_grey.save("bg_grey.png").unwrap();

        let mut ret = RgbaImage::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                let luma_fg = fg_grey.get_pixel(x, y).0[0];
                let luma_bg = bg_grey.get_pixel(x, y).0[0];

                let alpha = (255 - luma_fg).saturating_add(luma_bg);
                let luma_ret = ((luma_bg as f32 / alpha as f32) * 255.) as u8;

                ret.put_pixel(
                    x,
                    y,
                    image::Rgba::from([luma_ret, luma_ret, luma_ret, alpha]),
                );
            }
        }

        ret
    }
}

impl PhantomTank {
    fn to_gray(&self, pic: &RgbaImage, scale: f32) -> RgbaImage {
        let mut ret: RgbaImage = RgbaImage::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let p = pic.get_pixel(x, y);
                let grey = (((p.0[0] as u32 * 30 + p.0[1] as u32 * 59 + p.0[2] as u32 * 11) / 100)
                    as f32
                    * scale) as u8;
                ret.put_pixel(x, y, image::Rgba::from([grey, grey, grey, 255]));
            }
        }

        ret
    }
}
