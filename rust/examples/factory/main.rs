use mylib::factory::{dialog_rendering::init::initialize, maze_game::game};

fn main() {
    // dialog rendering
    let dialog = initialize();
    dialog.render();
    dialog.refresh();

    // maze game
    use mylib::factory::maze_game::magic_maze::MagicMaze;
    use mylib::factory::maze_game::ordinary_maze::OrdinaryMaze;

    let magic_maze = MagicMaze::new();
    let ordianry_maze = OrdinaryMaze::new();

    game::run(magic_maze);
    game::run(ordianry_maze);
}
