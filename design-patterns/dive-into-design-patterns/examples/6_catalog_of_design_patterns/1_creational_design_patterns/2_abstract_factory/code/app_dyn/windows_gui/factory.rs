use crate::gui::{Button, Checkbox, GuiFactoryDynamic};
use crate::windows_gui::button::WindowsButton;
use crate::windows_gui::checkbox::WindowsCheckbox;

pub struct WindowsFactory;

impl GuiFactoryDynamic for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}
