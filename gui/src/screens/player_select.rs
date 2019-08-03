use conrod::{color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

// use nfd::Response;

use fillercore::models::point::Point;
use crate::core::Screen;

widget_ids!(struct Ids {
    canvas,
    p1_title,
    p2_title,
    btn_p1_select,
    btn_p2_select,
    btn_start,
    map_size,
});

pub struct State {
    ids: Ids,
    map_size_start: conrod::Scalar,
    map_size_end: conrod::Scalar,
}

#[derive(WidgetCommon)]
pub struct PlayerSelect<'a> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    screen: &'a mut Screen,
    p1_path: &'a mut Option<String>,
    p2_path: &'a mut Option<String>,
    p1_start: &'a mut Point,
    p2_start: &'a mut Point,
    board_width: &'a mut u32,
    board_height: &'a mut u32,
}

impl<'a> PlayerSelect<'a> {
    pub fn new(screen: &'a mut Screen, p1_path: &'a mut Option<String>, p2_path: &'a mut Option<String>, p1_start: &'a mut Point, p2_start: &'a mut Point, board_width: &'a mut u32, board_height: &'a mut u32) -> Self {
        Self {
            common: widget::CommonBuilder::default(),
            screen,
            p1_path,
            p2_path,
            p1_start,
            p2_start,
            board_width,
            board_height,
        }
    }
}

impl<'a> Widget for PlayerSelect<'a> {
    type State = State;
    type Style = ();
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            map_size_start: 0.0,
            map_size_end: 10.0,
        }
    }

    fn style(&self) -> Self::Style {}

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { state, ui, id, .. } = args;

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
            // let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
            //     panic!(e);
            // });

            // match result {
            //     Response::Okay(file_path) => println!("File path = {:?}", file_path),
            //     Response::OkayMultiple(files) => println!("Files {:?}", files),
            //     Response::Cancel => println!("User canceled"),
            // }
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
            // let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
            //     panic!(e);
            // });

            // match result {
            //     Response::Okay(file_path) => println!("File path = {:?}", file_path),
            //     Response::OkayMultiple(files) => println!("Files {:?}", files),
            //     Response::Cancel => println!("User canceled"),
            // }
        }

        for _click in widget::Button::new()
            .parent(state.ids.canvas)
            .x_y(0.0, -250.0)
            .w_h(250.0, 50.0)
            .label("Start")
            .set(state.ids.btn_start, ui)
            {
                *self.screen = Screen::Game;
            }

        const PAD: conrod::Scalar = 20.0;
        let mut start: conrod::Scalar = 0.0;
        let mut end: conrod::Scalar = 0.0;
        for (edge, value) in widget::RangeSlider::new(state.map_size_start, state.map_size_end, 15.0, 100.0)
            .color(color::LIGHT_BLUE)
            .padded_w_of(state.ids.canvas, PAD)
            .h(30.0)
            .x_y(0.0, -200.0)
            // .mid_top_with_margin_on(state.ids.canvas, PAD)
            .set(state.ids.map_size, ui)
            {
                match edge {
                    widget::range_slider::Edge::Start => state.update(|state| {state.map_size_start = value}),
                    widget::range_slider::Edge::End => state.update(|state| {state.map_size_end = value}),
                }
            }
    }
}
