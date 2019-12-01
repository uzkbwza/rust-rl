pub mod dummy;
pub mod monster;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum AiType {
    Monster,
    _Dummy,
    _Friendly,
}
