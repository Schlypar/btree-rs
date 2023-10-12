mod btree;
pub mod db;

pub use db::{
    goods::{Crate, Person},
    DataBase, Key,
};

pub use eframe::run_native;
use eframe::{
    egui::{
        Button, CentralPanel, Direction, FontData, FontDefinitions, Label, Layout, RadioButton,
        RichText, ScrollArea, Separator, SidePanel, Slider, TextStyle, TopBottomPanel, Ui, Visuals,
    },
    emath::Align,
    epaint::{Color32, FontFamily, FontId},
    App, CreationContext,
};

use std::{
    fs::File,
    path::Path,
    time::{Duration, Instant},
};

use self::db::{file_handler::create_dir_all, From, KeyType, Random};

static mut FILE_PATH: Option<String> = None;

const DEFAULT_LIMIT: u64 = 50;

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const MAGENTA: Color32 = Color32::from_rgb(226, 0, 122);
const RED: Color32 = Color32::from_rgb(255, 0, 0);
const ORANGE: Color32 = Color32::from_rgb(255, 127, 80);
const GOLD: Color32 = Color32::from_rgb(212, 175, 55);

const PADDING: f32 = 5.;

#[derive(Debug, PartialEq, Eq)]
enum IndexState {
    Indexed(KeyType),
    NotIndexed,
}

#[derive(Debug, Clone)]
struct TimeMeasure {
    indexed: Duration,
    unindexed: Vec<Duration>,
}

#[derive(Debug)]
pub struct Application<'a> {
    data_base: Option<DataBase<'a, Crate>>,
    index_state: IndexState,
    data_limit: u64,
    changed: bool,
    is_opened: bool,
    search_key: Option<u64>,
    search_key_str: String,
    crates: Vec<(Crate, Option<(TimeMeasure, u64)>)>,
}

impl<'a> Application<'a> {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        unsafe { FILE_PATH = Some(String::default()) };

        Self {
            data_base: None,
            index_state: IndexState::NotIndexed,
            data_limit: DEFAULT_LIMIT,
            changed: true,
            is_opened: false,
            search_key: None,
            search_key_str: String::default(),
            crates: Vec::new(),
        }
    }
}

impl<'a> App for Application<'a> {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.configure_font(ctx);

        self.top_panel(ctx);
        self.footer(ctx);

        if !self.is_opened {
            CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(PADDING * 30.);
                    let enter_path = RichText::new("Enter path to the Database down below 󱞣 ")
                        .color(WHITE)
                        .text_style(TextStyle::Heading);
                    ui.label(enter_path);
                    ui.add_space(PADDING);

                    ui.text_edit_singleline(unsafe { FILE_PATH.as_mut().unwrap() });

                    if unsafe { FILE_PATH.as_ref().unwrap().is_empty() } {
                        let will_error = RichText::new("Path is empty. Error will occure")
                            .color(RED)
                            .size(9.);
                        ui.label(will_error);
                    } else if Path::new(unsafe { FILE_PATH.as_ref().unwrap() }).exists() {
                        let can_open = RichText::new("Path exists. Will try to open")
                            .color(CYAN)
                            .size(9.);
                        ui.label(can_open);
                    } else {
                        let can_create =
                            RichText::new("Path doesn't exist. Will create it automagically")
                                .color(ORANGE)
                                .size(9.);
                        ui.label(can_create);
                    }

                    ui.horizontal_top(|ui| {
                        let cancel_btn = Button::new("Cancel");
                        let approve_btn = Button::new("Yas");

                        ui.label("                                          ");
                        // ui.label("                                          ");
                        // ui.label("                          ");

                        if ui.add(cancel_btn).clicked() {
                            std::process::exit(0);
                        }

                        if ui.add(approve_btn).clicked() {
                            let db_path = Path::new(unsafe { FILE_PATH.as_ref().unwrap() });
                            self.data_base = if db_path.exists() {
                                Some(DataBase::new(db_path.into()).unwrap())
                            } else {
                                let mut nested_path: Vec<&str> =
                                    self.search_key_str.split('/').collect();
                                nested_path.pop();
                                let mut curr_path: String = String::default();
                                nested_path.iter().for_each(|dir| {
                                    curr_path.push_str(dir);
                                    curr_path.push('/');
                                });
                                if let Err(e) = create_dir_all(curr_path) {
                                    eprintln!("Error opening Database: {:?}", e);
                                }
                                if let Err(e) = File::create(db_path) {
                                    eprintln!("Error creating file: {:?}\nError: {:?}", db_path, e);
                                }
                                Some(DataBase::new(db_path.into()).unwrap())
                            };

                            self.is_opened = true;
                        }
                    });
                });
            });
        }

        if self.is_opened {
            SidePanel::left("Options panel").show(ctx, |ui| {
                self.render_index_type(ui);
                self.render_crates_limit(ui);
                self.render_db_options(ui);
            });

            CentralPanel::default().show(ctx, |ui| {
                self.header(ui);
                ui.vertical_centered(|ui| {
                    self.parse_input();
                    ScrollArea::new([false, true]).show(ui, |ui| {
                        if self.search_key.is_some()
                            && matches!(self.index_state, IndexState::Indexed(_))
                        {
                            // will bother with actually searching here
                            self.render_with_measure(ui);
                        } else {
                            // wont bother with actually searching here
                            if self.changed {
                                self.crates.clear();
                                self.crates.reserve_exact(self.data_limit as usize);
                                let db = self.data_base.as_mut().unwrap();
                                for i in 0..self.data_limit {
                                    if let Ok(data) = db.peek(i) {
                                        self.crates.push((data, None));
                                    }
                                }
                                self.changed = false;
                            }
                            self.render_crates(ui);
                        }
                    });
                });
            });
        }
    }
}

impl<'a> Application<'a> {
    fn top_panel(&mut self, ctx: &eframe::egui::Context) {
        TopBottomPanel::top("Top panel").show(ctx, |ui| {
            eframe::egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.add(Label::new("DrevniyRus"));
                });

                ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                    ui.heading(RichText::new("DataBase insider application"));
                });

                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    let close_btn = ui.add(Button::new(
                        RichText::new("  ").text_style(TextStyle::Heading),
                    ));
                    if close_btn.clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });
    }

    fn parse_input(&mut self) {
        if let Ok(num) = self.search_key_str.trim().parse::<u64>() {
            match self.search_key {
                Some(key) => {
                    if key != num {
                        self.changed = true;
                    }
                }
                None => self.changed = true,
            }
            self.search_key = Some(num);
        } else {
            if matches!(self.search_key, Some(_)) {
                self.changed = true;
            }
            self.search_key = None;
        }
    }

    fn header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("󰳶 NKOHA Trasportation").heading().color(GOLD));
            ui.text_edit_singleline(&mut self.search_key_str);
        });
    }

    fn footer(&mut self, ctx: &eframe::egui::Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(PADDING);
                ui.hyperlink("https://github.com/Schlypar/btree-rs");
                ui.add_space(PADDING);
            });
        });
    }

    fn render_db_options(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.colored_label(RED, RichText::new("Database options").underline().size(15.));
        ui.add_space(PADDING);

        ui.label(
            RichText::new(
                "From this options you can either delete entire db or wait a little and generate thousands upon thousands of random data.",
            )
            .size(13.)
            .color(WHITE)
            .weak(),
        );

        ui.add_space(PADDING);
        if ui
            .add(Button::new(RichText::new("Clean Database")))
            .clicked()
        {
            let db = self.data_base.as_mut().unwrap();
            match db.clean() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error cleaning Database: {:?}", e);
                }
            }
            self.changed = true;
            match db.clean() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error occured while cleaning Database: {:?}", e);
                }
            }
        }

        ui.add_space(PADDING);
        if ui
            .add(Button::new(RichText::new("Generate data")))
            .clicked()
        {
            let db = self.data_base.as_mut().unwrap();
            self.changed = true;
            match db.clean() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error occured while cleaning Database: {:?}", e);
                }
            }

            (0..DEFAULT_LIMIT * DEFAULT_LIMIT * DEFAULT_LIMIT).for_each(|_| {
                match db.add_record(Crate::random()) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error adding record to Database: {:?}", e);
                        panic!()
                    }
                }
            });
        }

        ui.add_space(PADDING);
        ui.add(Separator::default());
    }

    fn render_index_type(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.colored_label(RED, RichText::new("Indexing options").underline().size(15.));
        ui.add_space(PADDING);

        ui.label(
            RichText::new(
                "Here lies indexing options. At any given moment there can be only one index field or none at all. It'll take some time!",
            )
            .size(13.)
            .color(WHITE)
            .weak(),
        );

        ui.radio_value(
            &mut self.index_state,
            IndexState::NotIndexed,
            "Disable index",
        );

        if ui
            .add(RadioButton::new(
                self.index_state == IndexState::Indexed(KeyType::GoodsID),
                "Goods ID index",
            ))
            .clicked()
        {
            self.index_state = IndexState::Indexed(KeyType::GoodsID);
            if let IndexState::Indexed(key_type) = self.index_state {
                let db = self.data_base.as_mut().unwrap();
                db.index(key_type).unwrap_or_else(|e| {
                    eprintln!("Error indexing database by {:?}: {:?}", key_type, e)
                });
            }
        }

        if ui
            .add(RadioButton::new(
                self.index_state == IndexState::Indexed(KeyType::PostIndex(From::Sender)),
                "Sender post index index",
            ))
            .clicked()
        {
            self.index_state = IndexState::Indexed(KeyType::PostIndex(From::Sender));
            if let IndexState::Indexed(key_type) = self.index_state {
                let db = self.data_base.as_mut().unwrap();
                db.index(key_type).unwrap_or_else(|e| {
                    eprintln!("Error indexing database by {:?}: {:?}", key_type, e)
                });
            }
        }

        if ui
            .add(RadioButton::new(
                self.index_state == IndexState::Indexed(KeyType::PostIndex(From::Receiver)),
                "Receiver post index index",
            ))
            .clicked()
        {
            self.index_state = IndexState::Indexed(KeyType::PostIndex(From::Receiver));
            if let IndexState::Indexed(key_type) = self.index_state {
                let db = self.data_base.as_mut().unwrap();
                db.index(key_type).unwrap_or_else(|e| {
                    eprintln!("Error indexing database by {:?}: {:?}", key_type, e)
                });
            }
        }
        ui.add_space(PADDING);
        ui.add(Separator::default());
    }

    fn render_crates_limit(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.colored_label(RED, RichText::new("Crates limit").underline().size(15.));
        ui.add_space(PADDING);
        ui.label(
            RichText::new("How much crates to show")
                .size(13.)
                .color(WHITE)
                .weak(),
        );
        let old_data_limit = self.data_limit;
        ui.add(
            Slider::new(&mut self.data_limit, 1..=500)
                .text(RichText::new("n ∈ [1; 500]").weak().size(9.)),
        );
        if old_data_limit != self.data_limit {
            self.changed = true;
        }
        ui.add(Separator::default());
    }

    fn render_crate(data: &Crate, ui: &mut Ui, time_measure: Option<(TimeMeasure, u64)>) {
        ui.vertical(|ui| {
            let crate_name = format!("󰵉 {} {}", data.goods_name, data.goods_id);
            let producer = format!("󰾁 {}", data.producer);
            let sender = format!(
                " From: {} {} {}, {}",
                data.sender.name,
                data.sender.surname,
                data.sender.patronymic,
                data.sender.post_index
            );
            let receiver = format!(
                " To: {} {} {}, {}",
                data.receiver.name,
                data.receiver.surname,
                data.receiver.patronymic,
                data.receiver.post_index
            );

            ui.add_space(PADDING);
            ui.add(Separator::default());

            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                ui.colored_label(
                    MAGENTA,
                    RichText::new(crate_name).size(15.0).heading().underline(),
                );
                ui.label(
                    RichText::new(producer)
                        .weak()
                        .size(9.)
                        .text_style(TextStyle::Button),
                );
            });

            ui.add_space(PADDING);
            ui.colored_label(WHITE, sender);
            ui.colored_label(WHITE, receiver);
            ui.add_space(PADDING);

            let read_more = Label::new(
                RichText::new("Point at me to read more 󱞣 ")
                    .color(CYAN)
                    .size(11.)
                    .italics(),
            );
            if ui.add(read_more).hovered() {
                if let Some((time_measure, at)) = time_measure {
                    // let (time_measure, at) = time_measure.unwrap();
                    let (ref indexed, ref unindexed) =
                        (time_measure.indexed, time_measure.unindexed[at as usize]);
                    ui.vertical_centered(|ui| {
                        ui.add(Separator::default());
                        ui.add_space(PADDING);

                        let indexed_nanos = indexed.as_nanos();
                        let unindexed_nanos = unindexed.as_nanos();

                        let indexed_time =
                            format!("Time needed to find this crate using BTree {:?}", indexed);
                        let unindexed_time = format!(
                            "Time needed to find this crate by searching all file {:?}",
                            unindexed
                        );
                        let blazingly_joke = format!(
                            "\"Blazingly Faster\" Coefficient was {:.3}",
                            unindexed_nanos as f64 / indexed_nanos as f64
                        );

                        ui.label(RichText::new(indexed_time));
                        ui.label(RichText::new(unindexed_time));
                        ui.label(RichText::new(blazingly_joke).size(13.).color(MAGENTA));

                        ui.add_space(PADDING);
                        ui.add(Separator::default());
                    });
                } else {
                    ui.label(RichText::new("There is nothing to see here  ").color(RED));
                }
            }
        });
    }

    fn render_with_measure(&mut self, ui: &mut Ui) {
        if self.changed {
            let (time_measure, poss) = self.measure_and_give();
            if let Some(poss) = poss {
                // let poss = poss.unwrap();
                self.crates.clear();
                self.crates.reserve_exact(poss.len());

                for (index, pos) in (0..self.data_limit).zip(poss) {
                    let db = self.data_base.as_mut().unwrap();
                    if let Ok(data) = db.peek(pos) {
                        self.crates
                            .push((data, Some((time_measure.clone(), index))));
                    }
                }
                self.changed = false;
            } else {
                ui.vertical(|ui| {
                    ui.add_space(PADDING * 2.);
                    ui.label(
                        RichText::new("Nothing was found, absolutely nothing... 󰇸 ")
                            .heading()
                            .underline(),
                    );
                });
            }
        } else {
            self.render_crates(ui);
        }
    }

    fn render_crates(&mut self, ui: &mut Ui) {
        self.crates.iter().for_each(|(data, measurement)| {
            Application::render_crate(data, ui, measurement.clone());
        });
    }

    fn configure_font(&self, ctx: &eframe::egui::Context) {
        let mut font_def = FontDefinitions::default();
        let font_data = FontData::from_owned(
            include_bytes!("/usr/share/fonts/TTF/FiraCodeNerdFontPropo-Bold.ttf").to_vec(),
        );
        font_def.font_data.insert("Fira".to_string(), font_data);

        let font_data =
            FontData::from_owned(include_bytes!("/usr/share/fonts/ubuntu/Ubuntu-L.ttf").to_vec());
        font_def.font_data.insert("Ubuntu".to_string(), font_data);

        let font_data = FontData::from_owned(
            include_bytes!("/usr/share/fonts/TTF/CaskaydiaCoveNerdFontMono-Regular.ttf").to_vec(),
        );
        font_def
            .font_data
            .insert("Caskaydia".to_string(), font_data);

        font_def
            .families
            .insert(FontFamily::Proportional, vec!["Fira".to_string()]);

        font_def
            .families
            .insert(FontFamily::Monospace, vec!["Caskaydia".to_string()]);

        eframe::egui::style::default_text_styles().insert(
            TextStyle::Heading,
            FontId::new(35., FontFamily::Proportional),
        );
        eframe::egui::style::default_text_styles()
            .insert(TextStyle::Body, FontId::new(35., FontFamily::Proportional));
        eframe::egui::style::default_text_styles()
            .insert(TextStyle::Small, FontId::new(35., FontFamily::Proportional));
        eframe::egui::style::default_text_styles().insert(
            TextStyle::Monospace,
            FontId::new(35., FontFamily::Monospace),
        );

        ctx.set_fonts(font_def);
    }

    fn measure_and_give(&mut self) -> (TimeMeasure, Option<Vec<u64>>) {
        let key = self.search_key.unwrap();
        let mut poss: Vec<u64> = Vec::new();

        let mut unindexed: Vec<Duration> = Vec::new();
        let start = Instant::now();
        let db = self.data_base.as_mut().unwrap();
        for i in 0..db.len() {
            if let Ok(data) = db.peek(i as u64) {
                match &self.index_state {
                    IndexState::Indexed(key_type) => match key_type {
                        KeyType::GoodsID => {
                            if data.goods_id == key {
                                unindexed.push(start.elapsed());
                                poss.push(i as u64);
                            }
                        }
                        KeyType::PostIndex(From::Sender) => {
                            if data.sender.post_index == key as u32 {
                                unindexed.push(start.elapsed());
                                poss.push(i as u64);
                            }
                        }
                        KeyType::PostIndex(From::Receiver) => {
                            if data.receiver.post_index == key as u32 {
                                unindexed.push(start.elapsed());
                                poss.push(i as u64);
                            }
                        }
                    },
                    IndexState::NotIndexed => panic!(),
                }
            }
        }

        let poss = if poss.is_empty() { None } else { Some(poss) };

        let start = Instant::now();
        let poss = match self.index_state {
            IndexState::Indexed(key_type) => match key_type {
                KeyType::GoodsID => {
                    let db = self.data_base.as_mut().unwrap();
                    db.search_indexed(Key::GoodsID(key), None)
                }
                KeyType::PostIndex(from) => {
                    let db = self.data_base.as_mut().unwrap();
                    db.search_indexed(Key::PostIndex(key as u32), Some(from))
                }
            },
            IndexState::NotIndexed => poss,
        };
        let indexed = start.elapsed();

        (TimeMeasure { indexed, unindexed }, poss)
    }
}
