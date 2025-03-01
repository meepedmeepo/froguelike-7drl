use bracket_lib::{color::{ColorPair, RGB}, prelude::{FontCharType, Point}};




pub struct Health
{
    pub current : i32,
    pub max : i32,
}

pub struct Renderable
{
    pub glyph : FontCharType,
    pub fg : RGB,
    pub bg : RGB,
}

impl Renderable
{
    pub fn new(glyph : FontCharType, fg : RGB, bg : RGB) -> Renderable
    {
        Renderable { glyph, fg, bg }
    }
}

pub struct Name
{
    name : String,
}

impl Name
{
    pub fn new(name : &str) -> Name
    {
        Name { name: name.to_string() }
    }

    pub fn get(&self) -> String
    {
        self.name.clone()
    }
}

pub struct Description
{
    desc : String,
}

impl Description
{
    pub fn new(desc : &str) -> Description
    {
        Description { desc: desc.to_string() }
    }

    pub fn get(&self) -> String
    {
        self.desc.clone()
    }
}

pub struct Position
{
    pub pos : Point,
}

impl Position
{
    pub fn new(x : i32, y : i32) -> Position
    {
        Position{pos : Point::new(x, y)}
    }

    pub fn from_point(pos : Point) -> Position
    {
        Position { pos }
    }
}