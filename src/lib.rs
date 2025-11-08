use eframe::egui;

pub struct CounterApp {
    pub value: i32,
}

impl Default for CounterApp {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl CounterApp {
    /// 统一构造函数，桌面/Android 都用它
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for CounterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Counter");
            ui.horizontal(|ui| {
                if ui.button("+1").clicked() {
                    self.value += 1;
                }
                ui.label(format!("值：{}", self.value));
            });
        });
    }
}

// ===== Android 入口 =====
#[cfg(feature = "android")]
#[unsafe(no_mangle)] // ← 新写法
extern "Rust" fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native(
        "MyApp测试",
        options,
        Box::new(|cc| Ok(Box::new(CounterApp::new(cc)))),
    )
    .unwrap();
}
