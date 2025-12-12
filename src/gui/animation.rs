use egui::{Rect, Response, Ui};

#[derive(Copy, Clone, Debug)]
pub enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct MoveCfg {
    pub total_duration_ms: f32,
    pub amplitude_pct: f32,
    pub repeat_count: usize,
    pub direction: MoveDir,
}

impl Default for MoveCfg {
    fn default() -> Self {
        Self {
            total_duration_ms: 600.0,
            amplitude_pct: 0.5,
            repeat_count: 2,
            direction: MoveDir::Up,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct MoveState {
    start_time: f64,
}

impl Default for MoveState {
    fn default() -> Self {
        Self { start_time: 0.0 }
    }
}

pub fn animate_rect(ui: &Ui, resp: &Response, rect: &mut Rect, cfg: MoveCfg) {
    let id = resp.id.with("move_anim");
    let ctx = ui.ctx();
    let now = ui.input(|i| i.time);

    let mut state: Option<MoveState> = ctx.memory(|mem| mem.data.get_temp(id));

    if resp.clicked() && state.is_none() {
        state = Some(MoveState { start_time: now });
        ctx.request_repaint();
    }

    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    if let Some(s) = state {
        let elapsed_sec = (now - s.start_time) as f32;
        let elapsed_ms = elapsed_sec * 1000.0;

        if elapsed_ms < cfg.total_duration_ms {
            ctx.request_repaint();

            let total_segments = cfg.repeat_count * 2; // 每次 = 移出 + 归位
            let segment_duration = cfg.total_duration_ms / total_segments as f32;
            let current_segment = (elapsed_ms / segment_duration) as usize;
            let seg_progress = (elapsed_ms % segment_duration) / segment_duration;

            // 计算偏移量（正值）
            let offset_amount = match cfg.direction {
                MoveDir::Up | MoveDir::Down => rect.height() * cfg.amplitude_pct,
                MoveDir::Left | MoveDir::Right => rect.width() * cfg.amplitude_pct,
            };

            // 当前是“移出”还是“归位”？
            let displacement = if current_segment.is_multiple_of(2) {
                offset_amount * seg_progress // 移出：0 → full
            } else {
                offset_amount * (1.0 - seg_progress) // 归位：full → 0
            };

            // 应用方向符号
            match cfg.direction {
                MoveDir::Up => offset_y = -displacement,
                MoveDir::Down => offset_y = displacement,
                MoveDir::Left => offset_x = -displacement,
                MoveDir::Right => offset_x = displacement,
            }
        } else {
            state = None;
        }

        ctx.memory_mut(|mem| {
            if let Some(st) = state {
                mem.data.insert_temp(id, st);
            } else {
                let _ = mem.data.remove_temp::<MoveState>(id);
            }
        });

        // 应用最终偏移
        rect.min.x += offset_x;
        rect.max.x += offset_x;
        rect.min.y += offset_y;
        rect.max.y += offset_y;
    }
}

/// 自动位移动画，由外部 bool 控制是否启动新动画
/// - should_start = true 且当前无动画 → 启动一次
/// - should_start = false → 不启动，但已有动画继续播
pub fn animate_rect_auto_switch(ui: &Ui, rect: &mut Rect, cfg: MoveCfg, should_start: bool) {
    let id = ui.auto_id_with("move_anim");
    let ctx = ui.ctx();
    let now = ui.input(|i| i.time);

    // 从 egui memory 读取当前动画状态
    let mut state: Option<MoveState> = ctx.memory(|mem| mem.data.get_temp(id));

    // 如果外部要求启动，且当前没有动画，则启动
    if should_start && state.is_none() {
        state = Some(MoveState { start_time: now });
        ctx.request_repaint();
    }

    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    if let Some(s) = state {
        let elapsed_sec = (now - s.start_time) as f32;
        let elapsed_ms = elapsed_sec * 1000.0;

        if elapsed_ms < cfg.total_duration_ms {
            ctx.request_repaint();

            let total_segments = cfg.repeat_count * 2;
            let segment_duration = cfg.total_duration_ms / total_segments as f32;
            let current_segment = (elapsed_ms / segment_duration) as usize;
            let seg_progress = (elapsed_ms % segment_duration) / segment_duration;

            let offset_amount = match cfg.direction {
                MoveDir::Up | MoveDir::Down => rect.height() * cfg.amplitude_pct,
                MoveDir::Left | MoveDir::Right => rect.width() * cfg.amplitude_pct,
            };

            let displacement = if current_segment.is_multiple_of(2) {
                offset_amount * seg_progress
            } else {
                offset_amount * (1.0 - seg_progress)
            };

            match cfg.direction {
                MoveDir::Up => offset_y = -displacement,
                MoveDir::Down => offset_y = displacement,
                MoveDir::Left => offset_x = -displacement,
                MoveDir::Right => offset_x = displacement,
            }
        } else {
            // 动画结束，清理状态
            state = None;
        }

        // 写回状态（结束时会移除）
        ctx.memory_mut(|mem| {
            if let Some(st) = state {
                mem.data.insert_temp(id, st);
            } else {
                let _ = mem.data.remove_temp::<MoveState>(id);
            }
        });

        // 应用偏移
        rect.min.x += offset_x;
        rect.max.x += offset_x;
        rect.min.y += offset_y;
        rect.max.y += offset_y;
    }
}
