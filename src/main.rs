#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dqwm::{run_app, utils};

pub const APP_NAME: &str = "道起微末";

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default() // 创建视口构建器来配置窗口
            .with_app_id(APP_NAME)
            .with_title(APP_NAME)
            .with_inner_size([376.0, 731.4375]) // 设置窗口初始尺寸
            // .with_min_inner_size([200.0, 200.0]) // 窗口最小尺寸
            // .with_max_inner_size([500.0, 500.0]) // 非最大化时，窗口最大尺寸
            // .with_transparent(true) // 启用窗口透明
            .with_icon(utils::load_icon_png(
                include_bytes!("../assets/logo_400.png"),
                image::ImageFormat::Png,
            )) // 修改窗口/任务栏图标（详情请看上一笔记）
            .with_decorations(true), // false，无标题栏、边框
        // .with_close_button(true)     // false,禁用关闭按钮，不适用于 X11
        // .with_minimize_button(true)  // false，禁用最小化按钮，不适用于 X11
        // .with_maximize_button(true)  // false，禁用最大化按钮，不适用于 X11
        // .with_fullscreen(false)      // ture,全屏状态（无标题栏、边框）
        // .with_maximized(false)       // true,窗口最大化
        // .with_resizable(true)        // false，不能调整大小
        // .with_always_on_top() // 使窗口始终置顶
        // .with_window_level(egui::viewport::WindowLevel::AlwaysOnTop)    // 使窗口始终置顶，同上
        // .with_window_level(egui::viewport::WindowLevel::AlwaysOnBottom) // 使窗口始终置底(好像不管用)
        // .with_position(eframe::epaint::Pos2::new(100.0, 200.0)),     // 窗口在屏幕的位置，设置居中后无效
        centered: true,       // 窗口启动时在屏幕上居中显示
        ..Default::default()  // 使用其他默认选项
    };
    run_app(options);
}
