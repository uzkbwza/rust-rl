pub mod monster;
pub mod dummy;


#[derive(Debug, Copy, Clone)]
pub enum AiType {
    Monster,
    _Dummy,
    _Friendly,
}