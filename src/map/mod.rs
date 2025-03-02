mod cellular_automata;


use bracket_lib::{color::{BLACK, GREEN, WHITE}, prelude::{to_cp437, BaseMap, DistanceAlg, DrawBatch, Point, SmallVec}};
use hecs::Entity;

use crate::{components::Renderable, gamestate::State};


pub const MAPWIDTH : i32 = 65;
pub const MAPHEIGHT : i32 = 30;
pub const MAPSIZE : usize = MAPWIDTH as usize * MAPHEIGHT as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType
{
    Wall, Floor
}

impl TileType 
{
    pub fn as_renderable(&self) -> Renderable
    {
        match self
        {
            TileType::Wall => Renderable::new(to_cp437('#'), WHITE.into(), BLACK.into(), 1),
            TileType::Floor => Renderable::new(to_cp437('.'),  GREEN.into(), BLACK.into(),1)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map
{
    pub tiles : Vec<TileType>,
    pub blocked : Vec<bool>,
    pub tile_contents : Vec<Vec<Entity>>,
    pub visible_tiles : Vec<bool>,
    pub known_tiles : Vec<bool>,

}

impl Map
{
    pub fn new() -> Map
    {
        Map
        {
            tiles : vec![TileType::Wall; MAPSIZE],
            blocked : vec![false; MAPSIZE],
            tile_contents : vec![Vec::new(); MAPSIZE],
            visible_tiles : vec![true; MAPSIZE],
            known_tiles : vec![false; MAPSIZE],
        }
    }

    pub fn xy_idx(&self, x : i32, y : i32) -> usize
    {
        (y as usize * MAPWIDTH as usize) + x as usize
    }

    pub fn pos_from_idx(&self, idx : usize) -> Point
    {
        Point::new(idx as i32 % MAPWIDTH , idx as i32/ MAPWIDTH)
    }

    pub fn is_exit_valid(&self, x : i32, y : i32) -> bool
    {
        if x < 1 || x > MAPWIDTH-1 || y < 1 || y > MAPHEIGHT-1 { return false; }

        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    pub fn get_tile_glyphs(&self) -> Vec<Renderable>
    {
        self.tiles.iter().map(|tile| {
            tile.as_renderable()
        }).collect()
    }

    pub fn is_visible(&self, pos : Point) -> bool
    {
        let idx = self.xy_idx(pos.x, pos.y);

        self.visible_tiles[idx]
    }

    pub fn populate_blocked(&mut self)
    {
        for (i, tile) in self.tiles.iter().enumerate()
        {
            self.blocked[i] = self.is_opaque(i);
        } 
    }

    pub fn get_entites_at_tile(&self, idx : usize) -> Vec<Entity>
    {
        self.tile_contents[idx].clone()
    }
}

impl BaseMap for Map
{
    fn get_available_exits(&self, idx:usize) -> SmallVec<[(usize, f32); 10]> 
    {
        let mut exits = SmallVec::new();
        let x = idx as i32 % MAPWIDTH;
        let y = idx as i32 / MAPWIDTH;
        let w = MAPWIDTH as usize;
        //let tt = self.tiles[idx];
    
        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.)) };

            // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w)-1, 1.45)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w)+1, 1.45)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w)-1, 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w)+1, 1.45)); }
    
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 
    {
        let w = MAPWIDTH as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)    
    }

    fn is_opaque(&self, idx: usize) -> bool 
    {
        if idx > 0 && idx < self.tiles.len()
        {
            if self.tiles[idx] == TileType::Wall {true}
            else 
            {false}
        }
        else 
        {
            true
        }    
    }

}

pub fn build_map(state : &mut State)
{
    cellular_automata::cellular_automata(state);
}