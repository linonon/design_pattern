use super::game::{MazeGame, Room};

#[derive(Clone)]
pub struct MagicRoom {
    title: String,
}

impl MagicRoom {
    fn new(title: String) -> Self {
        Self { title }
    }
}

impl Room for MagicRoom {
    fn render(&self) {
        println!("Magic Room: #{}", self.title);
    }
}

pub struct MagicMaze {
    rooms: Vec<MagicRoom>,
}

impl MagicMaze {
    pub fn new() -> Self {
        Self {
            rooms: vec![
                MagicRoom::new("Inifinite Room".into()), // 這裡使用 .into 自動將 &str 轉成 String
                MagicRoom::new("Red Room".into()),
            ],
        }
    }
}

impl MazeGame for MagicMaze {
    type RoomImpl = MagicRoom;

    fn rooms(&self) -> Vec<Self::RoomImpl> {
        self.rooms.clone()
    }
}
