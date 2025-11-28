use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use eframe::egui;
use egui::{CentralPanel, TextureId, Visuals, Widget};

pub mod svg_utils;
pub mod ui {
    pub mod button;
}
pub mod game {
    pub mod res;
}
use game::res::{Res, ResType};

pub struct DQWMApp {
    pub value: i32,
    is_dark: bool,
    #[allow(dead_code)]
    texture: egui::TextureHandle,
    #[allow(dead_code)]
    texture1: egui::TextureHandle,
    aspect_ratio: f32, // 宽高比
    index: usize,      // 索引
    nav_tabs: Vec<(String, TextureId)>,

    // 游戏部分 => 资源
    data: Arc<RwLock<Vec<Res>>>,
    //
}

impl DQWMApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx; // 获取egui上下文
        ctx.set_visuals(Visuals::light()); // 亮色主题

        egui_extras::install_image_loaders(&cc.egui_ctx); // 注册图像加载器到egui上下文
        // 创建一个SVG纹理
        let texture = svg_utils::get_svg_texture(&cc.egui_ctx, "#605e63", 50, 20, 7.5);
        let texture1 = svg_utils::get_svg_texture(&cc.egui_ctx, "#7df604ff", 50, 20, 7.5);

        let tid1 = texture.id();
        let tid2 = texture1.id();

        let data = Arc::new(RwLock::new(vec![
            Res {
                name: ResType::Food.to(),
                num: 10,
                max: 10000,
            },
            Res {
                name: ResType::Wood.to(),
                num: 100,
                max: 10000,
            },
            Res {
                name: ResType::Stone.to(),
                num: 1000,
                max: 10000,
            },
        ]));

        let data_clone = data.clone();
        // 后台写线程
        thread::spawn(move || {
            let mut last_food = Instant::now();
            let mut last_wood = Instant::now();
            let mut last_stone = Instant::now();

            loop {
                let now = Instant::now();

                // 粮食：1 秒
                if now.duration_since(last_food) >= Duration::from_secs(1) {
                    let mut data = data_clone.write().unwrap();
                    data[0].num += 100;
                    last_food = now;
                }

                // 木材：100 ms
                if now.duration_since(last_wood) >= Duration::from_millis(100) {
                    let mut data = data_clone.write().unwrap();
                    data[1].num += 10;
                    last_wood = now;
                }

                // 石头：10 ms
                if now.duration_since(last_stone) >= Duration::from_millis(10) {
                    let mut data = data_clone.write().unwrap();
                    data[2].num += 1;
                    last_stone = now;
                }

                // 让出 CPU，防止空转占满一个核心
                thread::sleep(Duration::from_millis(1)); // 1ms
            }
        });

        Self {
            value: 0,
            is_dark: false,
            texture: texture,
            texture1: texture1,
            aspect_ratio: 50.0 / 20.0,
            index: 0,
            nav_tabs: vec![
                ("领地".to_string(), tid1),
                ("游戏".to_string(), tid1),
                ("商店".to_string(), tid2),
                ("诸天".to_string(), tid2),
            ],
            data: data,
        }
    }
}

impl eframe::App for DQWMApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.interaction.selectable_labels = false; // ← 关掉
        ctx.set_style(style);

        //  宽高
        let (w, h) = (ctx.viewport_rect().width(), ctx.viewport_rect().height());
        CentralPanel::default().show(ctx, |ui| {
            fps_label(ui);
            ui.heading(format!(
                "屏幕宽：{w} 高：{h}  \n\n当前页面: {} {}",
                self.index,
                self.nav_tabs[self.index].0.clone()
            ));
            if ui.button("切换主题").clicked() {
                if self.is_dark {
                    ctx.set_visuals(Visuals::light());
                    self.is_dark = false;
                } else {
                    ctx.set_visuals(Visuals::dark());
                    self.is_dark = true;
                }
            }

            // 读数据
            let data = self.data.read().unwrap();
            let food = &data[0];
            let wood = &data[1];
            let stone = &data[2];
            ui.label(format!("{}:{}/{}", food.name, food.num, food.max));
            ui.label(format!("{}:{}/{}", wood.name, wood.num, wood.max));
            ui.label(format!("{}:{}/{}", stone.name, stone.num, stone.max));

            egui::TopBottomPanel::bottom("button")
                .min_height(h * 0.06)
                .max_height(h * 0.06)
                .frame(egui::Frame::new().inner_margin(0.)) // 去内边距
                .show(ctx, |ui| {
                    let total_w = ui.available_width(); // 总宽
                    let max_h = ui.available_height();
                    let gap = 8.0; // 总间隙
                    let gap_half = gap;
                    let btn_w = (total_w - gap * 5.0) / 4.0;
                    let mut btn_h = btn_w / self.aspect_ratio; // 按钮高

                    if btn_h > max_h {
                        // 超高就放弃比例
                        btn_h = max_h; // 高锁死
                    }

                    ui.allocate_ui_with_layout(
                        ui.available_size(),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            // 头垫一半
                            ui.add_space(gap_half);
                            ui.spacing_mut().item_spacing.x = gap; // 按钮之间间隙

                            let btn_size = egui::vec2(btn_w, btn_h);

                            // 2. 带下标循环
                            for (idx, (label, tex)) in self.nav_tabs.iter().enumerate() {
                                //let r = bg_button(ui, label, *tex, btn_size);
                                let r = ui::button::QButton::new(label, *tex, btn_size).ui(ui);
                                if r.clicked() {
                                    self.index = idx; // 点谁设谁
                                }
                            }
                            // 尾垫一半
                            ui.add_space(gap_half);
                        },
                    );
                });
        });
        ctx.request_repaint(); // 立即刷新
    }
}

// 极简 FPS 小部件
pub fn fps_label(ui: &mut egui::Ui) {
    let fps = ui.ctx().input(|i| i.unstable_dt.recip()); // 官方给的上一帧耗时
    ui.label(format!("{:.0} FPS", fps));
}

pub fn _main(options: eframe::NativeOptions) {
    eframe::run_native(
        "道起微末",
        options,
        Box::new(|cc| Ok(Box::new(DQWMApp::new(cc)))),
    )
    .unwrap();
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
    _main(options);
}
