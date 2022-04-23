// use bevy::prelude::*;
pub mod game;
pub mod home;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    //states' enum
    Home,
    CountDown,
    Free,
    Ranking,
    // Paused,
}

pub struct DiskNumber(pub i32);

// pub fn push_to_state_stack(mut app_state: ResMut<State<AppState>>) {
//     app_state.push(AppState::Paused).unwrap();
// }

// pub fn pop_from_state_stack(mut app_state: ResMut<State<AppState>>) {
//     app_state.pop().unwrap();
// }
