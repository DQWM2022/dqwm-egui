use resvg;
use std::{io::Cursor, num::ParseIntError, sync::Arc};
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

// 加载 PNG 图片并创建纹理
pub fn load_png_texture_from_bytes(ctx: &egui::Context, png_bytes: &[u8]) -> egui::TextureHandle {
    let img = image::load_from_memory_with_format(png_bytes, image::ImageFormat::Png)
        .expect("加载图片失败")
        .to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
    ctx.load_texture(Uuid::new_v4(), color_image, egui::TextureOptions::LINEAR)
}

// 加载JPG图片并创建纹理
pub fn load_jpg_texture_from_bytes(ctx: &egui::Context, jpg_bytes: &[u8]) -> egui::TextureHandle {
    let img = image::load_from_memory_with_format(jpg_bytes, image::ImageFormat::Jpeg)
        .expect("加载图片失败")
        .to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
    ctx.load_texture(Uuid::new_v4(), color_image, egui::TextureOptions::LINEAR)
}

pub fn load_fonts(ctx: &egui::Context, key: &str, font_bytes: &[u8]) {
    // 创建默认字体配置容器
    let mut fonts = egui::FontDefinitions::default();

    // 注册自定义字体数据（需提前放置simsun.ttc在项目根目录）
    fonts.font_data.insert(
        key.to_owned(),
        Arc::new(egui::FontData::from_owned(font_bytes.to_vec())),
    );

    // 配置比例字体家族（用于常规文本）
    fonts
        .families // 访问字体家族集合
        .entry(egui::FontFamily::Proportional) // 获取比例字体入口
        .or_default() // 不存在则创建默认列表
        .insert(0, key.to_owned()); // 插入到最高优先级

    // 配置等宽字体家族（用于代码/表格）
    fonts
        .families
        .entry(egui::FontFamily::Monospace) // 获取等宽字体入口
        .or_default()
        .insert(0, key.to_owned()); // 插入到最高优先级

    // 将最终配置应用到egui上下文
    ctx.set_fonts(fonts);
}

// 加载图片
pub fn load_icon_png(img_bytes: &[u8], format: image::ImageFormat) -> egui::IconData {
    // 使用 match 或 if let 替代 unwrap
    let img = match image::load(Cursor::new(img_bytes), format) {
        Ok(img) => img.to_rgba8(),
        Err(e) => {
            eprintln!("Failed to load icon PNG: {}", e);
            // 返回一个 1x1 的透明占位图标，避免崩溃
            return egui::IconData {
                rgba: vec![0, 0, 0, 0],
                width: 1,
                height: 1,
            };
        }
    };

    let (w, h) = (img.width(), img.height());
    egui::IconData {
        rgba: img.into_raw().to_vec(),
        width: w,
        height: h,
    }
}
