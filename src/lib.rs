use egui::{CentralPanel, Context, Frame, Visuals};
use std::process;
pub mod app;
pub mod gui;
pub mod utils;

use app::Unit;

use crate::{app::StartBattle, gui::battle};

#[derive(Copy, Clone)]
pub enum AppPage {
    Index,  // 对应 0
    Battle, // 对应 1
}

impl AppPage {
    pub const ALL: [Self; 2] = [Self::Index, Self::Battle];

    // 如果你真的需要索引（比如用于 UI 布局）
    pub fn index(self) -> usize {
        match self {
            Self::Index => 0,
            Self::Battle => 1,
        }
    }

    // 从索引转回枚举（用于处理点击事件等）
    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub struct DQWMApp {
    pub value: i32,
    #[allow(dead_code)]
    texture: egui::TextureHandle,
    #[allow(dead_code)]
    texture1: egui::TextureHandle,
    unit_bg: egui::TextureHandle, // 最终纹理

    battle: StartBattle,
    rem: f32,
    current: AppPage,
    num1: String,
    num2: String,
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

        // 加载PNG
        let unit_bg = utils::load_png_texture_from_bytes(ctx, include_bytes!("../assets/unit.png"));

        Self {
            value: 0,
            texture,
            texture1,
            unit_bg,
            battle: StartBattle::default(),
            rem: 50.0,
            current: AppPage::Index,
            num1: "".to_string(),
            num2: "".to_string(),
        }
    }
    fn index_page(&mut self, ctx: &Context) {
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            ui.vertical(|ui| {
                // 1. 获取当前可用区域的矩形
                let rect = ui.available_rect_before_wrap();
                // 2. 立即用 Painter 填充白色背景
                ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);

                ui.label("敌方数量");
                ui.text_edit_singleline(&mut self.num1);
                ui.label("我方数量");
                ui.text_edit_singleline(&mut self.num2);

                if ui.button("转换数量并开始战斗").clicked() {
                    let num1 = self.num1.parse().unwrap_or(0);
                    let num2 = self.num2.parse().unwrap_or(0);
                    let enemy = Unit::test(num1); // 敌方
                    let friendly = Unit::test(num2); // 友方

                    self.battle = StartBattle::new(enemy, friendly);
                    self.battle.run(); // 战斗在后台运行，UI 不卡
                    self.current = AppPage::Battle;
                }

                if ui.button("首页页面").clicked() {
                    self.current = AppPage::Index;
                }
                if ui.button("战斗页面").clicked() {
                    self.current = AppPage::Battle;
                }

                if ui.button("开始战斗").clicked() {
                    let enemy = Unit::test(10); // 敌方
                    let friendly = Unit::test(12); // 友方

                    self.battle = StartBattle::new(enemy, friendly);
                    self.battle.run(); // 战斗在后台运行，UI 不卡
                    self.current = AppPage::Battle;
                }
            });
        });
    }

    fn battle_page(&mut self, ctx: &Context) {
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            let enemy_guard = self
                .battle
                .enemy_units
                .lock()
                .expect("Failed to lock enemy_units mutex");
            let friendly_guard = self
                .battle
                .friendly_units
                .lock()
                .expect("Failed to lock friendly_units mutex");
            let r = battle::QBattleView::new(self.unit_bg.id(), self.rem).render(
                &enemy_guard,
                &friendly_guard,
                ui,
            );

            if r.0.clicked() {
                log::info!("投降区域被点击了！");
            }
            if r.1.clicked() {
                log::info!("中间区域被点击了！");
            }
            if r.2.clicked() {
                self.current = AppPage::Index;
            }
        });
    }
}

impl eframe::App for DQWMApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.interaction.selectable_labels = false; // ← 关掉，否则文本会有选中态
        ctx.set_style(style);

        self.rem = (ctx.viewport_rect().width() * 100. / 750.).clamp(1.0, 100.0); // 设计稿750宽度基准

        // 根据 AppPage
        match self.current {
            AppPage::Index => self.index_page(ctx),
            AppPage::Battle => self.battle_page(ctx),
            // 如果有更多页面，继续加
        }
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
