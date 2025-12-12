use egui::{Color32, FontId, TextureId};

#[derive(Clone, Debug)]
pub enum MiddleIconAlign {
    Left,
    Right,
    Center,
}
#[derive(Clone, Debug)]
pub enum Side {
    Enemy,
    Ally,
}

#[derive(Clone, Debug)]
pub struct MiddleAreaStyle {
    pub margin: f32,
    pub enemy_count_style: MiddleCountStyle,
    pub ally_count_style: MiddleCountStyle,
    pub start_battle_style: MiddleIconStyle,
    pub run_style: MiddleIconStyle,
    pub return_style: MiddleIconStyle,
}

#[derive(Clone, Debug)]
pub struct MiddleIconStyle {
    pub icon: &'static str,
    pub font: FontId,
    pub color: Color32,
    pub align: MiddleIconAlign,
}

#[derive(Clone, Debug)]
pub struct MiddleCountStyle {
    pub icon: &'static str,      //
    pub text_font: egui::FontId, // 字体
    pub icon_font: egui::FontId, // 字体
    pub color: egui::Color32,    // 颜色
    pub side: Side,
}

#[derive(Clone, Debug)]
pub struct UnitStyle {
    pub width: f32,        // 单位宽
    pub height: f32,       // 单位高
    pub border_width: f32, // 单位边框

    pub name_font: FontId, // 名称字体
    pub hp_font: FontId,   // 血条字体
    pub attr_font: FontId, // 属性字体

    pub atk_def_height: f32, // 攻击防御区 高度
    pub atk_width: f32,      // 单位atk宽度
    pub def_width: f32,      // 单位def宽度

    pub atk_bg_color: Color32, // hex_color!("#82777780")
    pub def_bg_color: Color32, // hex_color!("#82777780")

    pub bg_texture_id: TextureId, // 背景纹理id

    pub name_margin_top: f32, // 0.02 *rem

    pub atk_font_icon: &'static str, // 攻击字体图标 \u{E681}
    pub def_font_icon: &'static str, // 防御字体图标 \u{E633}
}
