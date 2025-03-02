use crate::{components::{BlocksTile, Position}, gamestate::State};





pub fn map_indexing_system(state : &mut State)
{
    state.map.populate_blocked();

    for (_id, (_blocks, pos)) in state.world.query_mut::<(&BlocksTile, &Position)>()
    {
        let idx = state.map.pos_to_idx(*pos);

        state.map.blocked[idx] = true;
    }
}