use std::time::{Duration, Instant};

use crate::{R, UiExt, model::Unit};

use egui::{
    Color32, FontId, Id, Layout, Rect, Response, RichText, Sense, Stroke, StrokeKind, Ui,
    UiBuilder, Vec2, pos2, vec2,
};
use egui_extras::{Size, StripBuilder};

pub enum ArmyType {
    Ally,
    Enemy,
}
fn with_alpha(color: Color32, alpha: f32) -> Color32 {
    // 将 [0.0, 1.0] 映射到 [0, 255]
    let a_u8 = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), a_u8)
}

fn _build_name(ui: &mut Ui, name: &str) {
    ui.centered_and_justified(|ui| {
        ui.label(RichText::new(name).font(FontId::proportional(ui.rem(0.2))));
    });
}
fn _build_hp(ui: &mut Ui, hp: u128, max_hp: u128) {
    ui.centered_and_justified(|ui| {
        ui.label(RichText::new(format!("{hp}/{max_hp}")).font(FontId::proportional(ui.rem(0.2))));
    });
}
fn _build_atk(ui: &mut Ui, atk: u128) {
    ui.with_layout(Layout::left_to_right(egui::Align::Max), |ui| {
        ui.label(
            RichText::new(format!("\u{E681}{atk}"))
                .font(FontId::proportional(ui.rem(0.16)))
                .color(Color32::WHITE),
        );
    });
}
fn _build_def(ui: &mut Ui, def: u128) {
    ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
        ui.label(
            RichText::new(format!("{def}\u{E633}"))
                .font(FontId::proportional(ui.rem(0.16)))
                .color(Color32::WHITE),
        );
    });
}

fn _shadow(ui: &mut Ui, rect: Rect) {
    ui.painter().image(
        ui.get_texture_id(R::UnitShadow),
        rect.scale_from_center(2.0),
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
}
fn _border(ui: &mut Ui, rect: Rect, border_width: f32) {
    ui.painter().rect_stroke(
        rect,
        0.0,
        Stroke {
            width: border_width,
            color: Color32::BLACK,
        },
        StrokeKind::Inside,
    );
}
fn _bg_hp(ui: &mut Ui, rect: Rect, ratio: f32, opacity: f32) {
    let bg_rect = Rect::from_min_size(rect.min, vec2(rect.width() * ratio, rect.height()));

    ui.painter()
        .rect_filled(bg_rect, 0.0, with_alpha(Color32::RED, opacity));
}

fn _anim_atk(ui: &mut Ui, rect: &mut Rect, id: usize, time: Option<Instant>, army_type: ArmyType) {
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

fn _anim_def(ui: &mut Ui, id: usize, trigger: Option<Instant>) -> f32 {
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

fn _draw_damage_text(
    ui: &mut Ui,
    rect: Rect,
    trigger: Option<Instant>,
    id: usize,
    army_type: ArmyType,
) {
    const TEXT: &str = "-9999";
    const DURATION: Duration = Duration::from_millis(500);
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

pub fn render(ui: &mut Ui, unit: &Unit) -> Response {
    let w = ui.rem(1.0);
    let h = ui.rem(0.8);
    let border_width = ui.rem(0.03);
    ui.spacing_mut().item_spacing = Vec2::ZERO;
    let (mut rect, response) = ui.allocate_exact_size(vec2(w, h), Sense::click());
    // 动画
    let mut time = Option::None;
    if response.clicked() {
        time = Some(Instant::now());
    }
    _anim_atk(ui, &mut rect, unit.id, time, ArmyType::Ally);
    let opacity = _anim_def(ui, unit.id, time);

    let mut child_ui = ui.new_child(UiBuilder::new().max_rect(rect.shrink(border_width)));
    _shadow(ui, rect); //阴影
    _bg_hp(ui, rect, (unit.hp / unit.max_hp) as f32, opacity); //背景
    _border(ui, rect, border_width); // 绘制边框
    StripBuilder::new(&mut child_ui)
        .size(Size::relative(0.030))
        .size(Size::relative(0.335))
        .size(Size::relative(0.335))
        .size(Size::relative(0.060))
        .size(Size::relative(0.240))
        .vertical(|mut strip| {
            strip.empty();
            strip.cell(|ui| _build_name(ui, unit.name));
            strip.cell(|ui| _build_hp(ui, unit.hp, unit.max_hp));
            strip.empty();
            strip.cell(|ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.5))
                    .size(Size::relative(0.15))
                    .size(Size::relative(0.35))
                    .horizontal(|mut strip| {
                        strip.cell(|ui| _build_atk(ui, unit.atk));
                        strip.empty();
                        strip.cell(|ui| _build_def(ui, unit.atk));
                    });
            });
        });
    _draw_damage_text(ui, rect, time, unit.id, ArmyType::Enemy);
    response
}
