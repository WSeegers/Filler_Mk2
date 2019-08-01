use conrod::{widget, Widget, Labelable, Positionable, Sizeable, color, Colorable, Borderable};

use nfd::Response;

widget_ids!(struct Ids {
    canvas,
    p1_title,
    p2_title,
    btn_p1_select,
    btn_p2_select,
});

pub struct State {
    ids: Ids,
}

#[derive(WidgetCommon)]
pub struct PlayerSelect {
    #[conrod(common_builder)] common: widget::CommonBuilder,
}

impl PlayerSelect {
    pub fn new() -> Self {
        Self {
            common: widget::CommonBuilder::default(),
        }
    }
}

impl Widget for PlayerSelect {
    type State = State;
    type Style = ();
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {}

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            state,
            ui,
            id,
            ..
        } = args;

        widget::Canvas::new()
            .parent(id)
            .color(color::DARK_CHARCOAL)
            .border(0.0)
            .wh_of(id)
            .set(state.ids.canvas, ui);

        widget::Text::new("Player 1")
            .parent(state.ids.canvas)
            .x_y(-250.0, 200.0)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(state.ids.p1_title, ui);

        for _click in widget::Button::new()
            .parent(state.ids.p1_title)
            .x_y(-250.0, 100.0)
            .w_h(250.0, 50.0)
            .label("Select Player")
            .set(state.ids.btn_p1_select, ui)
            {
                let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
                    panic!(e);
                });

                match result {
                    Response::Okay(file_path) => println!("File path = {:?}", file_path),
                    Response::OkayMultiple(files) => println!("Files {:?}", files),
                    Response::Cancel => println!("User canceled"),
                }
            }

        widget::Text::new("Player 2")
            .parent(state.ids.canvas)
            .x_y(250.0, 200.0)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(state.ids.p2_title, ui);
        
        for _click in widget::Button::new()
            .parent(state.ids.p2_title)
            .x_y(250.0, 100.0)
            .w_h(250.0, 50.0)
            .label("Select Player")
            .set(state.ids.btn_p2_select, ui)
            {
                let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
                    panic!(e);
                });

                match result {
                    Response::Okay(file_path) => println!("File path = {:?}", file_path),
                    Response::OkayMultiple(files) => println!("Files {:?}", files),
                    Response::Cancel => println!("User canceled"),
                }
            }
    }
}
