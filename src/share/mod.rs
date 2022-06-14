mod constants;
mod load_state;
pub use constants::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState{
    PreLoad,
    Menu,
    SoulChoice,
    Load,
    SoulView,
    OverWorld,
    Multyvers
}