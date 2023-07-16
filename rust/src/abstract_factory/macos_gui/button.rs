use super::super::gui::Button;

pub struct MacButton;

impl Button for MacButton {
    fn press(&self) {
        println!("macOS button has pressed");
    }
}
