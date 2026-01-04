// Unit渲染方法

use std::time::{Duration, Instant};

use egui::{
    Align2, Color32, FontId, Id, Rect, Response, Sense, Shadow, Stroke, StrokeKind, Ui, hex_color,
    pos2, vec2,
};

use crate::{UiExt, model::Unit};

// 宏用于计算累计高度
macro_rules! sum_arr {
    ($arr:expr, $n:expr) => {
        $arr.iter().take($n).sum::<f32>()
    };
}

#[derive(Debug, Clone, Copy)]
pub enum ArmyType {
    Ally,
    Enemy,
}

pub fn render(ui: &mut Ui, cell_rect: &mut Rect, unit: &Unit, army_type: ArmyType) -> Response {
    let w = ui.rem(1.0);
    let h = ui.rem(0.8);
    let border_width = ui.rem(0.03);
    let shadow_blur = ui.rem(0.16) as u8;
    let name_font = FontId::proportional(ui.rem(0.20));
    let hp_font = FontId::proportional(ui.rem(0.16));
    let attr_font = FontId::proportional(ui.rem(0.16));

    let mut rect = Rect::from_center_size(cell_rect.center(), egui::vec2(w, h));
    let response = ui.allocate_rect(rect, Sense::click());

    // let (mut rect, response) = ui.allocate_exact_size(vec2(w, h), Sense::click());

    // 动画
    let mut time = Option::None;
    if response.clicked() {
        time = Some(Instant::now());
    }
    anim_atk(ui, &mut rect, unit.id, time, army_type);
    let opacity = anim_def(ui, unit.id, time);

    let p = ui.painter();
    // 1、阴影
    let shadow = Shadow {
        offset: [0, 0],
        blur: shadow_blur,
        spread: 0,
        color: Color32::RED,
    };
    p.add(shadow.as_shape(rect, 0.0));

    // 2、背景
    p.rect_filled(rect, 0.0, Color32::WHITE);
    let hp_ratio = (unit.hp / unit.max_hp) as f32;
    p.rect_filled(
        Rect::from_min_size(rect.min, vec2(w * hp_ratio, h)),
        0.0,
        with_alpha(Color32::RED, opacity),
    );

    // 3、边框
    p.rect_stroke(
        rect,
        0.0,
        Stroke {
            width: border_width,
            color: Color32::BLACK,
        },
        StrokeKind::Inside,
    );
    let rect = rect.shrink(border_width);
    let arr_y = [0.03, 0.34, 0.34, 0.05, 0.24];
    let w = rect.width();
    let h = rect.height();

    let name_rect = Rect::from_min_size(
        pos2(rect.min.x, rect.min.y + sum_arr!(arr_y, 1) * h),
        vec2(w, arr_y[1] * h),
    );

    let hp_rect = Rect::from_min_size(
        pos2(rect.min.x, rect.min.y + sum_arr!(arr_y, 2) * h),
        vec2(w, arr_y[2] * h),
    );
    let attr_h = rect.min.y + sum_arr!(arr_y, 4) * h;
    let atk_rect = Rect::from_min_size(pos2(rect.min.x, attr_h), vec2(w * 0.5, arr_y[4] * h));
    let def_rect = Rect::from_min_size(
        pos2(rect.min.x + 0.65 * w, attr_h),
        vec2(w * 0.35, arr_y[4] * h),
    );
    // 4、名称
    p.text(
        name_rect.center_top(),
        Align2::CENTER_TOP,
        unit.name,
        name_font,
        Color32::BLACK,
    );
    // 5、血量
    p.text(
        hp_rect.center(),
        Align2::CENTER_CENTER,
        format!("{}/{}", unit.hp, unit.max_hp),
        hp_font,
        Color32::BLACK,
    );
    // 5、攻击
    p.rect_filled(atk_rect, 0.0, hex_color!("#82777780"));
    p.text(
        atk_rect.left_bottom(),
        Align2::LEFT_BOTTOM,
        format!("\u{E681}{}", unit.atk),
        attr_font.clone(),
        Color32::WHITE,
    );

    // 6、防御
    p.rect_filled(def_rect, 0.0, hex_color!("#82777780"));
    p.text(
        def_rect.right_bottom(),
        Align2::RIGHT_BOTTOM,
        format!("{}\u{E633}", unit.def),
        attr_font.clone(),
        Color32::WHITE,
    );

    anim_text(ui, rect, time, unit.id, army_type);

    response
}

fn anim_atk(ui: &Ui, rect: &mut Rect, id: usize, time: Option<Instant>, army_type: ArmyType) {
    const DURATION: Duration = Duration::from_millis(400);
    let ctx = ui.ctx();
    let height = rect.height();
    let now = Instant::now();
    let anim_id = Id::new(("ATK", id));
    let dir = match army_type {
        ArmyType::Ally => -1.0,
        ArmyType::Enemy => 1.0,
    };
    let (offset_y, needs_repaint) = ctx.memory_mut(|mem| {
        if let Some(start_time) = time {
            mem.data.insert_temp(anim_id, start_time);
        }
        let Some(start_time) = mem.data.get_temp::<Instant>(anim_id) else {
            return (0.0, false);
        };
        let elapsed = now.duration_since(start_time);
        if elapsed >= DURATION {
            mem.data.remove::<Instant>(anim_id);
            return (0.0, false);
        }

        let t = elapsed.as_secs_f32() / DURATION.as_secs_f32();
        let seg_t = if t < 0.5 { t * 2.0 } else { (t - 0.5) * 2.0 };
        let max_off = 0.6 * height;

        let mag = if seg_t <= 0.5 {
            (1.0 - (1.0 - seg_t * 2.0).powi(3)) * max_off
        } else {
            ((1.0 - (seg_t - 0.5) * 2.0).powi(3)) * max_off
        };

        (dir * mag, true)
    });

    if needs_repaint {
        ctx.request_repaint();
    }

    rect.min.y += offset_y;
    rect.max.y += offset_y;
}

fn anim_def(ui: &Ui, id: usize, trigger: Option<Instant>) -> f32 {
    const DURATION: Duration = Duration::from_millis(300);
    let ctx = ui.ctx();
    let anim_id = Id::new(("DEF", id));

    let (opacity, needs_repaint) = ctx.memory_mut(|mem| {
        if let Some(now) = trigger {
            mem.data.insert_temp(anim_id, now);
        }

        let Some(start_time) = mem.data.get_temp::<Instant>(anim_id) else {
            return (1.0, false);
        };

        let elapsed = start_time.elapsed();
        if elapsed >= DURATION {
            mem.data.remove::<Instant>(anim_id);
            return (1.0, false);
        }

        let opacity = (elapsed.as_secs_f32() / DURATION.as_secs_f32()).clamp(0.0, 1.0);
        (opacity, true) // 需要重绘
    });

    if needs_repaint {
        ctx.request_repaint(); // ← 安全：在闭包外调用
    }

    opacity
}

fn anim_text(ui: &Ui, rect: Rect, trigger: Option<Instant>, id: usize, army_type: ArmyType) {
    const TEXT: &str = "-9999";
    const DURATION: Duration = Duration::from_millis(1000);
    let ctx = ui.ctx();
    let anim_id = Id::new(("DAMAGE_POPUP", id));

    // 判断左右（用于水平飘动方向）
    let screen_center_x = ctx.viewport_rect().center().x;
    let is_on_left = rect.center().x < screen_center_x;

    // === 状态更新（纯数据）===
    let state = ctx.memory_mut(|mem| {
        if let Some(start_time) = trigger {
            mem.data.insert_temp(anim_id, (start_time, rect));
        }

        let (start_time, base_rect) = mem.data.get_temp::<(Instant, Rect)>(anim_id)?;

        let elapsed = start_time.elapsed();
        if elapsed >= DURATION {
            mem.data.remove::<(Instant, Rect)>(anim_id);
            return None;
        }

        Some((start_time, base_rect))
    });

    // === 绘制（闭包外）===
    if let Some((start_time, base_rect)) = state {
        let elapsed = start_time.elapsed();
        let t = (elapsed.as_secs_f32() / DURATION.as_secs_f32()).clamp(0.0, 1.0);

        // 垂直偏移：根据 army_type 决定向上 or 向下
        let y_offset = match army_type {
            ArmyType::Ally => -40.0 * t, // 上方冒出，向上飘
            ArmyType::Enemy => 40.0 * t, // 下方冒出，向下飘
        };

        // 水平偏移：左→右，右→左
        let x_offset = if is_on_left { 30.0 } else { -30.0 } * t;

        // 起始锚点：Ally 用 top，Enemy 用 bottom
        let anchor_y = match army_type {
            ArmyType::Ally => base_rect.top(),
            ArmyType::Enemy => base_rect.bottom(),
        };

        let pos = pos2(base_rect.center().x + x_offset, anchor_y + y_offset);

        let alpha = ((1.0 - t) * 255.0).clamp(0.0, 255.0) as u8;
        let color = Color32::from_rgba_unmultiplied(255, 50, 50, alpha); // 红色

        ui.painter().text(
            pos,
            egui::Align2::CENTER_CENTER,
            TEXT,
            FontId::monospace(14.0),
            color,
        );

        ctx.request_repaint();
    }
}

#[inline]
fn with_alpha(color: Color32, alpha: f32) -> Color32 {
    // 将 [0.0, 1.0] 映射到 [0, 255]
    let a_u8 = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), a_u8)
}
