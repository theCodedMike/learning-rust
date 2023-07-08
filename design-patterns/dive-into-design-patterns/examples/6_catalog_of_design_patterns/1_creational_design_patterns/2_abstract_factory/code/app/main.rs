use crate::macos_gui::factory::MacFactory;
use crate::windows_gui::factory::WindowsFactory;

mod gui;
mod macos_gui;
mod render;
mod windows_gui;

/// cargo r --example abstract-factory-app
fn main() {
    let windows = true;

    if windows {
        render::render(WindowsFactory);
    } else {
        render::render(MacFactory);
    }
}
