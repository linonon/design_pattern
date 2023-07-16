use super::super::gui::Button;

pub struct WindowsButton;

impl Button for WindowsButton {
    fn press(&self) {
        println!("windows button has pressed");
    }
}
