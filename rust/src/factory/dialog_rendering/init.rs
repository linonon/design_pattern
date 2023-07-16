use crate::factory::dialog_rendering::gui::Dialog;
use crate::factory::dialog_rendering::html_gui::HtmlDialog;
use crate::factory::dialog_rendering::windows_gui::WindowsDialog;

pub fn initialize() -> &'static dyn Dialog {
    if cfg!(windows) {
        println!("-- Windows detected, creating Windows GUI --");
        &WindowsDialog
    } else {
        println!("-- No OS detected, creating HTML GUI --");
        &HtmlDialog
    }
}
