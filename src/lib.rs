use egui::{CentralPanel, Frame, Visuals, Widget};
use std::process;
pub mod app;
pub mod ui;
pub mod utils;

use app::Unit;

use crate::ui::battle;

pub struct DQWMApp {
    pub value: i32,
    #[allow(dead_code)]
    texture: egui::TextureHandle,
    #[allow(dead_code)]
    texture1: egui::TextureHandle,
    bg_unit: egui::TextureHandle, // 最终纹理

    units: Vec<Vec<Unit>>,
    rem: f32,
}
impl DQWMApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx; // 获取egui上下文
        ctx.set_visuals(Visuals::light()); // 亮色主题
        utils::load_fonts(ctx, "icon", include_bytes!("../assets/fonts/icon.ttf")); // 加载自定义字体

        egui_extras::install_image_loaders(&cc.egui_ctx); // 注册图像加载器到egui上下文
        // 创建一个SVG纹理
        let texture = utils::get_svg_texture(&cc.egui_ctx, "#605e63", 50, 20, 7.5);
        let texture1 = utils::get_svg_texture(&cc.egui_ctx, "#7df604ff", 50, 20, 7.5);

        // 加载背景图片
        // let bg_texture =
        //     utils::load_jpg_texture_from_bytes(ctx, include_bytes!("../assets/battle.jpg"));

        // 加载PNG
        let bg_unit = utils::load_png_texture_from_bytes(ctx, include_bytes!("../assets/unit.png"));

        Self {
            value: 0,
            texture,
            texture1,
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
            let r = battle::QBattleView::new(
                self.units.clone(),
                self.units.clone(),
                self.bg_unit.id(),
                self.rem,
            )
            .ui(ui);

            if r.clicked() {
                log::info!("中间区域被点击了！");
                if let Some(first_col) = self.units.get_mut(0)
                    && let Some(first_unit) = first_col.get_mut(0)
                {
                    first_unit.hp = first_unit.hp.saturating_sub(10);
                }
            }
        });

        ctx.request_repaint(); // 立即刷新
    }
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
