use egui::{
    Align2, Color32, FontId, Painter, Pos2, Rect, Response, Sense, StrokeKind, TextureId, Ui, Vec2,
    hex_color, pos2, vec2,
};

use crate::app::Unit;

// 战斗视图
#[derive(Clone)]
pub struct QBattleView {
    pub bg_unit_id: TextureId, // 背景纹理

    pub rem: f32, // 设计时字体大小

    unit_width: f32,        // 单位宽
    unit_height: f32,       // 单位高
    unit_border_width: f32, // 单位边框
    unit_attr_height: f32,  // 单位属性高度
    unit_atk_width: f32,    // 单位atk宽度
    unit_def_width: f32,    // 单位def宽度

    unit_name_font: FontId, // 名称字体
    unit_hp_font: FontId,   // 血条字体
    unit_attr_font: FontId, // 属性字体
    // 中间区域
    middle_amount_font: FontId, // 数量字体
    middle_amount_margin: f32,  // 中间数量外边距

    middle_amount_icon_font: FontId, // 数量图标字体

    middle_amount_enemy_icon: String,   // 敌方数量 图标
    middle_amount_enemy_color: Color32, // 敌方数量 颜色

    middle_amount_friendly_icon: String,   // 我方数量 图标
    middle_amount_friendly_color: Color32, // 我方数量 颜色

    middle_run_font: FontId,   // 逃跑 图标 字体
    middle_run_color: Color32, // 逃跑 颜色
    middle_run_icon: String,   // 逃跑 图标

    middle_start_battle_font: FontId,   // 开始战斗 图标 字体
    middle_start_battle_color: Color32, // 开始战斗 颜色
    middle_start_battle_icon: String,   // 开始战斗 图标

    middle_return_font: FontId,   // 返回 图标 字体
    middle_return_color: Color32, // 返回 颜色
    middle_return_icon: String,   // 返回 图标

    area_top_bottom_ratio: f32, // 上区域所占 比例
    area_middle_ratio: f32,     // 中区域所占 比例
}

impl QBattleView {
    pub fn new(bg_unit_id: TextureId, rem: f32) -> Self {
        Self {
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

            middle_amount_font: FontId::proportional(0.26 * rem),
            middle_amount_margin: 0.1 * rem,
            middle_amount_icon_font: FontId::proportional(0.38 * rem),

            middle_amount_enemy_icon: "\u{E6A6}".to_string(),
            middle_amount_enemy_color: Color32::BLACK,

            middle_amount_friendly_icon: "\u{E625}".to_string(),
            middle_amount_friendly_color: Color32::BLACK,

            middle_run_font: FontId::proportional(0.56 * rem),
            middle_run_color: Color32::BLACK,
            middle_run_icon: "\u{E64C}".to_string(),

            middle_start_battle_font: FontId::proportional(0.7 * rem),
            middle_start_battle_color: Color32::RED,
            middle_start_battle_icon: "\u{E65C}".to_string(),

            middle_return_font: FontId::proportional(0.46 * rem),
            middle_return_color: Color32::BLACK,
            middle_return_icon: "\u{E68D}".to_string(),

            area_top_bottom_ratio: 0.42,
            area_middle_ratio: 0.08,
        }
    }
    // 渲染
    pub fn render(
        self,
        enemy_units: &[Vec<Unit>],
        friendly_units: &[Vec<Unit>],
        ui: &mut Ui,
    ) -> (Response, Response, Response) {
        // 1. 申请整个可用区域（通常是 CentralPanel 的全部）
        let available_rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(available_rect, Sense::hover());
        let p = ui.painter();

        // 分割区域
        let (top_half, middle_rect, bottom_half) = self.split_battle_area(response.rect);
        // 渲染背景
        p.rect_filled(response.rect, 0.0, Color32::WHITE);
        // =========DEBUG===========
        p.rect_filled(top_half, 0.0, Color32::from_rgb(255, 200, 200)); // 淡红 - 敌方区
        p.rect_filled(middle_rect, 0.0, Color32::from_rgb(200, 255, 200)); // 淡绿 - 中间区
        p.rect_filled(bottom_half, 0.0, Color32::from_rgb(200, 200, 255)); // 淡蓝 - 友方区
        // ========DEBUG============

        // 待调整 TODO
        let enemy_total_count: usize = enemy_units.iter().map(Vec::len).sum();
        let friendly_total_count: usize = friendly_units.iter().map(Vec::len).sum();

        // 敌方区域
        self.render_battle_area(top_half, p, enemy_units, true);
        // 我方阵型
        self.render_battle_area(bottom_half, p, friendly_units, false);
        // 中间区域  并且 返回三个按钮的响应
        self.render_middle_area(middle_rect, ui, enemy_total_count, friendly_total_count)
    }
    // 渲染 单位 阵列
    fn render_battle_area(
        &self,
        rect: Rect,
        painter: &Painter,
        units: &[Vec<Unit>],
        reverse_vertical: bool,
    ) {
        if units.is_empty() || units[0].is_empty() {
            return;
        }

        let num_cols = units.len();
        let cell_width = (rect.width() / num_cols as f32).clamp(self.rem, 2.0 * self.rem);
        let total_used_width = cell_width * num_cols as f32;

        // 计算整体居中的起始X坐标
        let start_x = rect.min.x + (rect.width() - total_used_width) / 2.0;

        let base_y = if reverse_vertical {
            rect.max.y // 底部起点
        } else {
            rect.min.y
        };

        let cell_height = 1.2 * self.unit_width;

        for (col_idx, column) in units.iter().enumerate() {
            let x = start_x + col_idx as f32 * cell_width;

            for (row_idx, unit) in column.iter().enumerate() {
                let unit_rect = if reverse_vertical {
                    // 从底部开始向上绘制
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
                self.render_unit(unit_rect, painter, unit);
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
    // 单位 - 绘制 血量
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
    // 按照比例切分矩形
    fn split_battle_area(&self, rect: Rect) -> (Rect, Rect, Rect) {
        let total_h = rect.height();
        let top = Rect::from_min_max(
            rect.min,
            pos2(
                rect.max.x,
                rect.min.y + self.area_top_bottom_ratio * total_h,
            ),
        );

        let middle = Rect::from_min_max(
            pos2(
                rect.min.x,
                rect.min.y + self.area_top_bottom_ratio * total_h,
            ),
            pos2(
                rect.max.x,
                rect.min.y + (self.area_top_bottom_ratio + self.area_middle_ratio) * total_h,
            ),
        );

        let bottom = Rect::from_min_max(
            pos2(
                rect.min.x,
                rect.min.y + (self.area_top_bottom_ratio + self.area_middle_ratio) * total_h,
            ),
            rect.max,
        );

        (top, middle, bottom)
    }
    // 渲染中间区域
    fn render_middle_area(
        &self,
        rect: Rect,
        ui: &Ui,
        enemy_total_count: usize,
        friendly_total_count: usize,
    ) -> (Response, Response, Response) {
        // 左侧敌方我方人数
        self._draw_enemy_count(rect, ui, enemy_total_count);
        //左侧我方人数
        self._draw_friendly_count(rect, ui, friendly_total_count);
        // 逃跑图标
        let run_response = self._draw_run(rect, ui);
        // 开始战斗图标
        let start_battle_response = self._draw_start_battle(rect, ui);
        // 右侧 返回 图标
        let return_response = self._draw_return(rect, ui);
        (run_response, start_battle_response, return_response)
    }
    // 中间-绘制敌方数量
    fn _draw_enemy_count(&self, rect: Rect, ui: &Ui, num: usize) {
        let margin: f32 = self.middle_amount_margin;

        // 测量图标尺寸
        let icon_size = measure_text_size(
            ui,
            &self.middle_amount_enemy_icon,
            self.middle_amount_icon_font.clone(),
            self.middle_amount_enemy_color,
        );

        // 计算垂直居中的y坐标
        let icon_y_center = rect.top() + margin + icon_size.y / 2.0;

        // 图标位置
        let icon_x = rect.left() + margin;
        ui.painter().text(
            pos2(icon_x, icon_y_center),
            Align2::LEFT_CENTER,
            &self.middle_amount_enemy_icon,
            self.middle_amount_icon_font.clone(),
            self.middle_amount_enemy_color,
        );

        // 绘制数字（左对齐）
        ui.painter().text(
            pos2(icon_x + icon_size.x + margin, icon_y_center),
            Align2::LEFT_CENTER,
            num,
            self.middle_amount_font.clone(),
            Color32::BLACK,
        );
    }
    // 中间-绘制我方数量
    fn _draw_friendly_count(&self, rect: Rect, ui: &Ui, num: usize) {
        let margin = self.middle_amount_margin;
        // 测量图标尺寸
        let icon_size = measure_text_size(
            ui,
            &self.middle_amount_friendly_icon,
            self.middle_amount_icon_font.clone(),
            self.middle_amount_friendly_color,
        );
        // 计算图标y坐标使其垂直居中于底部margin处
        let y_bottom = rect.bottom() - margin;
        let icon_y_center = y_bottom - margin - icon_size.y / 2.0;
        // 图标位置
        let icon_x = rect.left() + margin;
        ui.painter().text(
            egui::pos2(icon_x, icon_y_center),
            egui::Align2::LEFT_CENTER,
            &self.middle_amount_friendly_icon,
            self.middle_amount_icon_font.clone(),
            self.middle_amount_friendly_color,
        );

        // 绘制数字，确保其与图标垂直居中
        let num_text = format!("{}", num);
        ui.painter().text(
            egui::pos2(icon_x + icon_size.x + margin, icon_y_center),
            egui::Align2::LEFT_CENTER,
            &num_text,
            self.middle_amount_font.clone(),
            Color32::BLACK,
        );
    }
    // 中间- 绘制 逃跑
    fn _draw_run(&self, rect: Rect, ui: &Ui) -> Response {
        // 使用rect的左半部分
        let left_half_rect = Rect::from_min_max(
            rect.min,
            pos2(rect.center().x, rect.max.y), // 到达rect的中心点作为新rect的右边界
        );

        // 计算文本/图标的大小
        let run_size = measure_text_size(
            ui,
            &self.middle_run_icon,
            self.middle_run_font.clone(),
            self.middle_run_color,
        );

        // 创建图标应当放置的矩形区域，并确保其在left_half_rect中居中
        let icon_rect = egui::Rect::from_center_size(
            left_half_rect.center(), // 使用left_half_rect的中心点作为新icon_rect的中心点
            run_size,
        );

        // 交互响应
        let run_response = ui.interact(icon_rect, ui.id().with("run_icon"), egui::Sense::click());

        // 绘制图标
        ui.painter().text(
            icon_rect.center(), // 使用icon_rect的中心点作为文本绘制起点
            egui::Align2::CENTER_CENTER,
            &self.middle_run_icon,
            self.middle_run_font.clone(),
            self.middle_run_color,
        );

        run_response
    }
    // 中间- 绘制 开始战斗
    fn _draw_start_battle(&self, rect: Rect, ui: &Ui) -> Response {
        let start_battle_size = measure_text_size(
            ui,
            &self.middle_start_battle_icon,
            self.middle_start_battle_font.clone(),
            self.middle_start_battle_color,
        );
        let start_battle_rect = egui::Rect::from_center_size(rect.center(), start_battle_size);
        let start_battle_id: egui::Id = ui.id().with("start_battle_id");
        let start_battle_response = ui.interact(start_battle_rect, start_battle_id, Sense::click());
        ui.painter().text(
            start_battle_rect.center(),
            egui::Align2::CENTER_CENTER,
            &self.middle_start_battle_icon,
            self.middle_start_battle_font.clone(),
            self.middle_start_battle_color,
        );
        start_battle_response
    }
    // 中间- 绘制 返回
    fn _draw_return(&self, rect: Rect, ui: &Ui) -> Response {
        // 假设我们只使用rect的右半部分
        let right_half_rect = Rect::from_min_max(
            pos2(rect.center().x, rect.min.y), // 从rect的中心点开始作为新rect的左边界
            rect.max,
        );

        // 计算文本/图标的大小
        let return_size = measure_text_size(
            ui,
            &self.middle_return_icon,
            self.middle_return_font.clone(),
            self.middle_return_color,
        );

        // 创建图标应当放置的矩形区域，并确保其在right_half_rect中居中
        let icon_rect = egui::Rect::from_center_size(
            right_half_rect.center(), // 使用right_half_rect的中心点作为新icon_rect的中心点
            return_size,
        );

        // 交互响应
        let return_response =
            ui.interact(icon_rect, ui.id().with("return_icon"), egui::Sense::click());

        // 绘制图标
        ui.painter().text(
            icon_rect.center(), // 使用icon_rect的中心点作为文本绘制起点
            egui::Align2::CENTER_CENTER,
            &self.middle_return_icon,
            self.middle_return_font.clone(),
            self.middle_return_color,
        );

        return_response
    }
}
#[inline]
fn measure_text_size(ui: &egui::Ui, text: &str, font: egui::FontId, color: egui::Color32) -> Vec2 {
    ui.fonts_mut(|f| f.layout_no_wrap(text.to_owned(), font, color))
        .size()
}
