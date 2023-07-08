use crate::gui::GuiFactory;
use crate::macos_gui::button::MacButton;
use crate::macos_gui::checkbox::MacCheckbox;

pub struct MacFactory;

impl GuiFactory for MacFactory {
    type B = MacButton;
    type C = MacCheckbox;

    fn create_button(&self) -> MacButton {
        MacButton
    }

    fn create_checkbox(&self) -> MacCheckbox {
        MacCheckbox
    }
}
