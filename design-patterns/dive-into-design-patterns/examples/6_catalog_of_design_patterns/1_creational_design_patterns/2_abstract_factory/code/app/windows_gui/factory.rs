use crate::gui::GuiFactory;
use crate::windows_gui::button::WindowsButton;
use crate::windows_gui::checkbox::WindowsCheckbox;

pub struct WindowsFactory;

impl GuiFactory for WindowsFactory {
    type B = WindowsButton;
    type C = WindowsCheckbox;

    fn create_button(&self) -> WindowsButton {
        WindowsButton
    }

    fn create_checkbox(&self) -> WindowsCheckbox {
        WindowsCheckbox
    }
}
