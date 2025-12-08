use egui::{CentralPanel, Context, Frame, Visuals};
use std::{
    process,
    sync::{Arc, mpsc::Sender},
};
pub mod app;
pub mod double_buffer;
pub mod gui;
pub mod utils;

use crate::{
    app::service::{Army, GameService},
    double_buffer::DoubleBuffer,
    gui::battle,
};

use std::sync::OnceLock;
use tokio::runtime::Runtime;

fn global_tokio_runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("无法创建全局Tokio运行时环境！")
    })
}

#[derive(Debug)]
pub enum GameCommand {
    Army(usize, usize), // 战斗信息
    StartBattle,
    StopBattle,
    StopService,
}

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
    cmd_tx: Sender<GameCommand>,
    army: Arc<DoubleBuffer<Army>>,
    #[allow(dead_code)]
    texture: egui::TextureHandle,
    #[allow(dead_code)]
    texture1: egui::TextureHandle,
    unit_bg: egui::TextureHandle, // 最终纹理
    rem: f32,
    current: AppPage,
    num1: String,
    num2: String,
}
impl DQWMApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx: &Context = &cc.egui_ctx; // 获取egui上下文
        ctx.set_visuals(Visuals::light()); // 亮色主题
        utils::load_fonts(ctx, "icon", include_bytes!("../assets/fonts/icon.ttf")); // 加载自定义字体

        egui_extras::install_image_loaders(&cc.egui_ctx); // 注册图像加载器到egui上下文
        // 创建一个SVG纹理
        let texture = utils::get_svg_texture(&cc.egui_ctx, "#605e63", 50, 20, 7.5);
        let texture1 = utils::get_svg_texture(&cc.egui_ctx, "#7df604ff", 50, 20, 7.5);

        // 加载PNG
        let unit_bg = utils::load_png_texture_from_bytes(ctx, include_bytes!("../assets/unit.png"));
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<GameCommand>();
        let buffer = Arc::new(DoubleBuffer::<Army>::new(Army::default()));

        GameService::new(cmd_rx, Arc::clone(&buffer)).start(); // 启动游戏服务
        Self {
            cmd_tx,
            army: buffer,
            texture,
            texture1,
            unit_bg,
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
                    if let Err(e) = self.cmd_tx.send(GameCommand::Army(num1, num2)) {
                        log::error!("发送开始战斗命令失败: {}", e);
                    }
                    self.current = AppPage::Battle;
                }

                if ui.button("首页页面").clicked() {
                    self.current = AppPage::Index;
                }
                if ui.button("战斗页面").clicked() {
                    self.current = AppPage::Battle;
                }

                if ui.button("开始战斗").clicked() {
                    self.current = AppPage::Battle;
                    if let Err(e) = self.cmd_tx.send(GameCommand::Army(600, 600)) {
                        log::error!("发送开始战斗命令失败: {}", e);
                    }
                }

                // 👇👇👇 新增：测试按钮 👇👇👇
                ui.separator();
                if ui.button("🧪 测试异步任务").clicked() {
                    log::info!("【UI线程】点击了测试按钮");

                    global_tokio_runtime().spawn(async move {
                        log::info!("【Tokio后台】异步任务已启动！");
                        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

                        log::info!("【Tokio后台】1秒后执行完毕");
                    });
                }
            });
        });
    }

    fn battle_page(&mut self, ctx: &Context) {
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            let army_arc = self.army.read();

            let r = battle::QBattleView::new(self.unit_bg.id(), self.rem).render(
                &army_arc.enemy_units,
                &army_arc.friendly_units,
                army_arc.enemy_num,
                army_arc.friendly_num,
                ui,
            );
            if r.0.clicked() {
                let _ = self.cmd_tx.send(GameCommand::StopBattle);
                log::info!("投降区域被点击了！");
            }
            if r.1.clicked() {
                log::info!("开始区域被点击了！");
                let _ = self.cmd_tx.send(GameCommand::StartBattle);
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
