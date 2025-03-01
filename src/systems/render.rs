use bracket_lib::prelude::{to_char, BTerm};

use crate::{components::{Position, Renderable}, gamestate::State};




pub fn render_system(state : &mut State, ctx : &mut BTerm)
{
    let m = state.map.clone();
    draw_map(state, ctx);

    let mut ents = state.world.query::<(&Renderable, &Position)>()
        .iter().filter_map(|(_ent,(rend, pos))|
        {
            if m.is_visible(pos.pos)
            {
                Some((*rend, *pos))
            }
            else {
                None
            }
        }).collect::<Vec<_>>();

    ents.sort_by(|a,b|
        {
            b.0.order.cmp(&a.0.order)
        });

    for (rend, pos) in ents.iter()
    {
        ctx.print_color(pos.pos.x, pos.pos.y, rend.fg, rend.bg, bracket_lib::terminal::to_char(rend.glyph as u8));
    }
}


pub fn draw_map(state : &mut State, ctx : &mut BTerm)
{
    state.map.tiles.iter()
        .enumerate()
        .filter_map(|(idx,tile)|
        {
            let pos = state.map.pos_from_idx(idx);
            if state.map.is_visible(pos) || state.map.known_tiles[idx]
            {
                Some((*tile, pos))
            }
            else {
                None
            }
        })
        .for_each(|(tile, pos)|
        {
            let mut glyph = tile.as_renderable();

            if !state.map.is_visible(pos)
            {
                glyph.fg = glyph.fg.to_greyscale();
                glyph.bg = glyph.bg.to_greyscale();
            }

            ctx.print_color(pos.x, pos.y, glyph.fg, glyph.bg, to_char(glyph.glyph as u8));
        });
}