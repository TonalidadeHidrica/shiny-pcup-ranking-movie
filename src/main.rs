use sdl2;
use cairo::{Context, Format, ImageSurface};
use sdl2::event::Event;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

fn main() -> anyhow::Result<()> {
    let (width, height) = (1920, 1080);

    let sdl_context = sdl2::init().map_err(anyhow::Error::msg)?;
    let video_subsystem = sdl_context.video().map_err(anyhow::Error::msg)?;
    let window = video_subsystem
        .window("", width / 2, height / 2)
        .allow_highdpi()
        .build()?;
    let mut canvas = window.into_canvas().build()?;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::ARGB8888,
        width as u32,
        height as u32,
    )?;
    texture.set_blend_mode(BlendMode::Blend);
    texture
        .with_lock(None, |pixels, stride| -> Result<_, anyhow::Error> {
            // let pixels =
            //     unsafe { std::slice::from_raw_parts_mut(pixels.as_mut_ptr(), pixels.len()) };
            // let surface = ImageSurface::create_for_data(
            //     pixels,
            //     Format::ARgb32,
            //     width as i32,
            //     height as i32,
            //     stride as i32,
            // )?;
            let pixels = unsafe {
                ImageSurface::create_for_data_unsafe(
                   pixels.as_mut_ptr(),
                   Format::ARgb32,
                   width as i32,
                   height as i32,
                   stride as i32,
                );
            };
            draw(Context::new(&surface));
            Ok(())
        })
        .map_err(anyhow::Error::msg)??;

    canvas.set_draw_color(Color::RGBA(20, 20, 20, 0));
    canvas.clear();
    canvas
        .copy(&texture, None, Rect::new(0, 0, width, height))
        .map_err(anyhow::Error::msg)?;
    canvas.present();

    for event in sdl_context
        .event_pump()
        .map_err(anyhow::Error::msg)?
        .wait_iter()
    {
        if let Event::Quit { .. } = event {
            break;
        }
    }

    Ok(())
}

fn draw(cr: Context){
}
