use conrod::{color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

use crate::core::Screen;

widget_ids!(struct Ids {
    canvas,
    title,
    btn_single,
    btn_tournament,
    btn_exit,
});

pub struct State {
    ids: Ids,
}

#[derive(WidgetCommon)]
pub struct Home<'a> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    screen: &'a mut Screen,
}

impl<'a> Home<'a> {
    pub fn new(screen: &'a mut Screen) -> Self {
        Self {
            common: widget::CommonBuilder::default(),
            screen,
        }
    }
}

impl<'a> Widget for Home<'a> {
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
        let widget::UpdateArgs { state, ui, id, .. } = args;

        // Background
        widget::Canvas::new()
            .parent(id)
            .color(color::DARK_CHARCOAL)
            .border(0.0)
            .wh_of(id)
            .set(state.ids.canvas, ui);

        // Title
        widget::Text::new("Filler")
            .parent(state.ids.canvas)
            .x_y(0.0, 200.0)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(state.ids.title, ui);

        // Buttons
        for _click in widget::Button::new()
            .parent(state.ids.canvas)
            .middle_of(state.ids.canvas)
            .w_h(200.0, 50.0)
            .label("Single Game")
            .set(state.ids.btn_single, ui)
        {
            *self.screen = Screen::Game;
        }

        for _click in widget::Button::new()
            .parent(state.ids.canvas)
            .w_h(200.0, 50.0)
            .label("Tournament")
            .set(state.ids.btn_tournament, ui)
        {
            *self.screen = Screen::Tournament;
        }

        for _click in widget::Button::new()
            .parent(state.ids.canvas)
            .w_h(200.0, 50.0)
            .label("Exit")
            .set(state.ids.btn_exit, ui)
        {
            *self.screen = Screen::Exit;
        }
    }
}
