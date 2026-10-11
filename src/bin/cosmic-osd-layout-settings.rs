//! Settings window for the keyboard layout indicator of cosmic-osd.

use cosmic::app::{Core, Settings, Task};
use cosmic::iced::{Color, Length, Size};
use cosmic::widget::color_picker::{ColorPickerModel, ColorPickerUpdate};
use cosmic::{Application, Element, widget};
use cosmic_config::{Config, ConfigGet, ConfigSet};

#[allow(dead_code)]
#[path = "../config.rs"]
mod config;

/// (config key, label, default, min, max)
const NUMS: [(&str, &str, i32, i32, i32); 3] = [
    ("font_size", "Размер текста", 20, 8, 96),
    ("margin_right", "Отступ справа", 48, 0, 400),
    ("margin_bottom", "Отступ снизу", 48, 0, 400),
];

#[derive(Clone, Debug)]
enum Msg {
    /// Slider moved: (index into NUMS, value)
    Num(usize, i32),
    /// Number typed: (index into NUMS, text)
    NumText(usize, String),
    TextColor(usize, String),
    BgColor(usize, String),
    /// Open the palette for (language index, is_background)
    OpenPicker(usize, bool),
    Picker(ColorPickerUpdate),
}

struct App {
    core: Core,
    config: Option<Config>,
    nums: [i32; 3],
    num_texts: [String; 3],
    /// per language: (text, bg) hex strings as typed
    colors: Vec<(String, String)>,
    picker: ColorPickerModel,
    editing: Option<(usize, bool)>,
}

impl App {
    fn save<T: serde::Serialize>(&self, key: &str, value: T) {
        if let Some(Err(why)) = self.config.as_ref().map(|c| c.set(key, value)) {
            eprintln!("could not save {key}: {why}");
        }
    }

    fn set_color(&mut self, i: usize, bg: bool, value: String) {
        let kind = if bg { "bg" } else { "text" };
        self.save_color(format!("{kind}_color_{}", config::LANGS[i]), &value);
        let slot = &mut self.colors[i];
        *(if bg { &mut slot.1 } else { &mut slot.0 }) = value;
    }

    /// Save only complete colors; "" clears back to the theme color.
    fn save_color(&self, key: String, value: &str) {
        if value.is_empty() || config::parse_hex(value).is_some() {
            self.save(&key, value.trim().to_owned());
        }
    }
}

impl Application for App {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Msg;
    const APP_ID: &'static str = "com.system76.CosmicOsdLayoutSettings";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _: ()) -> (Self, Task<Msg>) {
        let config = config::layout_config();
        let int = |key: &str, default| {
            config
                .as_ref()
                .and_then(|c| c.get::<i32>(key).ok())
                .unwrap_or(default)
        };
        let string = |key: String| {
            config
                .as_ref()
                .and_then(|c| c.get::<String>(&key).ok())
                .unwrap_or_default()
        };
        let colors = config::LANGS
            .iter()
            .map(|l| (string(format!("text_color_{l}")), string(format!("bg_color_{l}"))))
            .collect();
        let app = Self {
            core,
            nums: NUMS.map(|(key, _, default, min, max)| int(key, default).clamp(min, max)),
            num_texts: NUMS.map(|(key, _, default, min, max)| int(key, default).clamp(min, max).to_string()),
            colors,
            config,
            picker: new_picker(None),
            editing: None,
        };
        (app, Task::none())
    }

    fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::Num(i, v) => {
                self.nums[i] = v;
                self.num_texts[i] = v.to_string();
                self.save(NUMS[i].0, v);
            }
            Msg::NumText(i, text) => {
                // Keep whatever is typed; apply it once it is a valid number.
                if let Ok(v) = text.trim().parse::<i32>() {
                    let v = v.clamp(NUMS[i].3, NUMS[i].4);
                    self.nums[i] = v;
                    self.save(NUMS[i].0, v);
                }
                self.num_texts[i] = text;
            }
            Msg::TextColor(i, v) => self.set_color(i, false, v),
            Msg::BgColor(i, v) => self.set_color(i, true, v),
            Msg::OpenPicker(i, bg) => {
                let slot = &self.colors[i];
                let current = config::parse_hex(if bg { &slot.1 } else { &slot.0 });
                self.picker = new_picker(current).active(true);
                self.editing = Some((i, bg));
            }
            Msg::Picker(update) => {
                let applied = matches!(
                    update,
                    ColorPickerUpdate::AppliedColor | ColorPickerUpdate::ActionFinished
                );
                let cancelled = matches!(update, ColorPickerUpdate::Cancel);
                let task = self.picker.update::<cosmic::Action<Msg>>(update);
                if applied && let (Some((i, bg)), Some(c)) =
                    (self.editing, self.picker.get_applied_color())
                {
                    self.set_color(i, bg, to_hex(c));
                }
                if applied || cancelled {
                    self.editing = None;
                }
                return task;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Msg> {
        let mut section = widget::settings::section().title("Индикатор раскладки");
        for (i, (_, label, _, min, max)) in NUMS.iter().enumerate() {
            section = section.add(widget::settings::item(
                *label,
                widget::row::with_capacity(2)
                    .spacing(8)
                    .align_y(cosmic::iced::Alignment::Center)
                    .push(
                        widget::slider(*min..=*max, self.nums[i], move |v| Msg::Num(i, v))
                            .width(Length::Fixed(160.)),
                    )
                    .push(
                        widget::text_input("", &self.num_texts[i])
                            .on_input(move |t| Msg::NumText(i, t))
                            .width(Length::Fixed(64.)),
                    ),
            ));
        }
        let mut page = widget::column::with_capacity(5)
            .spacing(24)
            .padding(24)
            .push(section);

        for (i, lang) in config::LANGS.iter().enumerate() {
            let (text, bg) = &self.colors[i];
            let [tr, tg, tb, ta] = config::parse_hex(text).unwrap_or([1.; 4]);
            let bg_parsed = config::parse_hex(bg);
            let preview = widget::container(
                widget::text::title4(lang.to_uppercase())
                    .class(cosmic::theme::Text::Color(cosmic::iced::Color {
                        r: tr,
                        g: tg,
                        b: tb,
                        a: ta,
                    })),
            )
            .padding([8, 16])
            .class(cosmic::theme::Container::custom(move |_| {
                widget::container::Style {
                    background: bg_parsed
                        .map(|[r, g, b, a]| cosmic::iced::Color { r, g, b, a }.into()),
                    border: cosmic::iced::Border {
                        radius: 8.0.into(),
                        width: 1.0,
                        color: cosmic::iced::Color::from_rgba(0.5, 0.5, 0.5, 0.5),
                    },
                    ..Default::default()
                }
            }));
            page = page.push(
                widget::settings::section()
                    .title(format!("Цвета: {} (#RRGGBB, пусто — тема)", lang.to_uppercase()))
                    .add(widget::settings::item(
                        "Текст",
                        widget::row::with_capacity(2)
                            .spacing(8)
                            .push(
                                widget::text_input("#ffffff", text)
                                    .on_input(move |v| Msg::TextColor(i, v))
                                    .width(Length::Fixed(110.)),
                            )
                            .push(
                                widget::button::standard("Палитра")
                                    .on_press(Msg::OpenPicker(i, false)),
                            ),
                    ))
                    .add(widget::settings::item(
                        "Фон",
                        widget::row::with_capacity(2)
                            .spacing(8)
                            .push(
                                widget::text_input("#1a1a1a", bg)
                                    .on_input(move |v| Msg::BgColor(i, v))
                                    .width(Length::Fixed(110.)),
                            )
                            .push(
                                widget::button::standard("Палитра")
                                    .on_press(Msg::OpenPicker(i, true)),
                            ),
                    ))
                    .add(widget::settings::item("Предпросмотр", preview)),
            );
        }
        if let Some((i, bg)) = self.editing {
            let what = if bg { "фон" } else { "текст" };
            page = page.push(
                widget::settings::section()
                    .title(format!(
                        "Палитра: {} ({what})",
                        config::LANGS[i].to_uppercase()
                    ))
                    .add(
                        self.picker
                            .builder(Msg::Picker)
                            .save_label("Применить")
                            .cancel_label("Отмена")
                            .build("Недавние", "Копировать", "Скопировано"),
                    ),
            );
        }
        widget::scrollable(page).into()
    }
}

fn new_picker(current: Option<[f32; 4]>) -> ColorPickerModel {
    let initial = current.map_or(Color::WHITE, |[r, g, b, a]| Color { r, g, b, a });
    ColorPickerModel::new("HEX", "RGB", None, Some(initial))
}

fn to_hex(c: Color) -> String {
    let byte = |v: f32| (v.clamp(0., 1.) * 255.).round() as u8;
    format!("#{:02X}{:02X}{:02X}", byte(c.r), byte(c.g), byte(c.b))
}

fn main() -> cosmic::iced::Result {
    cosmic::app::run::<App>(
        Settings::default().size(Size::new(520., 720.)),
        (),
    )
}
