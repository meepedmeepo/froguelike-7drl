use bracket_lib::prelude::GameState;


pub struct State
{
    pub world : hecs::World
}

impl GameState for State
{
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) 
    {
        ctx.cls();
        ctx.print(20, 10, "@");
    }
}