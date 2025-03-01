use bracket_lib::prelude::GameState;
use hecs::Entity;

use crate::{map::Map, systems::render::render_system};


pub struct State
{
    pub world : hecs::World,
    pub map : Map,
    player_ent : Option<Entity>,
}

impl State
{   
    ///* MAKE SURE TO ADD PLAYER_ENT WHEN FUCKING SPAWNED THEM YOU FUCKING NERD WANKER
    pub fn new() -> State
    {
        State {world : hecs::World::new(), map : Map::new(), player_ent : None,}
    }
}

impl GameState for State
{
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) 
    {
        ctx.cls();
        render_system(self, ctx);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramState
{
    Ticking,
    AwaitingInput
}