use std::cell::Cell;

use super::behaviors::MouseBehavior;
use crate::{
    prelude::*,
    shell::{Key, KeyEvent},
};

#[derive(Clone)]
enum TextBoxAction {
    Key(KeyEvent),
    Mouse(Point),
}

/// The `TextBoxState` handles the text processing of the `TextBox` widget.
#[derive(AsAny)]
pub struct TextBoxState {
    action: RefCell<Option<TextBoxAction>>,
    cursor_x: Cell<f64>,
    len: Cell<usize>,
}

impl Default for TextBoxState {
    fn default() -> Self {
        TextBoxState {
            action: RefCell::new(None),
            cursor_x: Cell::new(0.0),
            len: Cell::new(0),
        }
    }
}

impl TextBoxState {
    fn action(&self, action: TextBoxAction) {
        *self.action.borrow_mut() = Some(action);
    }

    fn handle_key_event(&self, key_event: KeyEvent, ctx: &mut Context<'_>) {
        if !ctx.widget().get::<bool>("focused") {
            return;
        }

        let text = ctx.widget().clone::<String16>("text");
        let mut current_selection = *ctx.child("cursor").get::<TextSelection>("text_selection");

        match key_event.key {
            Key::Left => {
                if let Some(selection) = ctx
                    .child("cursor")
                    .try_get_mut::<TextSelection>("text_selection")
                {
                    selection.start_index =
                        (current_selection.start_index as i32 - 1).max(0) as usize;
                }
            }
            Key::Right => {
                if let Some(selection) = ctx
                    .child("cursor")
                    .try_get_mut::<TextSelection>("text_selection")
                {
                    selection.start_index = (current_selection.start_index + 1).min(text.len());
                }
            }
            Key::Backspace => {
                if !text.is_empty() && current_selection.start_index > 0 {
                    for _ in 0..=current_selection.length {
                        ctx.widget()
                            .get_mut::<String16>("text")
                            .remove(current_selection.start_index - 1);
                        current_selection.start_index =
                            (current_selection.start_index as i32 - 1).max(0) as usize;
                    }

                    if let Some(selection) = ctx
                        .child("cursor")
                        .try_get_mut::<TextSelection>("text_selection")
                    {
                        selection.start_index = current_selection.start_index;
                    }

                    ctx.push_event(ChangedEvent(ctx.entity));
                }
            }
            Key::Delete => {
                if !text.is_empty() && text.len() > current_selection.start_index {
                    for _ in 0..=current_selection.length {
                        ctx.widget()
                            .get_mut::<String16>("text")
                            .remove(current_selection.start_index);
                    }

                    ctx.push_event(ChangedEvent(ctx.entity));
                }
            }
            Key::Enter => ctx.push_event(ActivateEvent(ctx.entity)),
            _ => {
                if key_event.text.is_empty() {
                    return;
                }
                ctx.widget()
                    .get_mut::<String16>("text")
                    .insert_str(current_selection.start_index, key_event.text.as_str());

                if let Some(selection) = ctx
                    .child("cursor")
                    .try_get_mut::<TextSelection>("text_selection")
                {
                    selection.start_index =
                        current_selection.start_index + key_event.text.encode_utf16().count();
                }

                ctx.push_event(ChangedEvent(ctx.entity));
            }
        }
    }

    fn request_focus(&self, ctx: &mut Context<'_>) {
        let focused_widget = ctx.window().get::<Global>("global").focused_widget;

        if (focused_widget.is_some() && focused_widget.unwrap() == ctx.entity)
            || !ctx.widget().get::<bool>("enabled")
        {
            return;
        }

        if let Some(old_focused_element) = ctx.window().get::<Global>("global").focused_widget {
            let mut old_focused_element = ctx.get_widget(old_focused_element);
            old_focused_element.set("focused", false);
            old_focused_element.update_theme_by_state(false);
        }

        ctx.window().get_mut::<Global>("global").focused_widget = Some(ctx.entity);

        ctx.widget().set("focused", true);
        ctx.widget().update_theme_by_state(false);
        ctx.child("cursor").update_theme_by_state(false);
    }

    // Reset selection and offset if text is changed from outside
    fn reset(&self, ctx: &mut Context<'_>) {
        ctx.widget().set("text_selection", TextSelection::default());
        ctx.widget().set("scroll_offset", Point::default());
        ctx.push_event(ChangedEvent(ctx.entity));
    }
}

impl State for TextBoxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.len.set(ctx.widget().get::<String16>("text").len());
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        // check if text len is changed from outside
        if self.len.get() != ctx.widget().get::<String16>("text").len() {
            self.reset(ctx);
        }

        if let Some(action) = self.action.borrow().clone() {
            match action {
                TextBoxAction::Key(event) => {
                    self.handle_key_event(event, ctx);
                }
                TextBoxAction::Mouse(_p) => {
                    self.request_focus(ctx);
                }
            }
        }

        *self.action.borrow_mut() = None;
        ctx.widget().update_theme_by_state(false);
        self.len.set(ctx.widget().get::<String16>("text").len());
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let mut cursor_x_delta = 0.0;
        let mut scroll_viewer_width = 0.0;

        {
            if let Some(bounds) = ctx
                .child("scroll_viewer")
                .try_get_mut::<Rectangle>("bounds")
            {
                scroll_viewer_width = bounds.width();
            }
        }

        // maybe not use scroll viewer here

        // Adjust offset of text and cursor if cursor position is out of bounds

        {
            let mut cursor = ctx.child("cursor");

            if let Some(margin) = cursor.try_get_mut::<Thickness>("margin") {
                if margin.left() < 0.0 || margin.left() > scroll_viewer_width {
                    cursor_x_delta = self.cursor_x.get() - margin.left();
                    margin.set_left(self.cursor_x.get());
                }
                self.cursor_x.set(margin.left());
            }

            if let Some(bounds) = cursor.try_get_mut::<Rectangle>("bounds") {
                bounds.set_x(self.cursor_x.get());
            }
        }

        if cursor_x_delta != 0.0 {
            {
                if let Some(bounds) = ctx.child("text_block").try_get_mut::<Rectangle>("bounds") {
                    bounds.set_x(bounds.x() + cursor_x_delta);
                }
            }

            if let Some(scroll_offset) = ctx.widget().try_get_mut::<Point>("scroll_offset") {
                scroll_offset.x += cursor_x_delta;
            }
        }
    }
}

widget!(
    /// The `TextBox` widget represents a single line text input widget.
    ///
    /// * CSS element: `text-box`
    TextBox<TextBoxState>: ActivateHandler, ChangedHandler, KeyDownHandler {
        /// Sets or shares the text property.
        text: String16,

        /// Sets or shares the water_mark text property.
        water_mark: String16,

        /// Sets or shares the text selection property.
        text_selection: TextSelection,

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or share the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the text offset property.
        scroll_offset: Point,

        /// Sets or shares the (wheel, scroll) delta property.
        delta: Point,

         /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for TextBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TextBox")
            .selector("text-box")
            .text("")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto Regular")
            .text_selection(TextSelection::default())
            .scroll_offset(0.0)
            .padding(4.0)
            .background(colors::LYNCH_COLOR)
            .border_brush("transparent")
            .border_width(0.0)
            .border_radius(2.0)
            .min_size(128.0, 32.0)
            .focused(false)
            .delta(0.0)
            .child(
                MouseBehavior::create()
                    .on_mouse_down(move |states, p| {
                        states
                            .get::<TextBoxState>(id)
                            .action(TextBoxAction::Mouse(p));
                        true
                    })
                    .child(
                        Container::create()
                            .background(id)
                            .border_radius(id)
                            .border_width(id)
                            .border_brush(id)
                            .padding(id)
                            .child(
                                Grid::create()
                                    .child(
                                        ScrollViewer::create()
                                            .selector(Selector::default().id("scroll_viewer"))
                                            .scroll_offset(id)
                                            .scroll_viewer_mode(("custom", "disabled"))
                                            .delta(id)
                                            .child(
                                                TextBlock::create()
                                                    .selector(Selector::default().id("text_block"))
                                                    .vertical_alignment("center")
                                                    .foreground(id)
                                                    .text(id)
                                                    .water_mark(id)
                                                    .font(id)
                                                    .font_size(id)
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .child(
                                        Cursor::create()
                                            .selector(Selector::from("cursor").id("cursor"))
                                            .margin(0.0)
                                            .horizontal_alignment("start")
                                            .text(id)
                                            .font(id)
                                            .font_size(id)
                                            .scroll_offset(id)
                                            .focused(id)
                                            .text_selection(id)
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_key_down(move |states, event| -> bool {
                states
                    .get::<TextBoxState>(id)
                    .action(TextBoxAction::Key(event));
                false
            })
    }
}
