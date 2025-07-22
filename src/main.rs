use std::path::{ PathBuf, Path };
use std::sync::Arc;
use std::{ fs, thread };
use image::{ ImageBuffer, ImageFormat, imageops };

mod util;
use util::*;

mod settings;
use settings::Settings;

mod render;


pub fn main() {
    let size       = 1024;
    let target_dir = Arc::new(PathBuf::from("generated"));
    let settingss = [
        Settings::DEFAULT,
        Settings::WINTER,
        Settings::HOLIDAY,
        Settings::AUTUMN,
        Settings::HALLOWEEN
    ];

    fs::create_dir_all(&*target_dir).unwrap();
    let mut handles = Vec::new();
    for settings in settingss {
        let target_dir = target_dir.clone();
        handles.push(thread::spawn(move || {
            render_icon(&settings, size, &*target_dir);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Done.");
}


fn render_icon(settings : &Settings, size : u32, target_dir : &Path) {
    println!("Rendering icon {:?} ({size})...", settings.name);
    let resolution = U32x2 { x : size, y : size };

    let max_extents = F32x2 { x : (resolution.x - 1) as f32, y : (resolution.y - 1) as f32 };

    let mut fragment_buf = ImageBuffer::new(resolution.x, resolution.y);
    for (x, y, px,) in fragment_buf.enumerate_pixels_mut() {
        let uv = F32x2 { x : (x as f32) / max_extents.x, y : (y as f32) / max_extents.y };
        *px = image::Rgba::<f32>::from(render::fragment(&settings, uv));
    }

    let mut peaking_buf = ImageBuffer::new(resolution.x, resolution.y);
    for ((_, _, dstpx,), (_, _, srcpx,),) in peaking_buf.enumerate_pixels_mut().zip(fragment_buf.enumerate_pixels()) {
        *dstpx = if (is_peaking(*srcpx)) { *srcpx } else { image::Rgba([0.0; 4]) };
    }
    let bloom_buf = imageops::fast_blur(&peaking_buf, (max_extents.x + max_extents.y) * 0.5 * 0.0234375);

    let mut final_buf = ImageBuffer::new(resolution.x, resolution.y);
    for ((_, _, dstpx,), ((_, _, fragpx,), (_, _, bloompx,),),) in final_buf.enumerate_pixels_mut().zip(fragment_buf.enumerate_pixels().zip(bloom_buf.enumerate_pixels())) {
        let mut bloompx = *bloompx;
        bloompx.0[3] *= 1.25;
        *dstpx = rgba_f32_to_u8(add(*fragpx, bloompx));
    }

    final_buf.save_with_format(target_dir.join(format!("{}-{}.png", settings.name, size)), ImageFormat::Png).unwrap();
    println!("Finished icon {:?} ({size}).", settings.name);
}


fn is_peaking(rgba : image::Rgba<f32>) -> bool {
    let t = rgba.0[3];
    ((rgba.0[0] * t) > 1.0) || ((rgba.0[1] * t) > 1.0) || ((rgba.0[2] * t) > 1.0)
}

fn add(a : image::Rgba<f32>, b : image::Rgba<f32>) -> image::Rgba<f32> {
    let t = b.0[3];
    image::Rgba([
        a.0[0] + (b.0[0] * t),
        a.0[1] + (b.0[1] * t),
        a.0[2] + (b.0[2] * t),
        a.0[3] + (b.0[3] * t)
    ])
}

fn rgba_f32_to_u8(rgba : image::Rgba<f32>) -> image::Rgba<u8> {
    image::Rgba([
        (rgba.0[0].clamp(0.0, 1.0) * (u8::MAX as f32)) as u8,
        (rgba.0[1].clamp(0.0, 1.0) * (u8::MAX as f32)) as u8,
        (rgba.0[2].clamp(0.0, 1.0) * (u8::MAX as f32)) as u8,
        (rgba.0[3].clamp(0.0, 1.0) * (u8::MAX as f32)) as u8
    ])
}
