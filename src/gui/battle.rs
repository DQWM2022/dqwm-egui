use std::collections::VecDeque;

use egui::{
    Align2, Color32, FontId, Pos2, Rect, Response, Sense, StrokeKind, TextureId, Ui, Vec2,
    hex_color, pos2, vec2,
};

use crate::app::Unit;
use crate::gui::style::{
    MiddleAreaStyle, MiddleCountStyle, MiddleIconAlign, MiddleIconStyle, Side, UnitStyle,
};

fn unit_style_default(rem: f32, bg_texture_id: TextureId) -> UnitStyle {
    UnitStyle {
        width: rem,                  // 单位宽
        height: 0.8 * rem,           // 单位高
        border_width: 0.03 * rem,    // 单位边框
        atk_width: 0.5 * rem,        // 单位atk宽度
        def_width: 0.35 * rem,       // 单位def宽度
        atk_def_height: 0.22 * rem,  // 攻击防御区 高度
        name_margin_top: 0.02 * rem, // 名称区域距离顶部距离

        name_font: FontId::proportional(0.2 * rem), // 名称字体
        hp_font: FontId::proportional(0.2 * rem),   // 血条字体
        attr_font: FontId::proportional(0.16 * rem), // 属性字体

        atk_bg_color: hex_color!("#82777780"), // hex_color!("#82777780")
        def_bg_color: hex_color!("#82777780"), // hex_color!("#82777780")

        bg_texture_id, // 背景纹理id

        atk_font_icon: "\u{E681}", // 攻击字体图标 \u{E681}
        def_font_icon: "\u{E681}", // 防御字体图标 \u{E633}
    }
}
fn middle_area_style_default(rem: f32) -> MiddleAreaStyle {
    MiddleAreaStyle {
        margin: 0.1 * rem,
        enemy_count_style: MiddleCountStyle {
            icon: "\u{E6A6}", //
            icon_font: FontId::proportional(0.38 * rem),
            text_font: FontId::proportional(0.26 * rem),
            color: Color32::BLACK, // 颜色
            side: Side::Enemy,
        },
        ally_count_style: MiddleCountStyle {
            icon: "\u{E625}", //
            icon_font: FontId::proportional(0.38 * rem),
            text_font: FontId::proportional(0.26 * rem),
            color: Color32::BLACK, // 颜色
            side: Side::Ally,
        },
        run_style: MiddleIconStyle {
            icon: "\u{E64C}",
            font: FontId::proportional(0.56 * rem),
            color: Color32::BLACK,
            align: MiddleIconAlign::Left,
        },
        start_battle_style: MiddleIconStyle {
            icon: "\u{E65C}",
            font: FontId::proportional(0.56 * rem),
            color: Color32::RED,
            align: MiddleIconAlign::Center,
        },
        return_style: MiddleIconStyle {
            icon: "\u{E68D}",
            font: FontId::proportional(0.56 * rem),
            color: Color32::BLACK,
            align: MiddleIconAlign::Right,
        },
    }
}

// 战场视图
pub fn battle_view_ui(
    ui: &mut Ui,
    rem: f32,
    bg_unit_id: TextureId,
    enemy_units: &[VecDeque<Unit>],
    ally_units: &[VecDeque<Unit>],
    enemy_num: usize,
    ally_num: usize,
) -> (Response, Response, Response) {
    let unit_style = unit_style_default(rem, bg_unit_id);
    let middle_area_style = middle_area_style_default(rem);
    // 1. 申请整个可用区域（通常是 CentralPanel 的全部）
    let available_rect = ui.available_rect_before_wrap();
    let response = ui.allocate_rect(available_rect, Sense::hover());
    let p = ui.painter();

    // 分割区域
    let (top_half, middle_rect, bottom_half) = split_rect_vertically(response.rect, 0.42, 0.08);
    // 渲染背景
    p.rect_filled(response.rect, 0.0, Color32::WHITE);

    p.rect_filled(top_half, 0.0, Color32::from_rgb(255, 200, 200)); // 淡红 - 敌方区
    p.rect_filled(middle_rect, 0.0, Color32::from_rgb(200, 255, 200)); // 淡绿 - 中间区
    p.rect_filled(bottom_half, 0.0, Color32::from_rgb(200, 200, 255)); // 淡蓝 - 友方区

    // 敌方区域
    unit_grid_ui(top_half, ui, enemy_units, &unit_style, rem, Side::Enemy);
    // 我方阵型
    unit_grid_ui(bottom_half, ui, ally_units, &unit_style, rem, Side::Ally);

    middle_area_ui(ui, middle_rect, enemy_num, ally_num, &middle_area_style)
}

// 中间区域
fn middle_area_ui(
    ui: &Ui,
    rect: Rect,
    enemy_num: usize,
    ally_num: usize,
    style: &MiddleAreaStyle,
) -> (Response, Response, Response) {
    // 应用 margin：缩小 rect
    let inner_rect = Rect::from_min_size(
        rect.min + Vec2::splat(style.margin),
        (rect.size() - Vec2::splat(2.0 * style.margin)).max(Vec2::ZERO),
    );

    // 左侧敌方我方人数
    middle_count_ui(inner_rect, ui, enemy_num, &style.enemy_count_style);
    //左侧我方人数
    middle_count_ui(inner_rect, ui, ally_num, &style.ally_count_style);
    (
        middle_icon_ui(inner_rect, ui, &style.run_style),
        middle_icon_ui(inner_rect, ui, &style.start_battle_style),
        middle_icon_ui(inner_rect, ui, &style.return_style),
    )
}
fn middle_icon_ui(rect: Rect, ui: &Ui, style: &MiddleIconStyle) -> Response {
    // 计算文本/图标的大小
    let icon_size = measure_text_size(ui, style.icon, style.font.clone(), style.color);

    let icon_rect = match style.align {
        MiddleIconAlign::Left => Rect::from_min_max(rect.min, pos2(rect.center().x, rect.max.y)),
        MiddleIconAlign::Center => Rect::from_center_size(rect.center(), icon_size),
        MiddleIconAlign::Right => Rect::from_min_max(pos2(rect.center().x, rect.min.y), rect.max),
    };

    // 创建图标应当放置的矩形区域，并确保居中
    let icon_rect = egui::Rect::from_center_size(icon_rect.center(), icon_size);

    // 绘制图标
    ui.painter().text(
        icon_rect.center(), // 使用icon_rect的中心点作为文本绘制起点
        egui::Align2::CENTER_CENTER,
        style.icon,
        style.font.clone(),
        style.color,
    );

    ui.interact(icon_rect, ui.id().with(style.icon), egui::Sense::click())
}

fn middle_count_ui(rect: Rect, ui: &Ui, num: usize, style: &MiddleCountStyle) {
    // 测量图标尺寸
    let icon_size = measure_text_size(ui, &num.to_string(), style.icon_font.clone(), style.color);

    // 图标位置
    let icon_x = rect.left();
    let icon_y_center = match style.side {
        Side::Enemy => rect.top() + icon_size.y / 2.0,
        Side::Ally => rect.bottom() - icon_size.y / 2.0,
    };

    ui.painter().text(
        pos2(icon_x, icon_y_center),
        Align2::LEFT_CENTER,
        style.icon,
        style.icon_font.clone(),
        style.color,
    );

    // 绘制数字（左对齐）
    ui.painter().text(
        pos2(icon_x + icon_size.x, icon_y_center),
        Align2::LEFT_CENTER,
        num,
        style.text_font.clone(),
        Color32::BLACK,
    );
}

// 渲染 单位阵列
fn unit_grid_ui(
    rect: Rect,
    ui: &Ui,
    units: &[VecDeque<Unit>],
    style: &UnitStyle,
    rem: f32,
    side: Side,
) {
    if units.is_empty() {
        return;
    }

    let num_cols = units.len();
    let cell_width = (rect.width() / num_cols as f32).clamp(rem, 2.0 * rem);
    let start_x = rect.min.x + (rect.width() - cell_width * num_cols as f32) / 2.0;
    let cell_height = 1.2 * style.width;

    for (col_idx, column) in units.iter().enumerate() {
        let x = start_x + col_idx as f32 * cell_width;

        // 直接渲染所有收到的单位（后端已裁剪）
        for (row_idx, unit) in column.iter().enumerate() {
            let unit_rect = match side {
                Side::Enemy => {
                    let y_bottom = rect.max.y - row_idx as f32 * cell_height;
                    Rect::from_min_max(
                        pos2(x, y_bottom - cell_height),
                        pos2(x + cell_width, y_bottom),
                    )
                }
                Side::Ally => {
                    let y_top = rect.min.y + row_idx as f32 * cell_height;
                    Rect::from_min_size(pos2(x, y_top), vec2(cell_width, cell_height))
                }
            };

            unit_ui(style, unit_rect, unit, ui);
        }
    }
}

// 渲染单位
fn unit_ui(style: &UnitStyle, rect: Rect, unit: &Unit, ui: &Ui) {
    let p = ui.painter();
    // 1. 计算实际绘制区域（居中）
    let rect = Rect::from_center_size(rect.center(), vec2(style.width, style.height));
    // 2. 绘制阴影，通过绘制背景纹理实现，图片的左上角+ 阴影偏移量 才是矩形的左上角
    p.image(
        style.bg_texture_id,
        rect.scale_from_center(1.2), // 宽高各放大 1.2 倍（+10% 边距）,
        egui::Rect::from_min_max(Pos2::ZERO, egui::pos2(1.0, 1.0)), // UV坐标
        Color32::WHITE,              // 色调（可以用来调暗或着色）
    );
    // 3.血条背景
    let bg_rect = Rect::from_min_size(
        rect.min, // 使用原始rect的左上角坐标作为起点
        vec2(
            rect.width() * (unit.hp as f32 / unit.max_hp as f32),
            rect.height(),
        ), // 设置新宽度和原始高度
    );
    p.rect_filled(bg_rect, 0, Color32::RED);
    // 4. 边框
    p.rect_stroke(
        rect,
        0.0,
        egui::Stroke {
            width: style.border_width,
            color: Color32::BLACK,
        },
        StrokeKind::Inside,
    );

    // 5.名称
    p.text(
        rect.center_top() + vec2(0.0, style.border_width + style.name_margin_top),
        Align2::CENTER_TOP,
        unit.name,
        style.name_font.clone(),
        Color32::BLACK,
    );

    // 6.生命值

    p.text(
        rect.center(),
        Align2::CENTER_CENTER,
        format!("{}/{}", unit.hp, unit.max_hp),
        style.hp_font.clone(),
        Color32::BLACK,
    );

    // 7.攻击力背景矩形以及文本

    let atk_rect = Rect::from_min_max(
        pos2(
            rect.left() + style.border_width,
            rect.bottom() - style.atk_def_height,
        ),
        pos2(rect.left() + style.atk_width, rect.bottom()),
    );

    p.rect_filled(atk_rect, 0.0, style.atk_bg_color);
    // 绘制攻击力文本
    p.text(
        (
            rect.left() - style.border_width,
            rect.bottom() - style.border_width,
        )
            .into(),
        Align2::LEFT_BOTTOM,
        format_args!("{}{}", style.atk_font_icon, unit.atk),
        style.attr_font.clone(),
        Color32::WHITE,
    );

    // 8. 绘制 防御力背景矩形以及文本
    let def_rect = Rect::from_min_max(
        pos2(
            rect.right() - style.def_width,
            rect.bottom() - style.atk_def_height,
        ),
        pos2(rect.right(), rect.bottom()),
    );
    p.rect_filled(def_rect, 0.0, style.def_bg_color);
    p.text(
        (
            rect.right() + style.border_width,
            rect.bottom() - style.border_width,
        )
            .into(),
        Align2::RIGHT_BOTTOM,
        format_args!("{}{}", unit.def, style.def_font_icon),
        style.attr_font.clone(),
        Color32::WHITE,
    );
}

// 切分矩形
fn split_rect_vertically(
    rect: Rect,
    top_bottom_ratio: f32,
    middle_ratio: f32,
) -> (Rect, Rect, Rect) {
    let total_h = rect.height();
    let y0 = rect.min.y;
    let y1 = y0 + top_bottom_ratio * total_h;
    let y2 = y0 + (top_bottom_ratio + middle_ratio) * total_h;

    let top = Rect::from_min_max(rect.min, pos2(rect.max.x, y1));
    let middle = Rect::from_min_max(pos2(rect.min.x, y1), pos2(rect.max.x, y2));
    let bottom = Rect::from_min_max(pos2(rect.min.x, y2), rect.max);

    (top, middle, bottom)
}

#[inline]
fn measure_text_size(ui: &egui::Ui, text: &str, font: egui::FontId, color: egui::Color32) -> Vec2 {
    ui.fonts_mut(|f| f.layout_no_wrap(text.to_owned(), font, color))
        .size()
}
