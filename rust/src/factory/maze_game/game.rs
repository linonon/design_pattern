// Maze room 是需要被實例化的工廠方法
pub trait Room {
    fn render(&self);
}

// Maze game 有一個工廠方法用來生產不同的 rooms
pub trait MazeGame {
    type RoomImpl: Room;

    fn rooms(&self) -> Vec<Self::RoomImpl>;

    fn play(&self) {
        for room in self.rooms() {
            room.render();
        }
    }
}

/// The client code initializes resources and does other preparations
/// then it uses a factory to construct and run the game.
///
/// 這裡的 impl MazeGame 表示輸入的參數需要是實現了 MazeGame trait 的數據結構
pub fn run(maze_game: impl MazeGame) {
    println!("Loading resources...");
    println!("Starting the game...");

    maze_game.play();
}
