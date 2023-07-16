use super::super::gui::CheckBox;

pub struct WindowsCheckbox;

impl CheckBox for WindowsCheckbox {
    fn switch(&self) {
        println!("windows checkbox has switched");
    }
}
