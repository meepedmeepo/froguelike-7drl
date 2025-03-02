use bracket_lib::{prelude::{Point, Tile}, random::RandomNumberGenerator};

use crate::gamestate::State;

use super::{TileType, MAPHEIGHT, MAPWIDTH};



pub fn cellular_automata(state : &mut State)
{
    let mut rng = RandomNumberGenerator::new();

    for y in 1..MAPHEIGHT -1
    {
        for x in 1..MAPWIDTH -1
        {
            let roll = rng.roll_dice(1, 100);
            let idx = state.map.xy_idx(x, y);

            if roll > 55
            {
                state.map.tiles[idx] = TileType::Floor;
            } else {
                state.map.tiles[idx] = TileType::Wall;
            }
        }
    }


    for _i in 0..15
    {
        apply_iteration(state);
    }
}

fn apply_iteration(state : &mut State)
{
    let mut newtiles = state.map.tiles.clone();
    for y in 1..MAPHEIGHT-1
    {
        for x in 1..MAPWIDTH-1 
        {
    
            let idx = state.map.xy_idx(x, y);
            let mut neighbors = 0;

            if state.map.tiles[idx - 1] == TileType::Wall { neighbors += 1; }
            if state.map.tiles[idx + 1] == TileType::Wall { neighbors += 1; }
            if state.map.tiles[idx - MAPWIDTH as usize] == TileType::Wall { neighbors += 1; }
            if state.map.tiles[idx + MAPWIDTH as usize] == TileType::Wall { neighbors += 1; }
            if state.map.tiles[idx - (MAPWIDTH as usize - 1)] == TileType::Wall { neighbors += 1; }
            if state.map.tiles[idx - (MAPWIDTH as usize + 1)] == TileType::Wall { neighbors += 1; }
            if state.map.tiles[idx + (MAPWIDTH as usize - 1)] == TileType::Wall { neighbors += 1; }
            if state.map.tiles[idx + (MAPWIDTH as usize + 1)] == TileType::Wall { neighbors += 1; }

            if neighbors > 4 || neighbors == 0
            {
                newtiles[idx] = TileType::Wall;
            } else 
            {
                newtiles[idx] = TileType::Floor;    
            }
        }
    }

    state.map.tiles = newtiles;
}


fn cull_unreachable(state : &mut State, start_pos : Point)
{
    let idx = state.map.xy_idx(start_pos.x, start_pos.y);

    state.map.populate_blocked();



}