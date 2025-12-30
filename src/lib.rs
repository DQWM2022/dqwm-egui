use egui::{
    CentralPanel, Color32, Context, Frame, Id, Rect, TextureHandle, TextureId, Ui, Vec2, Visuals,
};
use image::ImageFormat;

use std::{
    collections::{HashMap, VecDeque},
    process,
    sync::Arc,
};

use crate::{model::Unit, unit_card::battle_ui};
pub mod model;
pub mod unit_card;
pub mod utils;
pub const APP_NAME: &str = "道起微末";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R {
    UnitShadow,
}
impl R {
    // 加载方法
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Resource,
}

//**拓展Ui */
pub trait UiExt {
    fn rem(&self, value: f32) -> f32;
    fn bg(&self, color: Color32);
    fn bg_rect(&self, rect: Rect, color: Color32);
    fn get_texture_id(&self, r: R) -> TextureId;
}
// 为 Ui 实现该 trait
impl UiExt for Ui {
    fn rem(&self, value: f32) -> f32 {
        self.ctx().viewport_rect().width() * 100.0 / 750.0 * value
    }

    //测试使用 设置背景
    fn bg(&self, color: Color32) {
        self.painter()
            .rect_filled(self.available_rect_before_wrap(), 0.0, color);
    }
    fn bg_rect(&self, rect: Rect, color: Color32) {
        self.painter().rect_filled(rect, 0.0, color);
    }
    fn get_texture_id(&self, r: R) -> TextureId {
        self.ctx().data(|data| {
            data.get_temp::<HashMap<R, TextureHandle>>(Id::new(Key::Resource))
                .expect("未初始化资源HashMap!")
                .get(&r)
                .expect("未初始化资源！")
                .id()
        })
    }
}

pub struct Application {
    utils: Vec<VecDeque<Unit>>,
}
impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx: &Context = &cc.egui_ctx; // 获取egui上下文
        ctx.set_visuals(Visuals::light()); // 亮色主题
        egui_extras::install_image_loaders(&cc.egui_ctx); // 注册图像加载器到egui上下文
        load_fonts(ctx, "iconfont", include_bytes!("../assets/fonts/icon.ttf")); // 加载自定义字体

        let mut res: HashMap<R, TextureHandle> = Default::default();

        res.insert(
            R::UnitShadow,
            utils::load_texture_from_bytes(
                ctx,
                include_bytes!("../assets/unit_shadow.png"),
                ImageFormat::Png,
            ),
        );
        // 资源
        ctx.data_mut(|w| {
            w.insert_temp::<HashMap<R, TextureHandle>>(Id::new(Key::Resource), res);
        });
        // ctx.data(|r| r.get_temp::<u32>(Id::new(1)));
        let utils = model::test(120);
        Self { utils }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.interaction.selectable_labels = false; // ← 关掉，否则文本会有选中态
        style.spacing.item_spacing = Vec2::ZERO;
        ctx.set_style(style);
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            ui.painter()
                .rect_filled(ctx.viewport_rect(), 0.0, Color32::WHITE);

            battle_ui(ui, &self.utils);
        });
        ctx.request_repaint(); // 立即刷新
    }
}

pub fn run(options: eframe::NativeOptions) {
    if let Err(err) = eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
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
    run(options);
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
