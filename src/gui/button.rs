use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, StrokeKind, TextureId, Ui, Widget};

/// 效果入参
#[derive(Copy, Clone, Debug)]
pub struct ShakeCfg {
    /// 总时长（毫秒）
    pub duration_ms: f32,
    /// 振幅（相对于矩形高度的百分比，0.01 = 1%）
    pub amplitude_pct: f32,
}

impl Default for ShakeCfg {
    fn default() -> Self {
        Self {
            duration_ms: 80.0,
            amplitude_pct: 0.10, // 10%
        }
    }
}

/// 运行时状态
#[derive(Copy, Clone, Debug)]
struct ShakeState {
    remain_ms: f32,
}

/// 纯函数：把 rect 按抖动参数就地修改
pub fn animate_rect(ui: &Ui, resp: &Response, rect: &mut egui::Rect, cfg: ShakeCfg) {
    let id = resp.id.with("shake");
    let mut state: ShakeState = ui.ctx().memory_mut(|mem| {
        mem.data
            .get_temp(id)
            .unwrap_or(ShakeState { remain_ms: 0.0 })
    });

    // 点击触发
    if resp.clicked() && state.remain_ms <= 0.0 {
        state.remain_ms = cfg.duration_ms;
    }

    // 更新计时
    let dt_ms = ui.ctx().input(|i| i.unstable_dt * 1000.0).min(16.0);
    state.remain_ms = (state.remain_ms - dt_ms).max(0.0);
    if state.remain_ms > 0.0 {
        ui.ctx().request_repaint();
    }

    // 计算偏移（百分比 -> 像素）
    let off = if state.remain_ms > 0.0 {
        let t = 1.0 - state.remain_ms / cfg.duration_ms; // 0..1
        (t * std::f32::consts::PI).sin() * rect.height() * cfg.amplitude_pct
    } else {
        0.0
    };

    // 写回
    ui.ctx().memory_mut(|mem| mem.data.insert_temp(id, state));

    // 就地改 rect
    rect.min.y += off;
    rect.max.y += off;
}

/* ---------- 按钮控件 ---------- */
#[derive(Clone)]
pub struct QButton {
    size: egui::Vec2,
    id: TextureId,
    shake: ShakeCfg,
    text: String,
}

impl QButton {
    pub fn new(text: &str, id: TextureId, size: egui::Vec2) -> Self {
        Self {
            id,
            size,
            text: text.to_string(),
            shake: ShakeCfg::default(),
        }
    }

    /// 链式设置抖动参数
    pub fn shake_cfg(mut self, cfg: ShakeCfg) -> Self {
        self.shake = cfg;
        self
    }
}

impl Widget for QButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            id,
            size,
            shake,
            text,
        } = self;
        let response = ui.allocate_response(size, Sense::click());

        let mut rect = response.rect;
        animate_rect(ui, &response, &mut rect, shake); // ← 就这一行

        // 绘制背景纹理
        ui.painter().image(
            id,
            rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
        //
        if response.hovered() {
            ui.painter().rect_stroke(
                rect,
                0.0,
                Stroke::new(1.0, ui.style().visuals.widgets.hovered.fg_stroke.color),
                StrokeKind::Inside,
            );
        }

        // 绘制文字
        let text_color = ui.style().visuals.text_color();
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(14.0),
            text_color,
        );

        response
    }
}
