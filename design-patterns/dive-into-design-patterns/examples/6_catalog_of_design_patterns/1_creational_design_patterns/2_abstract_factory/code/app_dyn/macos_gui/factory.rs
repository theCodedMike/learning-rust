use crate::gui::{Button, Checkbox, GuiFactoryDynamic};
use crate::macos_gui::button::MacButton;
use crate::macos_gui::checkbox::MacCheckbox;

pub struct MacFactory;

impl GuiFactoryDynamic for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}
