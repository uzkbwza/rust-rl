pub struct GameState {
    pub end: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum AiType {
    _Dummy,
    _Friendly,
    _Monster,
}
