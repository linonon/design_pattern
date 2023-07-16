use mylib::abstract_factory::{
    macos_gui::factory::MacFactory, render, windows_gui::factory::WindowsGuiFactory,
};

fn main() {
    let windows = true;

    if windows {
        render::render(WindowsGuiFactory)
    } else {
        render::render(MacFactory)
    }
}
