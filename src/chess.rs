use super::position::Position;
/// this file is for Chess Pieces related
///
/// each chess has a unique id, defined as follows
///
///  单     馬     象     士     将     士     象     馬     单
///  -8 === -6 === -4 === -2 === -1 === -3 === -5 === -7 === -9
///          -10                                      -11
///          鞄                                        鞄
///  -15  =======   -13  ====== -12 =====  -14  =======     -16
///   卒             卒          卒         卒               卒
///
///  +15  =======   +13  ====== +12 =====  +14  =======     +16
///   兵             兵          兵         兵               兵
///          +10                                      +11
///          炮                                        炮
///  +8 === +6 === +4 === +2 === +1 === +3 === +5 === +7 === +9
///  车     马     相     仕     帅     仕     相     马     车
use crate::{board::BoardShape, vec2d::Vec2d};

pub const MAX_CHESS_ID: i8 = 16;
pub const MIN_CHESS_ID: i8 = -16;

pub const RED_KING_ID: i8 = 1;
pub const RED_LEFT_SERVANT_ID: i8 = 2;
pub const RED_RIGHT_SERVANT_ID: i8 = 3;
pub const RED_LEFT_ELEPHANT_ID: i8 = 4;
pub const RED_RIGHT_ELEPHANT_ID: i8 = 5;
pub const RED_LEFT_HORSE_ID: i8 = 6;
pub const RED_RIGHT_HORSE_ID: i8 = 7;
pub const RED_LEFT_CAR_ID: i8 = 8;
pub const RED_RIGHT_CAR_ID: i8 = 9;
pub const RED_LEFT_CANNON_ID: i8 = 10;
pub const RED_RIGHT_CANNON_ID: i8 = 11;
pub const RED_MIDDLE_PAWN_ID: i8 = 12;
pub const RED_MIDDLE_LEFT_PAWN_ID: i8 = 13;
pub const RED_MIDDLE_RIGHT_PAWN_ID: i8 = 14;
pub const RED_LEFTEST_PAWN_ID: i8 = 15;
pub const RED_RIGHTEST_PAWN_ID: i8 = 16;

pub const BLACK_KING_ID: i8 = -1;
pub const BLACK_LEFT_SERVANT_ID: i8 = -2;
pub const BLACK_RIGHT_SERVANT_ID: i8 = -3;
pub const BLACK_LEFT_ELEPHANT_ID: i8 = -4;
pub const BLACK_RIGHT_ELEPHANT_ID: i8 = -5;
pub const BLACK_LEFT_HORSE_ID: i8 = -6;
pub const BLACK_RIGHT_HORSE_ID: i8 = -7;
pub const BLACK_LEFT_CAR_ID: i8 = -8;
pub const BLACK_RIGHT_CAR_ID: i8 = -9;
pub const BLACK_LEFT_CANNON_ID: i8 = -10;
pub const BLACK_RIGHT_CANNON_ID: i8 = -11;
pub const BLACK_MIDDLE_PAWN_ID: i8 = -12;
pub const BLACK_MIDDLE_LEFT_PAWN_ID: i8 = -13;
pub const BLACK_MIDDLE_RIGHT_PAWN_ID: i8 = -14;
pub const BLACK_LEFTEST_PAWN_ID: i8 = -15;
pub const BLACK_RIGHTEST_PAWN_ID: i8 = -16;

mod cannon;
mod car;
mod elephant;
mod horse;
mod king;
mod pawn;
mod servant;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChessKind {
    King,
    Servant,
    Elephant,
    Horse,
    Car,
    Cannon,
    Pawn,
}

pub trait ChessTrait {
    fn walk_options<'a>(&'a mut self, board_status: &BoardShape)
    -> (&'a [Option<Position>], usize);
    fn walk(&mut self, direction: Vec2d) -> bool;
    fn killed(&mut self);
    fn is_alive(&self) -> bool;
    fn get_name(&self) -> char;
}

#[derive(Debug, Clone)]
pub struct Chess<const N: usize> {
    job: ChessKind,
    id: i8, // positive for red, negative for black
    is_alive: bool,
    pos: Position,
    name: char,
    walk_options: [Option<Position>; N],
    option_count: usize,
}

impl<const N: usize> Chess<N> {
    pub fn new(job: ChessKind, id: i8, is_alive: bool, pos: Position, name: char) -> Self {
        Self {
            job,
            id,
            is_alive,
            pos,
            name,
            walk_options: [None; N],
            option_count: 0,
        }
    }

    pub fn killed(&mut self) {
        self.is_alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn get_name(&self) -> char {
        self.name
    }
}

pub fn same_side(x: i8, y: i8) -> bool {
    assert!(
        x != 0 && y != 0,
        "x:({}) and y:({}) should not be zero!",
        x,
        y
    );
    (x < 0) == (y < 0)
}
