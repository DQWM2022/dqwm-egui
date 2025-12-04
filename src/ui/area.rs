use egui::{
    Align2, Color32, FontId, Pos2, Rect, Response, Sense, StrokeKind, Ui, Vec2, hex_color, pos2,
    vec2,
};

use crate::app::Unit;

// 战斗视图
#[derive(Clone)]
pub struct QView {}
// 战斗区域
pub fn draw_area(
    ui: &mut Ui,
    rem: f32, // 设计时字体大小
    bg_unit: &egui::TextureHandle,
    units: &[Vec<Unit>],    // 列数据
    reverse_vertical: bool, // 是否垂直反转（bottom-up）
) -> Response {
    if units.is_empty() {
        return ui.allocate_response(Vec2::ZERO, Sense::hover());
    }
    let width = ui.available_width(); // 可用宽度
    let height = 0.45 * ui.ctx().viewport_rect().height(); // 高度限制在屏幕一半以内
    let response = ui.allocate_response(vec2(width, height), Sense::hover());

    // 计算起始点
    let base_x = response.rect.min.x;
    let base_y = if reverse_vertical {
        response.rect.max.y // 底部起点
    } else {
        response.rect.min.y
    };

    // 计算 宽度 和 高度
    let cell_width = (width / units.len() as f32).clamp(rem, 2.0 * rem);
    let cell_height = 1.2 * rem * 0.8;

    // 绘制每个单元格
    for (col_idx, column) in units.iter().enumerate() {
        for (row_idx, unit) in column.iter().enumerate() {
            // 计算单元格矩形
            let x = base_x + col_idx as f32 * cell_width;

            let rect = if reverse_vertical {
                // 从底部开始向上绘制
                // 注意：y 是矩形底部的位置，所以要减去高度
                let y_bottom = base_y - row_idx as f32 * cell_height;
                Rect::from_min_max(
                    pos2(x, y_bottom - cell_height), // 左上角
                    pos2(x + cell_width, y_bottom),  // 右下角
                )
            } else {
                // 从顶部开始向下绘制
                let y_top = base_y + row_idx as f32 * cell_height;
                Rect::from_min_size(pos2(x, y_top), vec2(cell_width, cell_height))
            };
            draw(unit, rem, rect, bg_unit, ui);
        }
    }

    response
}

pub fn draw(unit: &Unit, rem: f32, rect: Rect, bg_unit: &egui::TextureHandle, ui: &mut Ui) {
    // 宽高比
    let aspect_ratio = 0.8;
    // 从全局数据中获取设计时字体大小
    let size = Vec2::new(rem, aspect_ratio * rem);

    // 居中矩形
    let rect = Rect::from_center_size(rect.center(), size);

    let painter = ui.painter();

    // === 1. 自适应字体大小 ===
    let name_font = FontId::proportional(0.2 * rem);
    let hp_font = FontId::proportional(0.2 * rem);
    let attr_font = FontId::proportional(0.16 * rem);

    // 绘制阴影，通过绘制背景纹理实现，图片的左上角+ 阴影偏移量 才是矩形的左上角
    painter.image(
        bg_unit.id(),
        rect.scale_from_center(1.2), // 宽高各放大 1.2 倍（+10% 边距）,
        egui::Rect::from_min_max(Pos2::ZERO, egui::pos2(1.0, 1.0)), // UV坐标
        Color32::WHITE,              // 色调（可以用来调暗或着色）
    );

    // 计算血条背景的矩形
    let bg_rect = Rect::from_min_size(
        rect.min, // 使用原始rect的左上角坐标作为起点
        vec2(
            rect.width() * (unit.hp as f32 / unit.max_hp as f32),
            rect.height(),
        ), // 设置新宽度和原始高度
    );

    // 绘制背景（可选，方便看到效果）
    painter.rect_filled(bg_rect, 0, Color32::RED);

    // 边框宽度
    let border_width = 0.03 * rem;

    // 绘制名称
    painter.text(
        rect.center_top() + vec2(0.0, border_width + 0.02 * rem),
        Align2::CENTER_TOP,
        &unit.name,
        name_font,
        Color32::BLACK,
    );
    // 绘制当前血量/最大血量文本
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        format!("{}/{}", unit.hp, unit.max_hp),
        hp_font.clone(),
        Color32::BLACK,
    );

    // 绘制边框
    painter.rect_stroke(
        rect,
        0.0,
        egui::Stroke {
            width: border_width,
            color: Color32::BLACK,
        },
        StrokeKind::Inside,
    );

    // 绘制攻击力背景矩形
    let atk_rect = Rect::from_min_size(
        rect.min + (0., 0.77 * rem * aspect_ratio - border_width).into(),
        vec2(0.5 * rem, 0.22 * rem),
    );
    painter.rect_filled(atk_rect, 0.0, hex_color!("#82777780"));
    // 绘制攻击力文本
    painter.text(
        (rect.left() - 0.03 * rem, rect.bottom() - 0.03 * rem).into(),
        egui::Align2::LEFT_BOTTOM,
        format!("\u{E681}{}", unit.atk),
        attr_font.clone(),
        Color32::WHITE,
    );

    // 绘制防御力背景矩形以及文本（如果防御力大于0）
    if unit.def > 0 {
        let def_rect = Rect::from_min_size(
            rect.min + (0.60 * rem, 0.77 * rem * aspect_ratio - border_width).into(),
            vec2(0.35 * rem, 0.22 * rem),
        );
        painter.rect_filled(def_rect, 0.0, hex_color!("#82777780"));
        painter.text(
            (rect.right() + 0.03 * rem, rect.bottom() - 0.03 * rem).into(),
            egui::Align2::RIGHT_BOTTOM,
            format!("{}\u{E633}", unit.def),
            attr_font.clone(),
            Color32::WHITE,
        );
    }
}
