use resvg;
use std::num::ParseIntError;
use uuid::Uuid;

pub fn get_svg_texture(
    ctx: &egui::Context,
    bg_hex: &str,
    w: u32,
    h: u32,
    blur: f32,
) -> egui::TextureHandle {
    let svg_str = bg_svg(w, h, blur, bg_hex);
    let img: egui::ColorImage = egui_extras::image::load_svg_bytes_with_size(
        svg_str.as_bytes(),
        egui::SizeHint::Size {
            width: (w),
            height: (h),
            maintain_aspect_ratio: (false),
        },
        &resvg::usvg::Options::default(),
    )
    .expect("加载图片失败");
    ctx.load_texture(Uuid::new_v4(), img, egui::TextureOptions::LINEAR)
}

fn bg_svg(w: u32, h: u32, blur: f32, rgba: &str) -> String {
    // w 50 h 20 blur 7.5
    let rgba = hex_to_tuple(rgba).unwrap_or((0.0, 0.0, 0.0, 0.0)); // 默认透明
    format!(
        r#"<svg width="{w}" height="{h}" viewBox="0 0 {w} {h}" xmlns="http://www.w3.org/2000/svg">
  <rect width="{w}" height="{h}" filter="url(#insetShadow)"/>
  <filter id="insetShadow" x="-50%" y="-50%" width="200%" height="200%">
    <feGaussianBlur in="SourceAlpha" stdDeviation="{blur}" result="blur"/>
    <feComposite in="blur" in2="SourceAlpha" operator="arithmetic"
                 k2="-1" k3="1" result="innerBlur"/>
    <feColorMatrix in="innerBlur" type="matrix"
      values="0 0 0 0 {}
              0 0 0 0 {}
              0 0 0 0 {}
              0 0 0 {} 0"/>
  </filter>
</svg>"#,
        rgba.0, rgba.1, rgba.2, rgba.3
    )
}

fn hex_to_tuple(hex: &str) -> Result<(f32, f32, f32, f32), ParseIntError> {
    let hex = hex.trim_start_matches('#');

    // 解析红色部分
    let r = u8::from_str_radix(&hex[0..2], 16).map(|v| v as f32 / 255.0)?;

    // 解析绿色部分
    let g = u8::from_str_radix(&hex[2..4], 16).map(|v| v as f32 / 255.0)?;

    // 解析蓝色部分
    let b = u8::from_str_radix(&hex[4..6], 16).map(|v| v as f32 / 255.0)?;

    // 解析透明度部分，如果不存在则默认为1.0
    let a = hex.get(6..8).map_or(Ok(1.0), |s| {
        u8::from_str_radix(s, 16).map(|v| v as f32 / 255.0)
    })?;

    Ok((r, g, b, a))
}
