pub mod render;
pub mod visibility;

use bracket_lib::prelude::BTerm;
use render::render_system;
use visibility::*;

use crate::{gamestate::{ProgramState, State}, map};




pub fn run_systems(state : &mut State, ctx : &mut BTerm)
{
    map::indexing::map_indexing_system(state);
    if state.current_state == ProgramState::Ticking
    {
        visibility_system(state);
    }
    

    render_system(state, ctx);
}