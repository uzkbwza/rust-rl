pub mod monster;
pub mod dummy;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum AiType {
    Monster,
    _Dummy,
    _Friendly,
}