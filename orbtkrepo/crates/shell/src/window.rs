//! This module contains traits to inject custom logic into the window shell.

use crate::{event::*, utils::Point};

/// The window adapter is used to work with the window shell.ButtonState
///
/// It handles updates from the shell and provides method to update and render
/// its content.
pub trait WindowAdapter {
    // /// Renders the content.
    // fn render(&mut self, _canvas: &mut Canvas) {}

    // /// Updates the content.
    // fn update(&mut self) {}

    /// Is called after the window is resized.
    fn resize(&mut self, _width: f64, _height: f64) {}

    /// Is called after the mouse was moved.
    fn mouse(&mut self, _x: f64, _y: f64) {}

    /// Is called after the state of a mouse button is changed.
    fn mouse_event(&mut self, _event: MouseEvent) {}

    /// Is called if mouse wheel or trackpad detect scroll event.
    fn scroll(&mut self, _delta_x: f64, _delta_y: f64) {}

    /// Is called after the state of a keyboard key is changed.
    fn key_event(&mut self, _event: KeyEvent) {}

    /// Is called after the quite event of the window is called.
    fn quite_event(&mut self) {}

    /// Gets the current mouse position.
    fn mouse_position(&self) -> Point;
}

/// Used to define an additional updater for the window shell.
pub trait Updater {
    /// Called to update the content.
    fn update(&mut self);
}
