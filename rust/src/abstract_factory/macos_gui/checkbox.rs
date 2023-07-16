use super::super::gui::CheckBox;

pub struct MacCheckBox;

impl CheckBox for MacCheckBox {
    fn switch(&self) {
        println!("macOS checkbox has switched");
    }
}
