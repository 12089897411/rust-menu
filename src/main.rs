use eframe::egui::{Id, Widget};

use sense::SenseExt;

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
            ui.with_layout(
                eframe::egui::Layout::top_down(eframe::emath::Align::Min),
                |ui| {
                    let response = eframe::egui::TextEdit::multiline(&mut self.text)
                        .desired_width(f32::INFINITY)
                        .show(ui)
                        .response;

                    response.md_context_menu(|ui| {
                        println!("????");
                        // if ui
                        //     .menu_item_icon_and_text(
                        //         "Cut".into(),
                        //         "✂",
                        //         vec![eframe::egui::Key::A],
                        //         true,
                        //     )
                        //     .clicked()
                        // {};
                        ui.sub_menu_item_icon_and_text("Copy ", "📋", true, |ui| {
                            if ui
                                .menu_item_with_text("Copy Body", vec![eframe::egui::Key::C], true)
                                .clicked()
                            {
                                println!("1");
                            }
                            if ui
                                .menu_item_with_text("Copy Right", vec![eframe::egui::Key::C], true)
                                .clicked()
                            {}
                            if ui
                                .menu_item_with_text("Copy dd", vec![eframe::egui::Key::C], true)
                                .clicked()
                            {}
                        });
                        ui.sub_menu_item_icon_and_text("Save", "☺", true, |ui| {
                            if ui
                                .menu_item_with_text("Save Body", vec![eframe::egui::Key::C], true)
                                .clicked()
                            {}
                            if ui
                                .menu_item_with_text("Save Right", vec![eframe::egui::Key::C], true)
                                .clicked()
                            {}
                            if ui
                                .menu_item_with_text("Save dd", vec![eframe::egui::Key::C], true)
                                .clicked()
                            {}
                        });
                        if ui
                            .menu_item_icon_and_text(
                                "Delete".into(),
                                "📋",
                                vec![eframe::egui::Key::A],
                                true,
                            )
                            .clicked()
                        {};
                    });

                    ui.spacing();
                    ui.separator();

                    let resp = eframe::egui::TextEdit::multiline(&mut self.text)
                        .desired_width(f32::INFINITY)
                        .show(ui)
                        .response;

                    resp.md_context_menu(|ui| {
                        if ui
                            .menu_item_icon_and_text(
                                "Run1".into(),
                                "✂",
                                vec![eframe::egui::Key::A],
                                true,
                            )
                            .clicked()
                        {};
                        if ui
                            .menu_item_icon_and_text(
                                "Debug".into(),
                                "📋",
                                vec![eframe::egui::Key::A],
                                true,
                            )
                            .clicked()
                        {};
                        ui.sub_menu_item_icon_and_text("NO ", "📋", true, |ui| {
                            if ui
                                .menu_item_with_text("NO Body", vec![eframe::egui::Key::C], true)
                                .clicked()
                            {
                                println!("1");
                            }
                            if ui
                                .menu_item_with_text("NO Right", vec![eframe::egui::Key::C], true)
                                .clicked()
                            {}
                            if ui
                                .menu_item_with_text("NO dd", vec![eframe::egui::Key::C], true)
                                .clicked()
                            {}
                        });
                    });
                },
            )
            // response.context_menu(|ui| {
            //     ui.menu_button("O", |ui| ui.button("1"));
            //     ui.menu_button("B", |ui| ui.button("2"));
            // });
        });
        eframe::egui::TopBottomPanel::bottom("id").show(ctx, |ui| {});
    }
}

pub struct ContextMenuBuilder {
    id: eframe::egui::Id,
}
impl ContextMenuBuilder {
    pub fn new(id: eframe::egui::Id) -> Self {
        Self { id }
    }
}
impl ContextMenuBuilder {
    pub fn ui<'c, R>(
        self,
        response: &eframe::egui::Response,
        add_contents: impl FnOnce(&mut eframe::egui::Ui) -> R + 'c,
    ) -> Option<eframe::egui::InnerResponse<R>> {
        let ContextMenuBuilder { id } = self;
        let max_size = eframe::egui::vec2(300., 0.);
        let fill = eframe::egui::Color32::from_rgb(23, 29, 30);
        if response.secondary_clicked() {
            response.ctx.close_context_menu();
            let pos = if let Some(pos) = response.mouse_pos() {
                if response.rect.width() - pos.x > max_size.x {
                    Some(pos)
                } else {
                    Some(eframe::egui::Pos2::new(pos.x - 300., pos.y))
                }
            } else {
                None
            };
            response.ctx.push_menu_pos(pos)
        }

        if let Some(Some(pos2)) = response.ctx.get_menu_pos() {
            if response.rect.contains(pos2) {
                let resp = eframe::egui::Area::new(response.id.with(id))
                    .fixed_pos(pos2)
                    .interactable(true)
                    .order(eframe::egui::Order::Foreground)
                    .show(&response.ctx, |ui| {
                        ui.set_max_size(max_size);
                        eframe::egui::Frame {
                            inner_margin: eframe::egui::Margin::same(5.),
                            outer_margin: eframe::egui::Margin::same(0.),
                            rounding: eframe::egui::Rounding::same(10.),
                            shadow: eframe::epaint::Shadow::NONE,
                            fill,
                            stroke: eframe::egui::Stroke::NONE,
                        }
                        .show(ui, |ui| {
                            ui.with_layout(
                                eframe::egui::Layout::top_down(eframe::emath::Align::Center),
                                add_contents,
                            )
                            .inner
                        })
                    });
                if resp.response.clicked_elsewhere() {
                    response.ctx.close_context_menu();
                }
                Some(resp.inner)
            } else {
                None
            }
        } else {
            None
        }
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
    pub fn shortcut(mut self, shortcut: Vec<eframe::egui::Key>) -> Self {
        self.shortcut = shortcut;

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

pub struct SubMenuItemBuilder {
    label: Option<String>,
    icon: Option<String>,

    enable: bool,
}

impl SubMenuItemBuilder {
    pub fn new(label: Option<String>, icon: Option<String>, enable: bool) -> Self {
        Self {
            label,
            icon,
            enable,
        }
    }
    pub fn icon(icon: Option<String>, enable: bool) -> Self {
        Self {
            label: None,
            icon,
            enable,
        }
    }
    pub fn text(label: Option<String>, enable: bool) -> Self {
        Self {
            label,
            icon: None,
            enable,
        }
    }
}
impl SubMenuItemBuilder {
    fn show(
        self,
        ui: &mut eframe::egui::Ui,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::InnerResponse<Option<eframe::egui::InnerResponse<()>>> {
        let SubMenuItemBuilder {
            label,
            icon,
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

                            if let Some(text) = &label {
                                ui.label(text);
                            }
                            ui.with_layout(
                                eframe::egui::Layout::right_to_left(eframe::emath::Align::Center),
                                |ui| {
                                    ui.label("⏵");
                                },
                            );
                        },
                    );
                })
            })
            .set_sense(ui, eframe::egui::Sense::click());
        let sub_menu_id = resp.id.with("sub_menu");

        let state = ui
            .ctx()
            .get_sub_menu_rect()
            .and_then(|(id, rect)| {
                if id == sub_menu_id {
                    rect.expand(5.).check_pos(ui)
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let response = if resp.hovered() || state {
            ui.painter().rect(
                resp.rect,
                eframe::egui::Rounding::same(5.),
                eframe::egui::Color32::from_rgba_premultiplied(23, 29, 30, 10),
                eframe::egui::Stroke::NONE,
            );
            let max_size = eframe::egui::vec2(300., 0.);
            let pos = if let Some(Some(pos)) = ui.ctx().get_menu_pos() {
                if ui.ctx().screen_rect().width() - pos.x - resp.rect.width() > max_size.x {
                    [resp.rect.right_top().x, resp.rect.right_top().y - 5.]
                } else {
                    [resp.rect.left_top().x - 300., resp.rect.left_top().y - 5.]
                }
            } else {
                [0., 0.]
            };

            let resp = eframe::egui::Area::new(sub_menu_id)
                .fixed_pos(pos)
                .interactable(true)
                .order(eframe::egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    ui.allocate_ui([300., 0.].into(), |ui| {
                        eframe::egui::Frame {
                            inner_margin: eframe::egui::Margin::same(5.),
                            outer_margin: eframe::egui::Margin::same(0.),
                            rounding: eframe::egui::Rounding::same(10.),
                            shadow: eframe::epaint::Shadow::NONE,
                            fill: eframe::egui::Color32::from_rgb(23, 29, 30),
                            stroke: eframe::egui::Stroke::NONE,
                        }
                        .show(ui, |ui| {
                            ui.with_layout(
                                eframe::egui::Layout::top_down_justified(
                                    eframe::emath::Align::LEFT,
                                ),
                                add_contents,
                            )
                            .inner
                        })
                        .inner
                    })
                    .inner
                });
            ui.ctx()
                .push_sub_menu_rect((sub_menu_id, resp.response.rect));
            Some(resp)
        } else {
            None
        };
        eframe::egui::InnerResponse::new(response, resp)
    }
}

pub trait MenuItem {
    fn menu_item_icon_and_text(
        &mut self,
        text: &str,
        icon: &str,
        shortcut: Vec<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response;

    fn menu_item_with_text(
        &mut self,
        text: &str,
        shortcut: Vec<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response;
    fn menu_item_with_icon(
        &mut self,
        icon: &str,
        shortcut: Vec<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response;
    fn sub_menu_item_icon_and_text(
        &mut self,
        text: &str,
        icon: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::InnerResponse<Option<eframe::egui::InnerResponse<()>>>;
    fn sub_menu_item_with_text(
        &mut self,
        text: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::InnerResponse<Option<eframe::egui::InnerResponse<()>>>;
    fn sub_menu_item_with_icon(
        &mut self,
        icon: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::InnerResponse<Option<eframe::egui::InnerResponse<()>>>;
    // fn checkbox_menu_item(
    //     &mut self,
    //     icon: &str,
    //     text: &str,
    //     select: bool,
    //     enable: bool,
    //     add_contents: impl FnOnce(&mut eframe::egui::Ui),
    // ) -> eframe::egui::Response;
    // fn checkbox_icon_item(
    //     &mut self,
    //     icon: &str,
    //     select: bool,
    //     enable: bool,
    //     add_contents: impl FnOnce(&mut eframe::egui::Ui),
    // ) -> eframe::egui::Response;
    // fn checkbox_text_item(
    //     &mut self,
    //     text: &str,
    //     select: bool,
    //     enable: bool,
    //     add_contents: impl FnOnce(&mut eframe::egui::Ui),
    // ) -> eframe::egui::Response;
}
impl MenuItem for eframe::egui::Ui {
    fn menu_item_icon_and_text(
        &mut self,
        text: &str,
        icon: &str,
        shortcut: Vec<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response {
        MenuItemBuilder::new(Some(text.into()), Some(icon.into()), enable)
            .shortcut(shortcut)
            .ui(self)
    }

    fn menu_item_with_text(
        &mut self,
        text: &str,
        shortcut: Vec<eframe::egui::Key>,
        enable: bool,
    ) -> eframe::egui::Response {
        MenuItemBuilder::text(text.into(), enable)
            .shortcut(shortcut)
            .ui(self)
    }
    fn menu_item_with_icon(
        &mut self,
        icon: &str,
        shortcut: Vec<eframe::egui::Key>,
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
    ) -> eframe::egui::InnerResponse<Option<eframe::egui::InnerResponse<()>>> {
        SubMenuItemBuilder::new(Some(text.into()), Some(icon.into()), enable)
            .show(self, add_contents)
    }
    fn sub_menu_item_with_text(
        &mut self,
        text: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::InnerResponse<Option<eframe::egui::InnerResponse<()>>> {
        SubMenuItemBuilder::text(Some(text.into()), enable).show(self, add_contents)
    }
    fn sub_menu_item_with_icon(
        &mut self,
        icon: &str,
        enable: bool,
        add_contents: impl FnOnce(&mut eframe::egui::Ui),
    ) -> eframe::egui::InnerResponse<Option<eframe::egui::InnerResponse<()>>> {
        SubMenuItemBuilder::icon(Some(icon.into()), enable).show(self, add_contents)
    }
}

pub trait CloseMenuExt {
    fn close_context_menu(&self);
}
impl CloseMenuExt for eframe::egui::Context {
    fn close_context_menu(&self) {
        self.clear_sub_menu_rect();
        self.clear_menu_pos();
    }
}

pub trait MenuUIExt {
    fn push_menu_pos(&self, pos2: Option<eframe::epaint::Pos2>);
    fn get_menu_pos(&self) -> Option<Option<eframe::epaint::Pos2>>;

    fn push_sub_menu_rect(&self, rect: (eframe::egui::Id, eframe::egui::Rect));
    fn get_sub_menu_rect(&self) -> Option<(eframe::egui::Id, eframe::egui::Rect)>;

    fn clear_menu_pos(&self);
    fn clear_sub_menu_rect(&self);
}
impl MenuUIExt for eframe::egui::Context {
    fn push_menu_pos(&self, pos2: Option<eframe::epaint::Pos2>) {
        self.memory_mut(|f| f.data.insert_temp("my_context_menu".into(), pos2))
    }

    fn get_menu_pos(&self) -> Option<Option<eframe::epaint::Pos2>> {
        self.memory(|f| f.data.get_temp("my_context_menu".into()))
    }

    fn push_sub_menu_rect(&self, id_rect: (eframe::egui::Id, eframe::egui::Rect)) {
        self.memory_mut(|f| f.data.insert_temp("sub_menu_rect".into(), id_rect))
    }

    fn get_sub_menu_rect(&self) -> Option<(eframe::egui::Id, eframe::egui::Rect)> {
        self.memory(|f| f.data.get_temp("sub_menu_rect".into()))
    }

    fn clear_menu_pos(&self) {
        self.memory_mut(|f| {
            f.data
                .remove::<Option<eframe::egui::Pos2>>("my_context_menu".into())
        })
    }

    fn clear_sub_menu_rect(&self) {
        self.memory_mut(|f| f.data.remove::<eframe::egui::Rect>("sub_menu_rect".into()))
    }
}
pub trait ContextMenuExt {
    fn md_context_menu<R>(
        self,
        add_contents: impl FnOnce(&mut eframe::egui::Ui) -> R,
    ) -> Option<eframe::egui::InnerResponse<R>>;
}
impl ContextMenuExt for eframe::egui::Response {
    fn md_context_menu<R>(
        self,
        add_contents: impl FnOnce(&mut eframe::egui::Ui) -> R,
    ) -> Option<eframe::egui::InnerResponse<R>> {
        ContextMenuBuilder::new("my_context_menu".into()).ui(&self, add_contents)
    }
}

pub trait PosExt {
    fn mouse_pos(&self) -> Option<eframe::epaint::Pos2>;
}
impl PosExt for eframe::egui::Response {
    fn mouse_pos(&self) -> Option<eframe::epaint::Pos2> {
        self.ctx.input(|f| f.pointer.latest_pos())
    }
}

impl PosExt for eframe::egui::Ui {
    fn mouse_pos(&self) -> Option<eframe::epaint::Pos2> {
        self.input(|f| f.pointer.latest_pos())
    }
}
pub trait RectExt {
    fn check_pos(&self, ui: &mut eframe::egui::Ui) -> Option<bool>;
}
impl RectExt for eframe::egui::Rect {
    fn check_pos(&self, ui: &mut eframe::egui::Ui) -> Option<bool> {
        ui.mouse_pos().map(|pos| self.contains(pos))
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
