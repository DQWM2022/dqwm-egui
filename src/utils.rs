use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Cursor;
use std::sync::Arc;

use egui::{Context, TextureHandle};
use image::ImageFormat;

pub fn load_png(img_bytes: &[u8], format: image::ImageFormat) -> egui::IconData {
    // 使用 match 或 if let 替代 unwrap
    let img = match image::load(Cursor::new(img_bytes), format) {
        Ok(img) => img.to_rgba8(),
        Err(e) => {
            eprintln!("加载图标PNG失败: {}", e);
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

pub fn load_png_texture_from_bytes(ctx: &Context, png_bytes: &[u8]) -> egui::TextureHandle {
    let mut hasher = DefaultHasher::new();
    png_bytes.hash(&mut hasher); // 计算png_bytes的哈希值作为纹理的唯一标识符
    let id = hasher.finish().to_string();
    let img = image::load_from_memory_with_format(png_bytes, image::ImageFormat::Png)
        .expect("加载图片失败")
        .to_rgba8();

    let size = [img.width() as usize, img.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
    ctx.load_texture(id, color_image, egui::TextureOptions::LINEAR)
}

pub fn load_texture_from_bytes(
    ctx: &Context,
    png_bytes: &[u8],
    image_format: ImageFormat,
) -> TextureHandle {
    let mut hasher = DefaultHasher::new();
    png_bytes.hash(&mut hasher); // 计算png_bytes的哈希值作为纹理的唯一标识符
    let id = hasher.finish().to_string();
    let img = image::load_from_memory_with_format(png_bytes, image_format)
        .unwrap_or_default()
        .to_rgba8();

    let size = [img.width() as usize, img.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
    ctx.load_texture(id, color_image, egui::TextureOptions::LINEAR)
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
