use crate::factory::dialog_rendering::gui::{Button, Dialog};

/// 這個寫法類似於 go 中的 type HtmlButton struct{},
/// 然後可以定義 func (h *Html) render() {} 等方法.
///
/// 但是還是有不一樣的點, 因為 rust 中, 這個 HtmlButton 是不可以被實例化的.
pub struct HtmlButton;

impl Button for HtmlButton {
    fn render(&self) {
        println!("<button>Test Button</button>");
        self.on_click();
    }

    fn on_click(&self) {
        println!("Click! Button says - 'Hello World'");
    }
}

pub struct HtmlDialog;

impl Dialog for HtmlDialog {
    // 創建一個 HTML 的 button
    fn create_button(&self) -> Box<dyn Button> {
        // 使用 Box, 因為 Button 是一個 trait 所以會是 dyn 的.
        Box::new(HtmlButton)
    }
}
