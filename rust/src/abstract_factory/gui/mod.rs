/// 抽象工廠和抽象產品

pub trait Button {
    fn press(&self);
}

pub trait CheckBox {
    fn switch(&self);
}

/// 使用`泛型`定義`抽象工廠`
pub trait GuiFactory {
    type B: Button;
    type C: CheckBox;

    fn create_button(&self) -> Self::B;
    fn create_checkbox(&self) -> Self::C;
}

pub trait GuiFactoryDynamic {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn CheckBox>;
}
