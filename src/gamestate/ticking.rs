use bracket_lib::prelude::GameState;

use crate::{map, player, systems::{render::render_system, run_systems}};

use super::{ProgramState, State};



impl GameState for State
{
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) 
    {
        match self.current_state
        {
            ProgramState::Ticking =>
            {
                run_systems(self, ctx);

                self.current_state = ProgramState::AwaitingInput;
            }

            ProgramState::AwaitingInput =>
            {
                ctx.cls();
                self.current_state = player::player_input(self, ctx);
                render_system(self, ctx);
            }

            _ => {}
        }
    
    }
}