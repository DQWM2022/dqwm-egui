use std::collections::VecDeque;

use egui::{Align2, Color32, FontId, Rect, Response, Sense, Ui, Vec2, pos2, vec2};

use crate::{
    UiExt,
    components::unit_ui::{self, ArmyType},
    core::batttle::{ArmySnapshot, BattleEvent},
    model::Unit,
};

pub fn render(
    ui: &mut Ui,
    army: &ArmySnapshot,
    events: &VecDeque<BattleEvent>,
) -> (Response, Response, Response) {
    ui.spacing_mut().item_spacing = Vec2::ZERO;
    let rect = ui.available_rect_before_wrap();
    let h = rect.height();
    let _h = ui.rem(1.2); // 单元格高度
    let (top_rect, middle_rect, bottom_rect) = split_rect_vertically(rect, 0.42, 0.08);

    let count1 = (h * 0.42 / _h) as usize + 1;
    let count2 = (h * 0.50 / _h) as usize + 1;
    // 敌方区域
    unit_grid_ui(ui, top_rect, &army.enemys, count1, ArmyType::Enemy, &events);
    // 我方阵型
    unit_grid_ui(
        ui,
        bottom_rect,
        &army.allys,
        count2,
        ArmyType::Ally,
        &events,
    );

    ui.allocate_rect(rect, Sense::hover()); // 手动分配占满
    middle_ui(ui, middle_rect)
}

// 渲染 单位网格
fn unit_grid_ui(
    ui: &mut Ui,
    rect: Rect,
    units2: &[VecDeque<Unit>],
    max_count: usize,
    army_type: ArmyType,
    events: &VecDeque<BattleEvent>,
) {
    if units2.is_empty() {
        return;
    }
    let unit_width = ui.rem(1.0);
    let unit_height = ui.rem(0.8);
    let num_cols = units2.len(); // 列数
    let cell_width = (rect.width() / num_cols as f32).clamp(unit_width, 2.0 * unit_height);
    let cell_height = 1.2 * unit_width;
    let start_x = rect.min.x + (rect.width() - cell_width * num_cols as f32) / 2.0;

    for (col_idx, column) in units2.iter().enumerate() {
        let x = start_x + col_idx as f32 * cell_width;

        // 直接渲染所有收到的单位（后端已裁剪）
        for (row_idx, unit) in column.iter().take(max_count).enumerate() {
            let mut unit_rect = match army_type {
                ArmyType::Enemy => {
                    let y_bottom = rect.max.y - row_idx as f32 * cell_height;
                    Rect::from_min_max(
                        pos2(x, y_bottom - cell_height),
                        pos2(x + cell_width, y_bottom),
                    )
                }
                ArmyType::Ally => {
                    let y_top = rect.min.y + row_idx as f32 * cell_height;
                    Rect::from_min_size(pos2(x, y_top), vec2(cell_width, cell_height))
                }
            };

            let relevant_events: Vec<&BattleEvent> = events
                .iter()
                .filter(|ev| match ev {
                    BattleEvent::ATK { id, .. } if *id == unit.id as u128 => true,
                    BattleEvent::DEF { id, .. } if *id == unit.id as u128 => true,
                    _ => false,
                })
                .collect();

            unit_ui::render(ui, &mut unit_rect, unit, army_type, &relevant_events);
        }
    }
}

fn middle_ui(ui: &mut Ui, rect: Rect) -> (Response, Response, Response) {
    let rect = rect.shrink(ui.rem(0.1));
    let count_font = FontId::proportional(ui.rem(0.3));
    let run_font = FontId::proportional(ui.rem(0.56));
    let start_font = FontId::proportional(ui.rem(0.7));
    let back_font = FontId::proportional(ui.rem(0.46));

    let p = ui.painter();
    p.text(
        rect.left_top(),
        Align2::LEFT_TOP,
        "\u{E6A6} 99",
        count_font.clone(),
        Color32::BLACK,
    );
    p.text(
        rect.left_bottom(),
        Align2::LEFT_BOTTOM,
        "\u{E62A} 99",
        count_font.clone(),
        Color32::BLACK,
    );

    let (left_rect, mid_rect, right_rect) = compute_three_rects(rect, ui.rem(1.2));
    p.text(
        left_rect.center(),
        Align2::CENTER_CENTER,
        "\u{E64C}",
        run_font,
        Color32::BLACK,
    );
    p.text(
        mid_rect.center(),
        Align2::CENTER_CENTER,
        "\u{E65C}",
        start_font,
        Color32::BLACK,
    );
    p.text(
        right_rect.center(),
        Align2::CENTER_CENTER,
        "\u{E68D}",
        back_font,
        Color32::BLACK,
    );
    (
        ui.allocate_rect(left_rect, Sense::click()),
        ui.allocate_rect(mid_rect, Sense::click()),
        ui.allocate_rect(right_rect, Sense::click()),
    )
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

fn compute_three_rects(available_rect: Rect, width: f32) -> (Rect, Rect, Rect) {
    let height = available_rect.height(); // 高度占满可用区域

    let center_y = available_rect.center().y;

    // 中间矩形：整体居中
    let center_x = available_rect.center().x;
    let mid_rect = Rect::from_center_size(pos2(center_x, center_y), vec2(width, height));

    // 左侧区域：available_rect.left() 到 center_x
    let left_region_center_x = (available_rect.left() + center_x) / 2.0;
    let left_rect =
        Rect::from_center_size(pos2(left_region_center_x, center_y), vec2(width, height));

    // 右侧区域：center_x 到 available_rect.right()
    let right_region_center_x = (center_x + available_rect.right()) / 2.0;
    let right_rect =
        Rect::from_center_size(pos2(right_region_center_x, center_y), vec2(width, height));

    (left_rect, mid_rect, right_rect)
}
