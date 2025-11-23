use std::{ thread, time::{ Duration, Instant } };

use sdl3::{
    event::{ Event, WindowEvent },
    pixels::{ Color, PixelFormat },
    rect::Rect,
    sys::video::SDL_SetWindowShape,
    video::WindowFlags,
};

fn main() {
    let sdl = sdl3::init().unwrap();

    let video = sdl.video().unwrap();

    // canvas.set_opacity(0.5).unwrap();
    let image = image::open("intro.png").unwrap();

    let bytes = image.to_rgba8();
    const FRAME_COUNT: u32 = 60;
    const COLUMN_COUNT: u32 = 10;
    const LINE_COUNT: u32 = FRAME_COUNT.div_ceil(COLUMN_COUNT);
    println!("line count: {}", LINE_COUNT);
    let sprite_width = image.width().saturating_div(COLUMN_COUNT);
    let sprite_height = image.height().saturating_div(LINE_COUNT);

    let time_between_frame_ms = 2000 / FRAME_COUNT;
    let mut event_pump = sdl.event_pump().unwrap();

    let window = video
        .window(
            "Desktop Gremlin",
            image.width().saturating_div(COLUMN_COUNT),
            image.width().saturating_div(COLUMN_COUNT)
        )
        .borderless()
        .set_flags(WindowFlags::TRANSPARENT | WindowFlags::ALWAYS_ON_TOP | WindowFlags::BORDERLESS)
        .build()
        .unwrap();
    // println!("{:?}", window.window_pixel_format().bytes_per_pixel());
    // let surface = window.surface(&sdl.event_pump().unwrap()).unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_blend_mode(sdl3::render::BlendMode::Blend);
    let mut texture = canvas
        .create_texture_static(Some(PixelFormat::RGBA32), image.width(), image.height())
        .unwrap();

    let _ = texture.update(None, &bytes, (image.width() * 4) as usize);

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
    let mut i = 0;
    unsafe {
        SDL_SetWindowShape(canvas.window().raw(), std::ptr::null_mut());
    }
    let mut should_exit = false;

    // let event_channel = mpsc::channel();
    let mut bus = Instant::now();
    while !should_exit {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    should_exit = true;
                }
                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::Resized(w, h) => {
                            let w = w as u32;
                            let h = h as u32;

                            let _ = canvas.window_mut().set_size(u32::max(w, h), u32::max(w, h));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let rect = Rect::new(
            ((i % COLUMN_COUNT) * sprite_width) as i32,
            ((i / COLUMN_COUNT) * sprite_height) as i32,
            sprite_width,
            sprite_height
        );

        // let mut surface = surface::Surface
        //     ::new(sprite_width, sprite_height, PixelFormat::RGBA32)
        //     .unwrap();
        // surface.with_lock_mut(|buf| {
        //     buf.fill_with(0);
        // });
        if bus.elapsed() > Duration::from_millis(200) {
            unsafe {
                SDL_SetWindowShape(canvas.window().raw(), std::ptr::null_mut());
            }
        }

        canvas.clear();
        canvas.copy(&texture, rect, None).unwrap();
        canvas.present();
        // if bus.elapsed() > Duration::from_millis(200) {
        //     bus = Instant::now();
        //     unsafe {
        //         SDL_SetWindowShape(canvas.window().raw(), canvas.read_pixels(None).unwrap().raw());
        //     }
        // }
        thread::sleep(Duration::from_millis(time_between_frame_ms as u64));
        i += 1;
        if i == FRAME_COUNT {
            i = 0;
        }
        // canvas.clear();
    }
    println!("we exited safely!");
}

// enum RenderEvent {}
