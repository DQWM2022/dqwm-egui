use egui::{
    Align2, Color32, FontId, Painter, Pos2, Rect, Response, Sense, StrokeKind, TextureId, Ui,
    Widget, hex_color, pos2, vec2,
};

use crate::app::Unit;

// 战斗视图
#[derive(Clone)]
pub struct QBattleView {
    // 敌方单位
    pub enemy_units: Vec<Vec<Unit>>,
    // 我方单位
    pub friendly_units: Vec<Vec<Unit>>,

    pub bg_unit_id: TextureId, // 背景纹理

    pub rem: f32, // 设计时字体大小

    unit_width: f32,        // 单位宽
    unit_height: f32,       // 单位高
    unit_border_width: f32, // 单位边框
    unit_attr_height: f32,  // 单位属性高度
    unit_atk_width: f32,    // 单位atk宽度
    unit_def_width: f32,    // 单位def宽度

    unit_name_font: FontId, // 名称font
    unit_hp_font: FontId,   // 血条font
    unit_attr_font: FontId, // 属性font
}

impl QBattleView {
    pub fn new(
        enemy_units: Vec<Vec<Unit>>,
        friendly_units: Vec<Vec<Unit>>,
        bg_unit_id: TextureId,
        rem: f32,
    ) -> Self {
        Self {
            enemy_units,
            friendly_units,
            bg_unit_id,
            rem,
            unit_width: rem,
            unit_height: rem * 0.8,
            unit_border_width: 0.03 * rem,
            unit_attr_height: rem * 0.22,
            unit_atk_width: rem * 0.5,
            unit_def_width: rem * 0.35,
            unit_name_font: FontId::proportional(0.2 * rem),
            unit_hp_font: FontId::proportional(0.2 * rem),
            unit_attr_font: FontId::proportional(0.16 * rem),
        }
    }
    // 渲染 单位 阵列
    fn render_battle_area(
        &self,
        rect: Rect,
        painter: &Painter,
        units: &[Vec<Unit>],
        reverse_vertical: bool,
    ) {
        if units.is_empty() {
            return;
        };

        // 计算起始点
        let base_x = rect.min.x;
        let base_y = if reverse_vertical {
            rect.max.y // 底部起点
        } else {
            rect.min.y
        };

        // 计算 宽度 和 高度
        let cell_width = (rect.width() / units.len() as f32).clamp(self.rem, 2.0 * self.rem);
        let cell_height = 1.2 * self.unit_width;

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
                self.render_unit(rect, painter, unit);
            }
        }
    }
    // 单位 - 完整绘制
    fn render_unit(&self, rect: Rect, painter: &Painter, unit: &Unit) {
        // 居中矩形
        let rect = Rect::from_center_size(rect.center(), vec2(self.unit_width, self.unit_height));
        self._draw_unit_hp_bg(rect, painter, (unit.hp as f32) / (unit.max_hp as f32)); // 血条背景
        self._draw_unit_shadow(rect, painter); // 阴影
        self._draw_unit_border(rect, painter); // 边框
        self._draw_unit_name_info(rect, painter, &unit.name); //名称
        self._draw_unit_hp_info(rect, painter, unit.hp, unit.max_hp); // 血量
        self._draw_unit_atk_info(rect, painter, unit.atk); // 攻击
        self._draw_unit_def_info(rect, painter, unit.def); // 防御
    }
    // 单位 - 绘制 阴影
    fn _draw_unit_shadow(&self, rect: Rect, painter: &Painter) {
        // 绘制阴影，通过绘制背景纹理实现，图片的左上角+ 阴影偏移量 才是矩形的左上角
        painter.image(
            self.bg_unit_id,
            rect.scale_from_center(1.2), // 宽高各放大 1.2 倍（+10% 边距）,
            egui::Rect::from_min_max(Pos2::ZERO, egui::pos2(1.0, 1.0)), // UV坐标
            Color32::WHITE,              // 色调（可以用来调暗或着色）
        );
    }
    // 单位 - 绘制 边框
    fn _draw_unit_border(&self, rect: Rect, painter: &Painter) {
        painter.rect_stroke(
            rect,
            0.0,
            egui::Stroke {
                width: self.unit_border_width,
                color: Color32::BLACK,
            },
            StrokeKind::Inside,
        );
    }
    // 单位 - 绘制 血条背景
    fn _draw_unit_hp_bg(&self, rect: Rect, painter: &Painter, hp_ratio: f32) {
        let bg_rect = Rect::from_min_size(
            rect.min,                                     // 使用原始rect的左上角坐标作为起点
            vec2(rect.width() * hp_ratio, rect.height()), // 设置新宽度和原始高度
        );
        painter.rect_filled(bg_rect, 0, Color32::RED);
    }
    // 单位 - 绘制 名称
    fn _draw_unit_name_info(&self, rect: Rect, painter: &Painter, name: &str) {
        painter.text(
            rect.center_top() + vec2(0.0, self.unit_border_width + 0.02 * self.rem),
            Align2::CENTER_TOP,
            name,
            self.unit_name_font.clone(),
            Color32::BLACK,
        );
    }
    fn _draw_unit_hp_info(&self, rect: Rect, painter: &Painter, hp: u128, max_hp: u128) {
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            format!("{}/{}", hp, max_hp),
            self.unit_hp_font.clone(),
            Color32::BLACK,
        );
    }
    // 单位 - 绘制 攻击力背景矩形以及文本
    fn _draw_unit_atk_info(&self, rect: Rect, painter: &Painter, atk: u128) {
        let atk_rect = Rect::from_min_max(
            pos2(
                rect.left() + self.unit_border_width,
                rect.bottom() - self.unit_attr_height,
            ),
            pos2(rect.left() + self.unit_atk_width, rect.bottom()),
        );

        painter.rect_filled(atk_rect, 0.0, hex_color!("#82777780"));
        // 绘制攻击力文本
        painter.text(
            (
                rect.left() - self.unit_border_width,
                rect.bottom() - self.unit_border_width,
            )
                .into(),
            egui::Align2::LEFT_BOTTOM,
            format!("\u{E681}{}", atk),
            self.unit_attr_font.clone(),
            Color32::WHITE,
        );
    }
    // 单位 - 绘制 防御力背景矩形以及文本
    fn _draw_unit_def_info(&self, rect: Rect, painter: &Painter, def: u128) {
        let def_rect = Rect::from_min_max(
            pos2(
                rect.right() - self.unit_def_width,
                rect.bottom() - self.unit_attr_height,
            ),
            pos2(rect.right(), rect.bottom()),
        );
        painter.rect_filled(def_rect, 0.0, hex_color!("#82777780"));
        painter.text(
            (
                rect.right() + self.unit_border_width,
                rect.bottom() - self.unit_border_width,
            )
                .into(),
            egui::Align2::RIGHT_BOTTOM,
            format!("{}\u{E633}", def),
            self.unit_attr_font.clone(),
            Color32::WHITE,
        );
    }
}

impl Widget for QBattleView {
    fn ui(self, ui: &mut Ui) -> Response {
        // 1. 申请整个可用区域（通常是 CentralPanel 的全部）
        let available_rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(available_rect, Sense::click());

        // 2. 获取 painter（全局坐标系）
        let painter = ui.painter();

        // 3. 填充白色背景
        painter.rect_filled(response.rect, 0.0, Color32::WHITE);

        // 4. 将屏幕分为上、中、下
        let r = response.rect;
        let total_h = r.height();

        let top_half = Rect::from_min_max(r.min, pos2(r.max.x, r.min.y + 0.42 * total_h));
        let middle_rect = Rect::from_min_max(
            pos2(r.min.x, r.min.y + 0.42 * total_h),
            pos2(r.max.x, r.min.y + (0.42 + 0.08) * total_h),
        );
        let bottom_half =
            Rect::from_min_max(pos2(r.min.x, r.min.y + (0.42 + 0.08) * total_h), r.max);

        painter.rect_filled(top_half, 0.0, Color32::from_rgb(255, 200, 200)); // 淡红 - 敌方区
        painter.rect_filled(middle_rect, 0.0, Color32::from_rgb(200, 255, 200)); // 淡绿 - 中间区
        painter.rect_filled(bottom_half, 0.0, Color32::from_rgb(200, 200, 255)); // 淡蓝 - 友方区

        // 5. 在指定区域内绘制敌我单位（修改 draw_area 接收 Rect）
        // draw_area(ui, top_half, rem, bg_unit_id, &enemy_units, true);
        let p = ui.painter();
        self.render_battle_area(top_half, p, &self.enemy_units, true);
        //中间简单画个按钮 在 draw_area 之后、按钮之前插入：
        let middle_id: egui::Id = ui.id().with("middle_area");
        let middle_response = ui.interact(middle_rect, middle_id, Sense::click());
        // 画一个 确认
        // middle_response.as
        // middle_response.

        if middle_response.clicked() {
            log::info!("中间区域被点击了！");
            // if let Some(first_col) = self.enemy_units.get_mut(0)
            //     && let Some(first_unit) = first_col.get_mut(0)
            // {
            //     first_unit.hp = first_unit.hp.saturating_sub(10);
            // }
        }

        //draw_area(ui, bottom_half, rem, bg_unit_id, &friendly_units, false);
        self.render_battle_area(bottom_half, p, &self.friendly_units, false);

        middle_response
    }
}
