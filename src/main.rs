use eframe::{
    egui::{Id, Widget as _},
    epaint::Pos2,
};

use crate::sense::SenseExt;

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions {
        // transparent: true,
        // min_window_size: Some(Vec2::new(800., 600.)),
        // resizable: true,
        // decorated: false,
        drag_and_drop_support: true,
        ..Default::default()
    };

    let _run = eframe::run_native(
        "Scalpel",
        native_options,
        Box::new(move |eframe::CreationContext { egui_ctx, .. }| {
            let mut fonts = eframe::egui::FontDefinitions::default();
            fonts
                .families
                .entry(eframe::egui::FontFamily::Proportional)
                .or_default()
                .push("MaterialIcons-Regular".to_owned());

            let mut text_styles = egui_ctx.style().text_styles.clone();

            text_styles
                .entry(eframe::egui::TextStyle::Heading)
                .and_modify(|d| {
                    *d = eframe::epaint::FontId::new(14.8, eframe::egui::FontFamily::Proportional)
                });

            egui_ctx.set_fonts(fonts);
            egui_ctx.set_style(eframe::egui::Style {
                override_text_style: Some(eframe::egui::TextStyle::Heading),
                text_styles,
                animation_time: 0.3,
                ..Default::default()
            });

            let mut fonts = eframe::egui::FontDefinitions::default();

            fonts.font_data.insert(
                "FONT_PRO_SOLID".to_owned(),
                eframe::egui::FontData::from_static(fontawesome_fonts::FONT_PRO_SOLID),
            );
            fonts.font_data.insert(
                "FONT_BRANDS_REGULAR".to_owned(),
                eframe::egui::FontData::from_static(fontawesome_fonts::FONT_BRANDS_REGULAR),
            );

            let list = fonts
                .families
                .entry(eframe::egui::FontFamily::Proportional)
                .or_default();

            list.push("FONT_PRO_SOLID".to_owned());
            list.push("FONT_BRANDS_REGULAR".to_owned());

            // get_system_fonts(&mut fonts);

            egui_ctx.set_fonts(fonts);

            Box::new(HomePage::default())
        }),
    );
}

#[derive(Default)]
pub struct HomePage {
    text: String,
}
impl eframe::App for HomePage {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            let response = eframe::egui::TextEdit::multiline(&mut self.text)
                .show(ui)
                .response;
            response.md_context_menu(ui, |ui| {
                if ui
                    .menu_item_icon_and_text("Cut".into(), "📋", Some(eframe::egui::Key::A), true)
                    .clicked()
                {};
                ui.sub_menu_item_icon_and_text("Copy Body", "📋", true, |ui| {
                    if ui
                        .menu_item_icon_and_text(
                            "Cut".into(),
                            "📋",
                            Some(eframe::egui::Key::A),
                            true,
                        )
                        .clicked()
                    {};
                });
                ui.sub_menu_item_icon_and_text("Copy", "📋", true, |ui| {});
            });
        });
    }
}

#[derive(Clone)]
pub struct ContextMenuBuilder {
    id: eframe::egui::Id,
    pos2: Option<eframe::epaint::Pos2>,
    max_size: Option<eframe::egui::Vec2>,
}
impl ContextMenuBuilder {
    pub fn new(id: eframe::egui::Id) -> Self {
        Self {
            pos2: None,
            id,
            max_size: None,
        }
    }

    pub fn max_size(mut self, max_size: eframe::egui::Vec2) -> Self {
        self.max_size = Some(max_size);
        self
    }
    pub fn pos2(mut self, pos2: eframe::epaint::Pos2) -> Self {
        self.pos2 = Some(pos2);
        self
    }
}
impl ContextMenuBuilder {
    pub fn ui(
        self,
        ui: &mut eframe::egui::Ui,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::Response {
        let ContextMenuBuilder { pos2, id, max_size } = self;
        let max_size = max_size.unwrap_or(eframe::egui::vec2(300., 0.));
        let fill = eframe::egui::Color32::from_rgb(23, 29, 30);
        let pos2 = pos2.unwrap_or(eframe::egui::Pos2::ZERO);
        let resp = eframe::egui::Area::new(id)
            .fixed_pos(pos2)
            .interactable(true)
            .order(eframe::egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                ui.add_sized(max_size, |ui: &mut eframe::egui::Ui| {
                    eframe::egui::Frame {
                        inner_margin: eframe::egui::Margin::same(5.),
                        outer_margin: eframe::egui::Margin::same(0.),
                        rounding: eframe::egui::Rounding::same(10.),
                        shadow: eframe::epaint::Shadow::NONE,
                        fill,
                        stroke: eframe::egui::Stroke::NONE,
                    }
                    .show(ui, |ui| {
                        ui.vertical(|ui| add_contents(ui));
                    })
                    .response
                });
            })
            .set_sense(ui, eframe::egui::Sense::click());

        resp
    }
}

pub struct MenuItemBuilder {
    label: Option<String>,
    icon: Option<String>,
    shortcut: Vec<eframe::egui::Key>,
    enable: bool,
}

impl MenuItemBuilder {
    pub fn new(label: Option<String>, icon: Option<String>, enable: bool) -> Self {
        Self {
            label,
            icon,
            shortcut: vec![],
            enable,
        }
    }
    pub fn new_icon(icon: &str, enable: bool) -> Self {
        Self {
            label: None,
            icon: Some(icon.into()),
            shortcut: vec![],
            enable,
        }
    }
    pub fn text(label: &str, enable: bool) -> Self {
        Self {
            label: Some(label.into()),
            icon: None,
            shortcut: vec![],
            enable,
        }
    }
    pub fn shortcut(mut self, shortcut: Option<eframe::egui::Key>) -> Self {
        if let Some(shortcut) = shortcut {
            self.shortcut.push(shortcut);
        }

        self
    }
}

impl eframe::egui::Widget for MenuItemBuilder {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let MenuItemBuilder {
            label,
            icon,
            shortcut,
            enable,
        } = self;

        let frame = eframe::egui::Frame::none()
            .inner_margin(eframe::egui::Margin {
                left: 10.,
                right: 10.,
                top: 5.,
                bottom: 5.,
            })
            .rounding(eframe::egui::Rounding::same(5.));
        let resp = frame
            .show(ui, |ui| {
                ui.add_enabled_ui(enable, |ui| {
                    ui.with_layout(
                        eframe::egui::Layout::left_to_right(eframe::emath::Align::Center),
                        |ui| {
                            if let Some(icon) = &icon {
                                ui.allocate_ui(eframe::egui::vec2(10.0, 10.0), |ui| {
                                    ui.label(icon);
                                });
                            }

                            if let Some(text) = label {
                                ui.label(text);
                            }
                            ui.with_layout(
                                eframe::egui::Layout::right_to_left(eframe::emath::Align::Center),
                                |ui| {
                                    if !shortcut.is_empty() {
                                        use itertools::Itertools;

                                        let text = shortcut
                                            .iter()
                                            .map(|f| format!("{}", f.name().to_uppercase()))
                                            .join(" + ");
                                        ui.label(eframe::egui::RichText::new(text).small());
                                    }
                                },
                            );
                        },
                    );
                })
            })
            .set_sense(ui, eframe::egui::Sense::click());
        if resp.hovered() {
            ui.painter().rect(
                resp.rect,
                eframe::egui::Rounding::same(5.),
                eframe::egui::Color32::from_rgba_premultiplied(23, 29, 30, 10),
                eframe::egui::Stroke::NONE,
            );
        }

        resp
    }
}

// pub struct SubMenu {
//     id: eframe::egui::Id,
//     pos2: Option<eframe::epaint::Pos2>,
//     max_size: Option<eframe::egui::Vec2>,
// }
// impl SubMenu {
//     pub fn new(id: eframe::egui::Id) -> Self {
//         Self {
//             pos2: None,
//             id,
//             max_size: None,
//         }
//     }
//     pub fn max_size(mut self, max_size: eframe::egui::Vec2) -> Self {
//         self.max_size = Some(max_size);
//         self
//     }
//     pub fn pos2(mut self, pos2: eframe::epaint::Pos2) -> Self {
//         self.pos2 = Some(pos2);
//         self
//     }
// }

pub struct SubMenuItemBuilder {
    label: Option<String>,
    icon: Option<String>,

    enable: bool,
    sub_menu: ContextMenuBuilder,
}

impl SubMenuItemBuilder {
    pub fn new(label: Option<String>, icon: Option<String>, enable: bool) -> Self {
        Self {
            label,
            icon,
            enable,
            sub_menu: ContextMenuBuilder::new(Id::new(uuid::Uuid::new_v4())),
        }
    }
    pub fn icon(icon: Option<String>, enable: bool) -> Self {
        Self {
            label: None,
            icon,
            enable,
            sub_menu: ContextMenuBuilder::new(Id::new(uuid::Uuid::new_v4())),
        }
    }
    pub fn text(label: Option<String>, enable: bool) -> Self {
        Self {
            label,
            icon: None,
            enable,
            sub_menu: ContextMenuBuilder::new(Id::new(uuid::Uuid::new_v4())),
        }
    }
}
impl SubMenuItemBuilder {
    fn ui(
        self,
        ui: &mut eframe::egui::Ui,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::Response {
        let SubMenuItemBuilder {
            label,
            icon,
            enable,
            sub_menu,
        } = self;
        let frame = eframe::egui::Frame::none()
            .inner_margin(eframe::egui::Margin {
                left: 10.,
                right: 10.,
                top: 5.,
                bottom: 5.,
            })
            .rounding(eframe::egui::Rounding::same(5.));
        ui.with_layout(
            eframe::egui::Layout::left_to_right(eframe::emath::Align::Min),
            |ui| {
                let resp = frame
                    .show(ui, |ui| {
                        ui.add_enabled_ui(enable, |ui| {
                            ui.with_layout(
                                eframe::egui::Layout::left_to_right(eframe::emath::Align::Center),
                                |ui| {
                                    if let Some(icon) = &icon {
                                        ui.allocate_ui(eframe::egui::vec2(10.0, 10.0), |ui| {
                                            ui.label(icon);
                                        });
                                    }

                                    if let Some(text) = label {
                                        ui.label(text);
                                    }
                                    ui.with_layout(
                                        eframe::egui::Layout::right_to_left(
                                            eframe::emath::Align::Center,
                                        ),
                                        |ui| {
                                            ui.label(eframe::egui::RichText::new(">"));
                                        },
                                    );
                                },
                            );
                        })
                    })
                    .set_sense(ui, eframe::egui::Sense::click());

                if resp.hovered() {
                    ui.painter().rect(
                        resp.rect,
                        eframe::egui::Rounding::same(5.),
                        eframe::egui::Color32::from_rgba_premultiplied(23, 29, 30, 10),
                        eframe::egui::Stroke::NONE,
                    );
                    ui.push_menu_item_id(resp.id);
                    sub_menu.pos2(resp.rect.right_top()).ui(ui, add_contents);
                } else if ui.get_menu_item_id().is_some() {
                    ui.painter().rect(
                        resp.rect,
                        eframe::egui::Rounding::same(5.),
                        eframe::egui::Color32::from_rgba_premultiplied(23, 29, 30, 10),
                        eframe::egui::Stroke::NONE,
                    );
                    sub_menu.pos2(resp.rect.right_top()).ui(ui, add_contents);
                }
            },
        )
        .response
    }
}

pub trait MenuItem {
    fn menu_item_icon_and_text(
        &mut self,
        text: &str,
        icon: &str,
        shortcut: Option<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response;

    fn menu_item_with_text(
        &mut self,
        text: &str,
        shortcut: Option<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response;
    fn menu_item_with_icon(
        &mut self,
        icon: &str,
        shortcut: Option<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response;
    fn sub_menu_item_icon_and_text(
        &mut self,
        text: &str,
        icon: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::Response;
    fn sub_menu_item_with_text(
        &mut self,
        text: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::Response;
    fn sub_menu_item_with_icon(
        &mut self,
        icon: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::Response;
    fn close_context_menu(&mut self);
}
impl MenuItem for eframe::egui::Ui {
    fn menu_item_icon_and_text(
        &mut self,
        text: &str,
        icon: &str,
        shortcut: Option<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response {
        MenuItemBuilder::new(Some(text.into()), Some(icon.into()), enable)
            .shortcut(shortcut)
            .ui(self)
    }

    fn menu_item_with_text(
        &mut self,
        text: &str,
        shortcut: Option<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response {
        MenuItemBuilder::text(text.into(), enable)
            .shortcut(shortcut)
            .ui(self)
    }
    fn menu_item_with_icon(
        &mut self,
        icon: &str,
        shortcut: Option<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response {
        MenuItemBuilder::new_icon(icon.into(), enable)
            .shortcut(shortcut)
            .ui(self)
    }
    fn sub_menu_item_icon_and_text(
        &mut self,
        text: &str,
        icon: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::Response {
        SubMenuItemBuilder::new(Some(text.into()), Some(icon.into()), enable).ui(self, add_contents)
    }
    fn sub_menu_item_with_text(
        &mut self,
        text: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::Response {
        SubMenuItemBuilder::text(Some(text.into()), enable).ui(self, add_contents)
    }
    fn sub_menu_item_with_icon(
        &mut self,
        icon: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::Response {
        SubMenuItemBuilder::icon(Some(icon.into()), enable).ui(self, add_contents)
    }

    fn close_context_menu(&mut self) {
        self.clear_menu_item_id();
        self.clear_menu_pos();
    }
}

pub trait MenuExt {
    fn push_menu_pos(&mut self, pos2: Option<eframe::epaint::Pos2>);
    fn get_menu_pos(&mut self) -> Option<Option<eframe::epaint::Pos2>>;
    fn clear_menu_pos(&mut self);
    fn push_menu_item_id(&mut self, id: eframe::egui::Id);
    fn get_menu_item_id(&mut self) -> Option<eframe::egui::Id>;
    fn clear_menu_item_id(&mut self);
}

impl MenuExt for eframe::egui::Ui {
    fn push_menu_pos(&mut self, pos2: Option<eframe::epaint::Pos2>) {
        self.memory_mut(|f| f.data.insert_temp(Id::new("my_context_menu"), pos2))
    }

    fn get_menu_pos(&mut self) -> Option<Option<eframe::epaint::Pos2>> {
        self.memory(|f| f.data.get_temp(Id::new("my_context_menu")))
    }

    fn clear_menu_pos(&mut self) {
        self.memory_mut(|f| f.data.remove::<Option<Pos2>>(Id::new("my_context_menu")))
    }

    fn push_menu_item_id(&mut self, id: eframe::egui::Id) {
        self.memory_mut(|f| f.data.insert_temp(Id::new("menu_item_id"), id))
    }

    fn get_menu_item_id(&mut self) -> Option<eframe::egui::Id> {
        self.memory(|f| f.data.get_temp(Id::new("menu_item_id")))
    }

    fn clear_menu_item_id(&mut self) {
        self.memory_mut(|f| f.data.remove::<Id>(Id::new("menu_item_id")))
    }
}

pub trait ContextMenu {
    fn md_context_menu(
        self,
        ui: &mut eframe::egui::Ui,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> Option<eframe::egui::Response>;
}
impl ContextMenu for eframe::egui::Response {
    fn md_context_menu(
        self,
        ui: &mut eframe::egui::Ui,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> Option<eframe::egui::Response> {
        if self.secondary_clicked() {
            ui.push_menu_pos(self.interact_pointer_pos())
        }
        let pos2 = ui.get_menu_pos();
        let resp = if let Some(Some(pos2)) = pos2 {
            Some(
                ContextMenuBuilder::new(Id::new("my_context_menu"))
                    .pos2(pos2)
                    .ui(ui, add_contents),
            )
        } else {
            None
        };
        if let Some(response) = &resp {
            if response.clicked_elsewhere() {
                ui.clear_menu_pos();
            }
        }

        resp
    }
}

pub mod sense {
    use eframe::egui::{InnerResponse, Response, Sense, Ui};

    pub trait SenseExt {
        fn set_sense(self, ui: &mut Ui, sense: Sense) -> Response;
    }

    impl SenseExt for Response {
        #[inline(always)]
        fn set_sense(self, ui: &mut Ui, sense: Sense) -> Response {
            ui.allocate_rect(self.rect, sense)
        }
    }

    impl<T> SenseExt for InnerResponse<T> {
        #[inline(always)]
        fn set_sense(self, ui: &mut Ui, sense: Sense) -> Response {
            self.response.set_sense(ui, sense)
        }
    }
}
