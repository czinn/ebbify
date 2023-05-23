// Adapated from the DragValue widget.
use egui::*;

/// Same state for all [`ValidatedTextEdit`]s.
#[derive(Clone, Debug, Default)]
pub(crate) struct MonoState {
    /// For temporary edit of a [`ValidatedTextEdit`] value.
    /// Couples with the current focus id.
    edit_string: Option<String>,
}

type Formatter<'a, T> = Box<dyn 'a + Fn(&T) -> String>;
type Parser<'a, T> = Box<dyn 'a + Fn(&str, &T) -> Option<T>>;

/// Combined into one function (rather than two) to make it easier
/// for the borrow checker.
type GetSetValue<'a, T> = Box<dyn 'a + FnMut(Option<T>) -> T>;

fn get<'a, T>(get_set_value: &mut GetSetValue<'a, T>) -> T {
    (get_set_value)(None)
}

fn set<'a, T>(get_set_value: &mut GetSetValue<'a, T>, value: T) {
    (get_set_value)(Some(value));
}

/// A validated text entry for arbitrary types that reverts to the previous value if the text
/// cannot be parsed.
///
/// ```
/// ui.add(
///     egui::ValidatedTextEdit::new(&mut my_f32)
///         .display_formatter(|f: &f32| f.to_string())
///         .parser(|s: &str| s.parse().ok()),
///     );
/// ```
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct ValidatedTextEdit<'a, T> {
    get_set_value: GetSetValue<'a, T>,
    display_formatter: Option<Formatter<'a, T>>,
    edit_formatter: Option<Formatter<'a, T>>,
    parser: Option<Parser<'a, T>>,
}

impl<'a, T> ValidatedTextEdit<'a, T>
where
    T: std::cmp::PartialEq + Clone,
{
    pub fn new(value: &'a mut T) -> Self {
        Self::from_get_set(move |v: Option<T>| {
            if let Some(v) = v {
                *value = v;
            }
            value.clone()
        })
    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<T>) -> T) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            display_formatter: None,
            edit_formatter: None,
            parser: None,
        }
    }

    pub fn display_formatter(mut self, formatter: impl 'a + Fn(&T) -> String) -> Self {
        self.display_formatter = Some(Box::new(formatter));
        self
    }

    pub fn edit_formatter(mut self, formatter: impl 'a + Fn(&T) -> String) -> Self {
        self.edit_formatter = Some(Box::new(formatter));
        self
    }

    pub fn parser(mut self, parser: impl 'a + Fn(&str, &T) -> Option<T>) -> Self {
        self.parser = Some(Box::new(parser));
        self
    }
}

fn monostate(mem: &mut Memory) -> &mut MonoState {
    mem.data.get_temp_mut_or_default(Id::null())
}

impl<'a, T> Widget for ValidatedTextEdit<'a, T>
where
    T: std::cmp::PartialEq,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            mut get_set_value,
            display_formatter,
            edit_formatter,
            parser,
        } = self;

        // The widget has the same ID whether it's in edit or button mode.
        let id = ui.next_auto_id();

        let has_focus = ui.memory_mut(|mem| {
            mem.interested_in_focus(id);
            mem.has_focus(id)
        });

        let value = get(&mut get_set_value);

        let display_text = match display_formatter {
            Some(display_formatter) => display_formatter(&value),
            None => "Unknown value".to_owned(),
        };

        let text_style = ui.style().drag_value_text_style.clone();

        let mut response = if has_focus {
            let mut edit_text = ui
                .memory_mut(|mem| monostate(mem).edit_string.take())
                .unwrap_or_else(|| match edit_formatter {
                    Some(edit_formatter) => edit_formatter(&value),
                    None => display_text.clone(),
                });
            let response = ui.add(
                TextEdit::singleline(&mut edit_text)
                    .clip_text(false)
                    .horizontal_align(ui.layout().horizontal_align())
                    .vertical_align(ui.layout().vertical_align())
                    .margin(ui.spacing().button_padding)
                    .min_size(ui.spacing().interact_size)
                    .id(id)
                    .desired_width(ui.spacing().interact_size.x)
                    .font(text_style),
            );
            // Only update the value when the user presses enter, or clicks elsewhere. NOT every frame.
            if response.lost_focus() {
                let parsed_value = match parser {
                    Some(parser) => parser(&edit_text, &value),
                    None => None,
                };
                if let Some(parsed_value) = parsed_value {
                    set(&mut get_set_value, parsed_value);
                }
            }
            ui.memory_mut(|mem| monostate(mem).edit_string = Some(edit_text));
            response
        } else {
            let button = Button::new(RichText::new(&display_text).text_style(text_style))
                .wrap(false)
                .sense(Sense::click_and_drag())
                .min_size(ui.spacing().interact_size);

            let mut response = ui.add(button);

            if ui.style().explanation_tooltips {
                response =
                    response.on_hover_text(format!("{}\nClick to enter a value.", display_text,));
            }

            if response.clicked() {
                ui.memory_mut(|mem| {
                    monostate(mem).edit_string = None;
                    mem.request_focus(id);
                });
                let mut state = TextEdit::load_state(ui.ctx(), id).unwrap_or_default();
                state.set_ccursor_range(Some(text::CCursorRange::two(
                    epaint::text::cursor::CCursor::default(),
                    epaint::text::cursor::CCursor::new(display_text.chars().count()),
                )));
                state.store(ui.ctx(), response.id);
            }

            response
        };

        response.changed = get(&mut get_set_value) != value;

        response
    }
}
