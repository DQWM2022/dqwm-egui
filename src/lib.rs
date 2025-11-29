use std::{
    process,
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
            Res::new(ResType::Food.to(), 10, 10000, 1000, 100),
            Res::new(ResType::Wood.to(), 100, 10000, 100, 10),
            Res::new(ResType::Stone.to(), 1000, 10000, 10, 1),
        ]));

        let data_clone = data.clone();
        // 后台写线程
        thread::spawn(move || {
            loop {
                let now = Instant::now();

                // 尝试获取写锁
                if let Ok(mut resources) = data_clone.write() {
                    let mut next_wakeup = Duration::from_secs(1); // 默认最多等 1 秒

                    // 遍历所有资源，检查是否需要更新
                    for res in resources.iter_mut() {
                        let interval_ms = res.change_interval;
                        if interval_ms == 0 {
                            continue; // 跳过不自动增长的资源
                        }

                        let elapsed = now.duration_since(res.last_update);
                        let interval = Duration::from_millis(interval_ms);

                        if elapsed >= interval {
                            // 执行增长
                            res.num = (res.num + res.change_value).min(res.max);
                            res.last_update = now;
                        }

                        // 计算该资源下次更新还需等待多久
                        let remaining = interval.saturating_sub(elapsed);
                        if remaining < next_wakeup {
                            next_wakeup = remaining;
                        }
                    }

                    // 释放锁后再 sleep（减少锁持有时间）
                    drop(resources);

                    // 至少 sleep 1ms，防止忙等待；最多 sleep 1 秒（兜底）
                    thread::sleep(next_wakeup.max(Duration::from_millis(1)));
                } else {
                    // 锁被毒化（poisoned），短暂休眠后重试
                    thread::sleep(Duration::from_millis(10));
                }
            }
        });

        Self {
            value: 0,
            is_dark: false,
            texture,
            texture1,
            aspect_ratio: 50.0 / 20.0,
            index: 0,
            nav_tabs: vec![
                ("领地".to_string(), tid1),
                ("游戏".to_string(), tid1),
                ("商店".to_string(), tid2),
                ("诸天".to_string(), tid2),
            ],
            data,
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
            // 资源显示
            if let Ok(data) = self.data.read() {
                for res in data.iter() {
                    ui.label(format!("{}: {}/{}", res.name, res.num, res.max));
                }
            }

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
