pub use crate::prelude::*;

/// Rendering 2D glyphs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

/// Our hero
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

/// Enemies
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

/// Random movement of entities
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);
