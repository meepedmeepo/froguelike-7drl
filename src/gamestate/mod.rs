mod ticking;


pub use ticking::*;
use bracket_lib::prelude::{GameState, Point};
use hecs::Entity;

use crate::{components::Creature, map::Map, systems::render::render_system};


pub struct State
{
    pub world : hecs::World,
    pub map : Map,
    player_ent : Option<Entity>,
    pub player_pos : Point,
    pub current_state : ProgramState
}

impl State
{   
    ///* MAKE SURE TO ADD PLAYER_ENT WHEN FUCKING SPAWNED THEM YOU FUCKING NERD WANKER
    pub fn new() -> State
    {
        State 
        {
            world : hecs::World::new(), map : Map::new(), player_ent : None,
            player_pos : Point::zero(), current_state : ProgramState::Ticking
        }
    }

    pub fn get_player(&self) -> Entity
    {
        self.player_ent.unwrap()
    }

    pub fn set_player(&mut self, player : Entity)
    {
        self.player_ent = Some(player);
    }

    pub fn get_creatures_at_tile(&self, idx : usize) -> Vec<Entity>
    {
        let ents = self.map.get_entites_at_tile(idx);

        ents.iter().filter_map(|e|
        {
            if self.world.get::<&Creature>(*e).is_ok()
            {
                Some(*e)
            }
            else {
                None
            }
        }).collect()
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramState
{
    Ticking,
    AwaitingInput,
    Targetting,
}