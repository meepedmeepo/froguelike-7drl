use bracket_lib::prelude::field_of_view;

use crate::{components::{FoV, Position}, gamestate::State, map::{MAPHEIGHT, MAPWIDTH}};



pub fn visibility_system(state : &mut State)
{
    let p = state.get_player();

    for (id, (fov, pos)) in state.world.query_mut::<(&mut FoV, &Position)>()
    {
        if fov.dirty
        {
            fov.dirty = false;
            fov.visible_tiles.clear();
            fov.visible_tiles = field_of_view(pos.pos, fov.range, &state.map);
            fov.visible_tiles.retain(|p| p.x >= 0 && p.x < MAPWIDTH && p.y >= 0 && p.y < MAPHEIGHT);

            if id == p
            {
                for rev in state.map.visible_tiles.iter_mut(){*rev = false;}

                for vis in fov.visible_tiles.iter()
                {
                    let idx = state.map.point_to_idx(*vis);

                    state.map.visible_tiles[idx] = true;
                    state.map.known_tiles[idx] = true;
                }
            }
        }
    }
}