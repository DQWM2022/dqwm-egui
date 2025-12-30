use std::collections::VecDeque;

use crate::{R, UiExt, model::Unit};

use egui::{
    Align, Color32, FontId, Layout, Rect, Response, RichText, Sense, Stroke, StrokeKind, Ui,
    UiBuilder, Vec2, Widget, vec2,
};
use egui_extras::{Size, StripBuilder};

pub struct UnitUIStyle {
    border_width: f32,
    shadow_id: R,
}
impl Default for UnitUIStyle {
    fn default() -> Self {
        Self {
            border_width: 0.03,
            shadow_id: R::UnitShadow,
        }
    }
}
#[derive(Default)]
pub struct UnitUI {
    style: UnitUIStyle,
}
impl UnitUI {
    fn _build_name(&self, ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            ui.label(RichText::new("杀戮之神").font(FontId::proportional(ui.rem(0.2))));
        });
    }
    fn _build_hp(&self, ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            ui.label(RichText::new("999/1000").font(FontId::proportional(ui.rem(0.2))));
        });
    }
    fn _build_atk(&self, ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right(egui::Align::Max), |ui| {
            ui.label(
                RichText::new("\u{E681}99")
                    .font(FontId::proportional(ui.rem(0.16)))
                    .color(Color32::WHITE),
            );
        });
    }
    fn _build_def(&self, ui: &mut Ui) {
        ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
            ui.label(
                RichText::new("99\u{E633}")
                    .font(FontId::proportional(ui.rem(0.16)))
                    .color(Color32::WHITE),
            );
        });
    }

    fn _shadow(&self, ui: &mut Ui, rect: Rect) {
        ui.painter().image(
            ui.get_texture_id(self.style.shadow_id),
            rect.scale_from_center(2.0),
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    }
    fn _border(&self, ui: &mut Ui, rect: Rect) {
        ui.painter().rect_stroke(
            rect,
            0.0,
            Stroke {
                width: ui.rem(self.style.border_width),
                color: Color32::BLACK,
            },
            StrokeKind::Inside,
        );
    }
    fn _bg_hp(&self, ui: &mut Ui, rect: Rect) {
        let bg_rect = Rect::from_min_size(rect.min, vec2(rect.width() * 1.0, rect.height()));
        ui.painter().rect_filled(bg_rect, 0.0, Color32::RED);
    }
}

impl Widget for UnitUI {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.rem(1.0), ui.rem(0.8)), egui::Sense::hover());

        let mut child_ui =
            ui.new_child(UiBuilder::new().max_rect(rect.shrink(ui.rem(self.style.border_width))));
        self._shadow(ui, rect); //阴影
        self._bg_hp(ui, rect); //背景
        self._border(ui, rect); // 绘制边框
        StripBuilder::new(&mut child_ui)
            .size(Size::relative(0.030))
            .size(Size::relative(0.335))
            .size(Size::relative(0.335))
            .size(Size::relative(0.060))
            .size(Size::relative(0.240))
            .vertical(|mut strip| {
                strip.empty();
                strip.cell(|ui| self._build_name(ui));
                strip.cell(|ui| self._build_hp(ui));
                strip.empty();
                strip.cell(|ui| {
                    StripBuilder::new(ui)
                        .size(Size::relative(0.5))
                        .size(Size::relative(0.15))
                        .size(Size::relative(0.35))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| self._build_atk(ui));
                            strip.empty();
                            strip.cell(|ui| self._build_def(ui));
                        });
                });
            });

        response
    }
}

pub fn battle_ui(ui: &mut Ui, units2: &[VecDeque<Unit>]) {
    ui.spacing_mut().item_spacing = Vec2::ZERO;
    let w = ui.available_width();
    let h = ui.available_height(); // 只读一次

    let a_rect = ui.allocate_exact_size(vec2(w, h * 0.42), Sense::hover()).0;
    let b_rect = ui.allocate_exact_size(vec2(w, h * 0.08), Sense::hover()).0;
    let c_rect = ui.allocate_exact_size(vec2(w, h * 0.50), Sense::hover()).0;
    let mut a_ui = ui.new_child(UiBuilder::new().max_rect(a_rect));
    let mut b_ui = ui.new_child(UiBuilder::new().max_rect(b_rect));
    let mut c_ui = ui.new_child(UiBuilder::new().max_rect(c_rect));

    a_ui.bg(Color32::from_rgb(255, 200, 200));
    b_ui.bg(Color32::from_rgb(200, 255, 200));
    c_ui.bg(Color32::from_rgb(200, 200, 255));
    unit_grid_ui(&mut a_ui, units2, Layout::bottom_up(Align::Center));
    middle_ui(&mut b_ui);
    unit_grid_ui(&mut c_ui, units2, Layout::top_down(Align::Center));
}
pub fn unit_grid_ui(ui: &mut Ui, units2: &[VecDeque<Unit>], layout: Layout) -> Response {
    let unit_h = ui.rem(1.0);
    let h = ui.available_height();
    let max_visible = (h / unit_h) as usize + 1; // 最多能显示多少行
    let col_w = ui.rem(2.0).min(ui.available_width() / units2.len() as f32);
    ui.centered_and_justified(|ui| {
        ui.set_max_width(col_w * units2.len() as f32);
        ui.columns(units2.len(), |uis| {
            for (i, col_ui) in uis.iter_mut().enumerate() {
                col_ui.with_layout(layout, |ui| {
                    let count = max_visible.min(units2[i].len());
                    for _unit in units2[i].range(0..count) {
                        ui.add_sized(Vec2::new(ui.available_width(), unit_h), UnitUI::default());
                    }
                });
            }
        });
    })
    .response
}

pub fn middle_ui(ui: &mut Ui) {
    //  E6A6 E62A  1 E64C  2 E65C 3 E68D

    StripBuilder::new(ui)
        .size(Size::relative(0.1))
        .size(Size::relative(0.3))
        .size(Size::relative(0.2))
        .size(Size::relative(0.3))
        .size(Size::relative(0.1))
        .horizontal(|mut strip| {
            strip.cell(|ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.5))
                    .size(Size::relative(0.5))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                                ui.label(
                                    RichText::new("\u{E6A6} 99")
                                        .font(FontId::proportional(ui.rem(0.3))),
                                );
                            });
                        });
                        strip.cell(|ui| {
                            ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                                ui.label(
                                    RichText::new("\u{E62A} 99")
                                        .font(FontId::proportional(ui.rem(0.3))),
                                );
                            });
                        });
                    });
            });

            strip.cell(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new("\u{E64C}").font(FontId::proportional(ui.rem(0.8))));
                });
            });
            strip.cell(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new("\u{E65C}").font(FontId::proportional(ui.rem(0.8))));
                });
            });

            strip.cell(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new("\u{E68D}").font(FontId::proportional(ui.rem(0.8))));
                });
            });
            strip.empty();
        });
}
