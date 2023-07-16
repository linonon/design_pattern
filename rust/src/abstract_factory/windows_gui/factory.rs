use super::super::gui::{GuiFactory, GuiFactoryDynamic};
use super::button::WindowsButton;
use super::checkbox::WindowsCheckbox;

pub struct WindowsGuiFactory;

impl GuiFactory for WindowsGuiFactory {
    type B = WindowsButton;
    type C = WindowsCheckbox;

    fn create_button(&self) -> Self::B {
        WindowsButton
    }

    fn create_checkbox(&self) -> Self::C {
        WindowsCheckbox
    }
}

impl GuiFactoryDynamic for WindowsGuiFactory {
    fn create_button(&self) -> Box<dyn crate::abstract_factory::gui::Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn crate::abstract_factory::gui::CheckBox> {
        Box::new(WindowsCheckbox)
    }
}
