pub trait Button {
    fn render(&self);
    fn on_click(&self);
}

/// Dialo 有一個工廠方法 `create_button`
///
/// 它會根據工廠的具體 implement 創建不同的 button
pub trait Dialog {
    /// 工廠方法. 必須被具體的實例重載
    fn create_button(&self) -> Box<dyn Button>;

    fn render(&self) {
        let button = self.create_button();
        button.render();
    }

    fn refresh(&self) {
        println!("Dialog - Refresh");
    }
}
