use egui::{Response, Widget};
use egui_phosphor::bold as ph;

#[derive(Clone)]
pub struct WindowFrame {
    title: String,
}

impl WindowFrame {
    const TOP_BAR_HEIGHT: f32 = 36.0;

    pub fn new(title: &str) -> Self {
        WindowFrame {
            title: title.to_owned(),
        }
    }

    pub fn show(
        &self, 
        ctx: &egui::Context, 
        add_contents: impl FnOnce(&mut egui::Ui),
    ) {
        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));

        let panel_frame = egui::Frame::new()
            .fill(ctx.style().visuals.window_fill())
            .stroke(if is_maximized {
                egui::Stroke::NONE
            } else {
                let mut stroke = ctx.style().visuals.window_stroke();
                stroke.width = 1.0;
                stroke
            });

        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let resize_margin = 12.0;
            let pointer_pos = ctx.input(|i| i.pointer.hover_pos());
            if let Some(pos) = pointer_pos {
                let mut resize_dir = None;
                let mut cursor_icon = None;
                let window_rect = app_rect;

                // Check edges and corners
                let left = (pos.x - window_rect.left()).abs() < resize_margin && pos.y > window_rect.top() + Self::TOP_BAR_HEIGHT;
                let right = (pos.x - window_rect.right()).abs() < resize_margin && pos.y > window_rect.top() + Self::TOP_BAR_HEIGHT;
                let bottom = (pos.y - window_rect.bottom()).abs() < resize_margin;

                // Corners
                if right && bottom {
                    resize_dir = Some(egui::ResizeDirection::SouthEast);
                    cursor_icon = Some(egui::CursorIcon::ResizeNwSe);
                } else if left && bottom {
                    resize_dir = Some(egui::ResizeDirection::SouthWest);
                    cursor_icon = Some(egui::CursorIcon::ResizeNeSw);
                } else if left {
                    resize_dir = Some(egui::ResizeDirection::West);
                    cursor_icon = Some(egui::CursorIcon::ResizeEast);
                } else if right {
                    resize_dir = Some(egui::ResizeDirection::East);
                    cursor_icon = Some(egui::CursorIcon::ResizeEast);
                }

                if let Some(icon) = cursor_icon {
                    ctx.output_mut(|o| o.cursor_icon = icon);
                }

                if let Some(dir) = resize_dir {
                    if ui.input(|i| i.pointer.primary_down()) {
                        ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(dir));
                    }
                }
            }

            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + Self::TOP_BAR_HEIGHT;
                rect
            };
            title_bar_ui(ui, title_bar_rect, &self.title);

            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);

            let mut content_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));
            add_contents(&mut content_ui);
        });
    }
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    let painter = ui.painter();

    let title_bar_response = ui.interact(
        title_bar_rect,
        egui::Id::new("title_bar"),
        egui::Sense::click_and_drag(),
    );

    painter.text(
        title_bar_rect.center(),
        egui::Align2::CENTER_CENTER,
        title,
        egui::FontId::proportional(16.0),
        ui.style().visuals.text_color(),
    );

    painter.line_segment(
        [
            title_bar_rect.left_bottom() + egui::vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + egui::vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.drag_started_by(egui::PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(title_bar_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        },
    );
}

fn close_maximize_minimize(ui: &mut egui::Ui) {
    let close_response = ui.add(WindowButton::new(ph::X));
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_maximized {
        let maximized_response = ui.add(WindowButton::new(ph::ARROWS_IN));
        if maximized_response.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(false));
        }
    } else {
        let maximized_response = ui.add(WindowButton::new(ph::ARROWS_OUT));
        if maximized_response.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui.add(WindowButton::new(ph::MINUS));
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }
}

pub struct WindowButton {
    icon: egui::RichText,
}

impl WindowButton {
    pub const BUTTON_HEIGHT: f32 = 20.0;

    pub fn new(icon: impl Into<egui::RichText>) -> Self {
        WindowButton {
            icon: icon.into(),
        }
    }
}

impl Widget for WindowButton {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.add(
            egui::Button::new(
                self.icon
                    .size(WindowButton::BUTTON_HEIGHT)
                    .strong()
            )
            .frame(true)
            .fill(egui::Color32::TRANSPARENT)
        )
    }
}