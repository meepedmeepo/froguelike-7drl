

use bracket_lib::{color::{BLACK, GREEN, RGB}, prelude::{to_cp437, BTerm, BaseMap, Point}};

use crate::{components::{Description, Enemy, Name, Position, Renderable}, gamestate::{ProgramState, State}};



pub struct Player;
pub struct Score{pub value : i32}

pub fn spawn_player(state : &mut State, pos : Point)
{
    state.player_pos = pos;

    state.world.spawn((
        Player, Name::new("Jelly"),
        Description::new("This is you! Stop looking at yourself and go eat bugs!"),
        Score{value : 0},
        Renderable::new(to_cp437('f'), GREEN.into(), BLACK.into(), 3),
        Position::from_point(pos),
        ));
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

                _ => ProgramState::AwaitingInput
            }
        }
    }
}


fn attempt_move(state : &mut State, delta : (i32,i32)) -> ProgramState
{
    let mut pos = state.player_pos;

    pos += delta.into();

    let idx = state.map.xy_idx(pos.x, pos.y);

    if !state.map.blocked[idx]
    {
        state.player_pos = pos;
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

                return ProgramState::Ticking;
            }
        }
    }

    ProgramState::AwaitingInput
}