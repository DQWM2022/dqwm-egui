use egui::{CentralPanel, Frame, Visuals};
use std::process;
use std::sync::Arc;
pub mod app;
pub mod ui;
pub mod utils;

use app::Unit;

use crate::ui::area;

pub struct DQWMApp {
    pub value: i32,
    #[allow(dead_code)]
    texture: egui::TextureHandle,
    #[allow(dead_code)]
    texture1: egui::TextureHandle,
    bg_texture: egui::TextureHandle,
    bg_unit: egui::TextureHandle, // 最终纹理

    units: Vec<Vec<Unit>>,
    rem: f32,
}
impl DQWMApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx; // 获取egui上下文
        ctx.set_visuals(Visuals::light()); // 亮色主题
        load_fonts(ctx); // 加载自定义字体

        egui_extras::install_image_loaders(&cc.egui_ctx); // 注册图像加载器到egui上下文
        // 创建一个SVG纹理
        let texture = utils::get_svg_texture(&cc.egui_ctx, "#605e63", 50, 20, 7.5);
        let texture1 = utils::get_svg_texture(&cc.egui_ctx, "#7df604ff", 50, 20, 7.5);

        // 加载背景图片
        let bg_texture =
            utils::load_jpg_texture_from_bytes(ctx, include_bytes!("../assets/bg.jpg"));
        // 加载PNG
        let bg_unit = utils::load_png_texture_from_bytes(ctx, include_bytes!("../assets/unit.png"));

        Self {
            value: 0,
            texture,
            texture1,
            bg_texture,
            //bg_unit: svg_utils::get_texture_by_svg(&cc.egui_ctx, 100, 80),
            bg_unit,
            units: Unit::test(),
            rem: 22.0,
        }
    }
}

impl eframe::App for DQWMApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.interaction.selectable_labels = false; // ← 关掉，否则文本会有选中态
        ctx.set_style(style);

        self.rem = (ctx.viewport_rect().width() * 100. / 750.).clamp(1.0, 100.0); // 设计稿750宽度基准

        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            // 背景
            let screen_rect = ui.max_rect(); // 获取屏幕矩形

            ui.painter().image(
                self.bg_texture.id(),
                screen_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), // UV坐标
                egui::Color32::WHITE, // 色调（可以用来调暗或着色）
            );
            area::draw_area(ui, self.rem, &self.bg_unit, &self.units, true);
            if ui.button("text").clicked() {
                log::info!("按钮被点击了！");
                // 扣除第一个单位10点生命值作为示例
                if let Some(first_col) = self.units.get_mut(0)
                    && let Some(first_unit) = first_col.get_mut(0)
                {
                    if first_unit.hp >= 10 {
                        first_unit.hp -= 10;
                    } else {
                        first_unit.hp = 0;
                    }
                }
            }
            area::draw_area(ui, self.rem, &self.bg_unit, &self.units, false);
        });
        ctx.request_repaint(); // 立即刷新
    }
}

fn load_fonts(ctx: &egui::Context) {
    // 创建默认字体配置容器
    let mut fonts = egui::FontDefinitions::default();

    // 注册自定义字体数据（需提前放置simsun.ttc在项目根目录）
    fonts.font_data.insert(
        "icon".to_owned(), // 字体标识名
        Arc::new(
            // 使用Arc实现线程安全共享
            egui::FontData::from_owned(
                // 转换字体数据为egui格式
                include_bytes!("../assets/fonts/icon.ttf") // 编译时嵌入字体文件
                    .to_vec(), // 转为Vec<u8>
            ),
        ),
    );

    // 配置比例字体家族（用于常规文本）
    fonts
        .families // 访问字体家族集合
        .entry(egui::FontFamily::Proportional) // 获取比例字体入口
        .or_default() // 不存在则创建默认列表
        .insert(0, "icon".to_owned()); // 插入到最高优先级

    // 配置等宽字体家族（用于代码/表格）
    fonts
        .families
        .entry(egui::FontFamily::Monospace) // 获取等宽字体入口
        .or_default()
        .insert(0, "icon".to_owned()); // 插入到最高优先级

    // 将最终配置应用到egui上下文
    ctx.set_fonts(fonts);
}

pub fn run_app(options: eframe::NativeOptions) {
    if let Err(err) = eframe::run_native(
        "道起微末",
        options,
        Box::new(|cc| Ok(Box::new(DQWMApp::new(cc)))),
    ) {
        eprintln!("应用启动失败！: {}", err);
        process::exit(1);
    }
}

// ===== Android 入口 =====
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
extern "Rust" fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    run_app(options);
}
