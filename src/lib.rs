use crate::core::batttle::{ArmySnapshot, BattleEvent, BattleOutput};
use crate::{components::battle_page, model::Unit};

use eframe::{App, NativeOptions};
use egui::{
    Align2, CentralPanel, Color32, Context, FontId, Frame, Id, LayerId, Order, Plugin, Rect,
    TextureHandle, TextureId, Ui, Vec2, Visuals, pos2,
};
use flume::Receiver;
use global::global_tokio_runtime;
use image::ImageFormat;
use std::{
    collections::{HashMap, VecDeque},
    process,
    sync::Arc,
    time::{Duration, Instant},
};

pub mod components;
pub mod core;
pub mod global;
pub mod model;
pub mod utils;

pub const APP_NAME: &str = "道起微末";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R {
    UnitShadow,
}
impl R {
    // 加载方法
    pub fn load(self, ctx: &Context) -> TextureHandle {
        utils::load_texture_from_bytes(
            ctx,
            include_bytes!("../assets/unit_shadow.png"),
            ImageFormat::Png,
        )
    }
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
            let mut map = data
                .get_temp::<HashMap<R, TextureHandle>>(Id::new(Key::Resource))
                .expect("未初始化资源HashMap!");
            map.entry(r).or_insert_with(|| r.load(self.ctx())).id()
        })
    }
}

#[derive(Debug)]
struct FPSPlugin {
    fps: f32,
    last_update: Instant,
    frame_count: u32,
}
impl Default for FPSPlugin {
    fn default() -> Self {
        Self {
            fps: 0.0,
            last_update: Instant::now(),
            frame_count: 0,
        }
    }
}

impl Plugin for FPSPlugin {
    fn debug_name(&self) -> &'static str {
        "FPS"
    }

    fn on_begin_pass(&mut self, ctx: &Context) {
        let now = Instant::now();
        self.frame_count += 1;

        // 每 0.5 秒更新一次 FPS（避免数字跳得太快）
        if now.duration_since(self.last_update) >= Duration::from_millis(500) {
            let elapsed_secs = now.duration_since(self.last_update).as_secs_f32();
            self.fps = self.frame_count as f32 / elapsed_secs;
            self.frame_count = 0;
            self.last_update = now;
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, egui::Id::new("fps_debug")));
        painter.text(
            pos2(10.0, 10.0), // 文本位置
            Align2::LEFT_TOP, // 对齐方式
            format!("FPS:{:.1}", self.fps),
            FontId::monospace(14.0),      // 字体大小
            Color32::from_rgb(0, 255, 0), // 颜色
        );
    }
}

pub struct Application {
    battle_rx: Option<Receiver<BattleOutput>>,
    current_army: ArmySnapshot,
    current_event: VecDeque<BattleEvent>,
}
impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx: &Context = &cc.egui_ctx; // 获取egui上下文
        ctx.set_visuals(Visuals::light()); // 亮色主题
        egui_extras::install_image_loaders(&cc.egui_ctx); // 注册图像加载器到egui上下文
        load_fonts(ctx, "iconfont", include_bytes!("../assets/fonts/icon.ttf")); // 加载自定义字体

        ctx.add_plugin(FPSPlugin::default());
        let res: HashMap<R, TextureHandle> = Default::default();
        // 资源
        ctx.data_mut(|w| {
            w.insert_temp::<HashMap<R, TextureHandle>>(Id::new(Key::Resource), res);
        });
        // ctx.data(|r| r.get_temp::<u32>(Id::new(1)));

        // let (army_tx, army_rx) = channel::bounded(2);
        // let (event_tx, event_rx) = channel::bounded(128);

        Self {
            battle_rx: None,
            current_army: Default::default(),
            current_event: Default::default(),
        }
    }
}

impl App for Application {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.interaction.selectable_labels = false; // ← 关掉，否则文本会有选中态
        style.spacing.item_spacing = Vec2::ZERO;
        ctx.set_style(style);

        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            ui.painter()
                .rect_filled(ctx.viewport_rect(), 0.0, Color32::WHITE);

            // battle_page::render(ui, &self.utils);

            if let Some(ref mut rx) = self.battle_rx {
                // 只有非空时才执行以下所有逻辑
                while let Ok(out) = rx.try_recv() {
                    match out {
                        BattleOutput::ArmySnapshot(army) => {
                            self.current_army = army;
                        }
                        BattleOutput::BattleEvent(event) => {
                            println!("当前事件{:?}", event);
                            self.current_event.push_back(event);

                            // 自动限制最多 50 条：超出就从前面弹出
                            if self.current_event.len() > 50 {
                                self.current_event.pop_front();
                            }
                        }
                    }
                }
            }
            let (a, b, c) = battle_page::render(ui, &self.current_army, &self.current_event);

            if a.clicked() {
                let (battle_tx, battle_rx) = flume::bounded::<BattleOutput>(10);
                self.battle_rx = Some(battle_rx);
                global_tokio_runtime().spawn(async move {
                    log::info!("开始==》");
                    let _ = battle_tx.send(BattleOutput::ArmySnapshot(ArmySnapshot {
                        enemys: model::test(120),
                        allys: model::test(120),
                        enemys_num: 120,
                        allys_num: 120,
                    }));
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    loop {
                        for i in [1, 2, 3, 4, 5, 6] {
                            let _ = battle_tx.send(BattleOutput::BattleEvent(BattleEvent::atk(i)));
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        }
                        for i in [121, 122, 123, 124, 125, 126] {
                            let _ = battle_tx
                                .send(BattleOutput::BattleEvent(BattleEvent::def(i, i * 10)));
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        }
                    }
                });
            }
            if b.clicked() {
                global_tokio_runtime().spawn(async {
                    log::info!("开始==》");
                    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
                    // 后台逻辑写在这里（不能操作 UI）
                });
            }
            if c.clicked() {
                global_tokio_runtime().spawn(async {
                    log::info!("开始==》");
                    tokio::time::sleep(tokio::time::Duration::from_secs(9)).await;
                    // 后台逻辑写在这里（不能操作 UI）
                });
            }
        });

        ctx.request_repaint(); // 立即刷新
    }
}

pub fn run(options: NativeOptions) {
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
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let handle = rt.handle().clone();
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
