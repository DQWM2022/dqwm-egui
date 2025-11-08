use dqwm::CounterApp;

fn main() -> eframe::Result {
    env_logger::init(); // 桌面端日志
    eframe::run_native(
        "CounterApp",
        Default::default(),
        Box::new(|cc| Ok(Box::new(CounterApp::new(cc)))),
    )
}
