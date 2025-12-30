use bad_signals::signals::{common::Signalable, signals::Signal};
use image::DynamicImage;
use sdl3::{
    pixels::Color,
    render::{Canvas, FRect, Texture},
    video::Window,
};

use crate::{
    gremlin::GLOBAL_PIXEL_FORMAT,
    ui::{Composable, Notify, Render},
    utils::{img_get_bytes_global, into_opt_rect},
};

pub struct Image {
    data: DynamicImage,
}

impl Image {
    pub fn new(file_dir: &str) -> anyhow::Result<Self> {
        Ok(Image {
            data: image::open(file_dir)?,
        })
    }
}

impl Render for Image {
    /// size of Image and rendering texture should be the same, otherwise the function would do panic
    fn render(
        &self,
        texture: &mut sdl3::render::Texture,
        rect: Option<sdl3::render::FRect>, // styles: Option<Vec<RenderStyle>>
    ) -> anyhow::Result<()> {
        texture.with_lock(into_opt_rect(rect), |buffer, _| {
            buffer.swap_with_slice(img_get_bytes_global(&self.data).unwrap().as_mut_slice())
        })?;

        Ok(())
    }

    fn render_canvas(
        &self,
        canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
        rect: Option<sdl3::render::FRect>, // styles: Option<Vec<RenderStyle>>s
    ) -> anyhow::Result<()> {
        let texture = canvas.texture_creator();

        let mut texture = texture.create_texture_static(
            GLOBAL_PIXEL_FORMAT,
            self.data.width(),
            self.data.height(),
        )?;

        let image_bytes = img_get_bytes_global(&self.data).unwrap();
        let image_bytes = image_bytes.as_slice();

        texture.update(
            None,
            image_bytes,
            (self.data.width() as usize) * GLOBAL_PIXEL_FORMAT.bytes_per_pixel(),
        )?;

        canvas.copy(&texture, None, rect)?;
        drop(texture);
        Ok(())
    }
}

impl Notify for Image {
    fn notify(&self, _: super::ComponentEvent) {}
}

impl Composable for Image {}

// kinda too lazy to implement this rn so maybe later
pub struct LazyImage {}

pub struct Button {
    pub color: Color,
    pub width: SizeUnit,
    pub height: SizeUnit,
    pub on_click: Signal<()>,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            width: SizeUnit::Percentage(100),
            height: SizeUnit::Pixel(100),
            on_click: Signal::new(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SizeUnit {
    Pixel(u32),
    Percentage(u32),
}

impl SizeUnit {
    pub fn pix(w: u32, h: u32) -> (SizeUnit, SizeUnit) {
        (SizeUnit::Pixel(w), SizeUnit::Pixel(h))
    }
    pub fn percentage(w: u32, h: u32) -> (SizeUnit, SizeUnit) {
        (SizeUnit::Percentage(w), SizeUnit::Percentage(h))
    }
}

impl Render for Button {
    fn render(
        &self,
        texture: &mut Texture,
        rect: Option<FRect>, // styles: Option<Vec<RenderStyle>>
    ) -> anyhow::Result<()> {
        let _ = texture.with_lock(into_opt_rect(rect), |buf, _| {
            for i in 0..buf.len() {
                match i % 4 {
                    0 => {
                        buf[i] = self.color.r;
                    }
                    1 => {
                        buf[i] = self.color.g;
                    }
                    2 => {
                        buf[i] = self.color.b;
                    }
                    3 => {
                        buf[i] = self.color.a;
                    }
                    _ => {}
                }
            }
        });
        Ok(())
    }

    fn render_canvas(
        &self,
        canvas: &mut Canvas<Window>,
        rect: Option<FRect>, // styles: Option<Vec<RenderStyle>>
    ) -> anyhow::Result<()> {
        let color = canvas.draw_color();
        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect).unwrap();
        canvas.set_draw_color(color);

        Ok(())
    }
}
