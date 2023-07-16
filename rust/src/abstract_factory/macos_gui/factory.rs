use super::super::gui::*;
use super::button::MacButton;
use super::checkbox::MacCheckBox;

pub struct MacFactory;

impl GuiFactory for MacFactory {
    type B = MacButton;
    type C = MacCheckBox;

    fn create_button(&self) -> Self::B {
        MacButton
    }

    fn create_checkbox(&self) -> Self::C {
        MacCheckBox
    }
}

impl GuiFactoryDynamic for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(MacCheckBox)
    }
}
