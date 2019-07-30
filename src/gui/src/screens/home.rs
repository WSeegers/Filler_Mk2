use conrod::{widget, Widget, Labelable, Positionable, Sizeable, color, Colorable, Borderable};

widget_ids!(struct Ids {
    canvas,
    title,
    btn_single,
    btn_tournament,
    btn_exit,
});

pub struct State {
    ids: Ids,
    clicks: usize,
}

#[derive(WidgetCommon)]
pub struct Home {
    #[conrod(common_builder)] common: widget::CommonBuilder,
}

impl Home {
    pub fn new() -> Self {
        Self {
            common: widget::CommonBuilder::default(),
        }
    }
}

impl Widget for Home {
    type State = State;
    type Style = ();
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            clicks: 0,
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
            state.update(|state| state.clicks += 1);
        }

        for _click in widget::Button::new()
            .parent(state.ids.canvas)
            .w_h(200.0, 50.0)
            .label("Tournament")
            .set(state.ids.btn_tournament, ui)
        {
            state.update(|state| state.clicks += 1);
        }

        for _click in widget::Button::new()
            .parent(state.ids.canvas)
            .w_h(200.0, 50.0)
            .label("Exit")
            .set(state.ids.btn_exit, ui)
        {
            state.update(|state| state.clicks += 1);
        }
    }
}
