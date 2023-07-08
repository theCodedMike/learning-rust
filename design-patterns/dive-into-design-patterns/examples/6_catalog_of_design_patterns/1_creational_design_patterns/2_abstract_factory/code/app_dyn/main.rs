use crate::gui::GuiFactoryDynamic;
use crate::macos_gui::factory::MacFactory;
use crate::windows_gui::factory::WindowsFactory;

mod gui;
mod macos_gui;
mod render;
mod windows_gui;

/// cargo r --example abstract-factory-app-dyn
fn main() {
    let windows = false;

    // Allocate a factory object in runtime depending on unpredictable input.
    let factory: &dyn GuiFactoryDynamic = if windows {
        &WindowsFactory
    } else {
        &MacFactory
    };

    // Factory invocation can be inlined right here.
    let button = factory.create_button();
    button.press();

    // Factory object can be passed to a function as a parameter.
    render::render(factory);
}
