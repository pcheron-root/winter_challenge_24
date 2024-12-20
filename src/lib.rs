pub mod arena;
pub mod fib;
pub mod player;

pub use arena::Arena;
pub use player::Player;

pub const WALL: i32 = 0;
pub const ROOT: i32 = 1;
pub const BASIC: i32 = 2;
pub const TENTACLE: i32 = 3;
pub const HARVESTER: i32 = 4;
pub const SPORER: i32 = 5;
pub const A: i32 = 6;
pub const B: i32 = 7;
pub const C: i32 = 8;
pub const D: i32 = 9;
pub const UNKNOWN: i32 = 10;

pub const MINE: i32 = 64;
pub const ENEMY: i32 = 32;
