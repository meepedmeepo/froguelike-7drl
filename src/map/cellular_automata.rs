use bracket_lib::{prelude::Tile, random::RandomNumberGenerator};

use crate::gamestate::State;

use super::{TileType, MAPHEIGHT, MAPWIDTH};



pub fn cellular_automata(state : &mut State)
{
    let mut rng = RandomNumberGenerator::new();

    for y in 1..MAPHEIGHT
    {
        for x in 1..MAPWIDTH
        {
            let roll = rng.roll_dice(1, 100);
            let idx = state.map.xy_idx(x, y);
            
            if roll > 55
            {
                state.map.tiles[idx] = TileType::Ground;
            } else {
                state.map.tiles[idx] = TileType::Wall;
            }
        }
    }

}