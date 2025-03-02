

use bracket_lib::{color::{BLACK, GREEN, RGB}, prelude::{to_cp437, BTerm, BaseMap, Point, VirtualKeyCode}};

use crate::{components::{Description, Enemy, FoV, Name, Position, Renderable}, gamestate::{ProgramState, State}};



pub struct Player;
pub struct Score{pub value : i32}

pub fn spawn_player(state : &mut State, pos : Point)
{
    state.player_pos = pos;

    let p_ent = state.world.spawn((
        Player, Name::new("Jelly"),
        Description::new("This is you! Stop looking at yourself and go eat bugs!"),
        Score{value : 0},
        Renderable::new(to_cp437('f'), GREEN.into(), BLACK.into(), 3),
        Position::from_point(pos),
        FoV{dirty : true, visible_tiles :  Vec::new(), range : 6,}
        ));

    state.set_player(p_ent);
}


pub fn player_input(state : &mut State, ctx : &mut BTerm) -> ProgramState
{
    match ctx.key
    {
        None => ProgramState::AwaitingInput,
        
        Some(key) =>
        {
            match key
            {
                VirtualKeyCode::Numpad4 =>return attempt_move(state, (-1, 0)),
                VirtualKeyCode::Numpad6 => return attempt_move(state,(1,0)),
                VirtualKeyCode::Numpad8 => return attempt_move(state,(0,-1)),
                VirtualKeyCode::Numpad2 => return attempt_move(state,(0,1)),

                // Diagonals
                VirtualKeyCode::Numpad9 => return attempt_move(state, (1, -1)),
                VirtualKeyCode::Numpad7 => return attempt_move(state, (-1, -1)),
                VirtualKeyCode::Numpad3 => return attempt_move(state, (1, 1)),
                VirtualKeyCode::Numpad1 => return attempt_move(state, (-1, 1)),

                VirtualKeyCode::Numpad5 | VirtualKeyCode::Space => ProgramState::Ticking,

                _ => ProgramState::AwaitingInput
            }
        }
    }
}


fn attempt_move(state : &mut State, delta : (i32,i32)) -> ProgramState
{
    
    let mut pos = state.player_pos;
    let start_idx = state.map.xy_idx(pos.x, pos.y);

    pos += delta.into();

    let idx = state.map.xy_idx(pos.x, pos.y);

    if !state.map.blocked[idx]
    {
        state.map.blocked[start_idx] = false;
        state.player_pos = pos;
        state.map.blocked[idx] = true;

        state.world.get::<&mut FoV>(state.get_player()).unwrap().dirty = true;

        let _ = state.world.insert_one(state.get_player(), Position::from_point(pos));

        return ProgramState::Ticking

    }else 
    {
        if state.map.is_opaque(idx) {return ProgramState::AwaitingInput;}
        
        let creatures = state.get_creatures_at_tile(idx);
        
        for c in creatures
        {
            if state.world.get::<&Enemy>(c).is_ok()
            {
                todo!("SHOULD MARK ENEMY FOR ATTACK HEHEHHEHEHEH");
                return ProgramState::Ticking;
            }
        }
    }

    ProgramState::AwaitingInput
}