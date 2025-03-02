use bracket_lib::prelude::*;
use froguelike::{gamestate::State, map::{build_map, Map}, player::spawn_player};


embedded_resource!(DBYTE_FONT, "../resources/dinobyte_12x16.png");



fn main() -> BError
{
    link_resource!(DBYTE_FONT, "resources/dinobyte_12x16.png");

    //bracket_lib::terminal::EMBED.lock().get_resource(path)
    println!("Hello, world!");
    let mut term = BTermBuilder::new()
        .with_dimensions(80, 50)
        .with_title("FrogueLike")
        .with_tile_dimensions(12, 16)
        .with_fps_cap(60.)
        .with_font("dinobyte_12x16.png", 12, 16)
        .with_simple_console(80, 50, "dinobyte_12x16.png")
        .build()?;

   // term.set_active_font(0, true);

    let mut gs = State::new();
    build_map(&mut gs);
    spawn_player(&mut gs, (20,20).into());
    main_loop(term, gs)
}
