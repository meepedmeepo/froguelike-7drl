

use bracket_lib::{color::{BLACK, GREEN, RGB}, prelude::{to_cp437, Point}};

use crate::{components::{Description, Name, Position, Renderable}, gamestate::State};



pub struct Player;
pub struct Score{pub value : i32}

pub fn spawn_player(state : &mut State, pos : Point)
{
    state.world.spawn((
        Player, Name::new("Jelly"),
        Description::new("This is you! Stop looking at yourself and go eat bugs!"),
        Score{value : 0},
        Renderable::new(to_cp437('f'), GREEN.into(), BLACK.into()),
        Position::from_point(pos)
        ));
}