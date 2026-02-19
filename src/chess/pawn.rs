use crate::chess::{
    BLACK_LEFTEST_PAWN_ID, BLACK_MIDDLE_LEFT_PAWN_ID, BLACK_MIDDLE_PAWN_ID,
    BLACK_MIDDLE_RIGHT_PAWN_ID, BLACK_RIGHTEST_PAWN_ID, Chess, ChessKind, ChessTrait,
    RED_LEFTEST_PAWN_ID, RED_MIDDLE_LEFT_PAWN_ID, RED_MIDDLE_PAWN_ID, RED_MIDDLE_RIGHT_PAWN_ID,
    RED_RIGHTEST_PAWN_ID, same_side,
};
use crate::position::Position;
use crate::vec2d::Vec2d;
use crate::{pos, vec2d};

const RED_WALK_DIRECTIONS: [Vec2d; 3] = [vec2d!(1, 0), vec2d!(0, 1), vec2d!(-1, 0)];
const BLACK_WALK_DIRECTINOS: [Vec2d; 3] = [vec2d!(1, 0), vec2d!(0, -1), vec2d!(-1, 0)];
const WALK_OPTIONS_COUNT: usize = 3;

#[derive(Debug, Clone)]
pub struct Pawn(Chess<WALK_OPTIONS_COUNT>);

impl Pawn {
    pub fn new(id: i8) -> Self {
        let pos = match id {
            RED_MIDDLE_PAWN_ID => pos!(4, 3),
            RED_MIDDLE_LEFT_PAWN_ID => pos!(2, 3),
            RED_MIDDLE_RIGHT_PAWN_ID => pos!(6, 3),
            RED_LEFTEST_PAWN_ID => pos!(0, 3),
            RED_RIGHTEST_PAWN_ID => pos!(8, 3),
            BLACK_MIDDLE_PAWN_ID => pos!(4, 6),
            BLACK_MIDDLE_LEFT_PAWN_ID => pos!(2, 6),
            BLACK_MIDDLE_RIGHT_PAWN_ID => pos!(6, 6),
            BLACK_LEFTEST_PAWN_ID => pos!(0, 6),
            BLACK_RIGHTEST_PAWN_ID => pos!(8, 6),
            _ => panic!("invalid id for pawn: {}", id),
        };
        let name = if id > 0 { '兵' } else { '卒' };
        Self(Chess::new(ChessKind::Pawn, id, true, pos, name))
    }

    pub fn new_with_pos(id: i8, pos: Position) -> Self {
        let name = match id {
            RED_MIDDLE_PAWN_ID
            | RED_MIDDLE_LEFT_PAWN_ID
            | RED_MIDDLE_RIGHT_PAWN_ID
            | RED_LEFTEST_PAWN_ID
            | RED_RIGHTEST_PAWN_ID => '兵',
            BLACK_MIDDLE_PAWN_ID
            | BLACK_MIDDLE_LEFT_PAWN_ID
            | BLACK_MIDDLE_RIGHT_PAWN_ID
            | BLACK_LEFTEST_PAWN_ID
            | BLACK_RIGHTEST_PAWN_ID => '卒',
            _ => panic!("invalid id for pawn: {}", id),
        };
        Self(Chess::new(ChessKind::Pawn, id, true, pos, name))
    }
}

impl ChessTrait for Pawn {
    fn killed(&mut self) {
        self.0.killed();
    }
    fn is_alive(&self) -> bool {
        self.0.is_alive()
    }
    fn get_name(&self) -> char {
        self.0.get_name()
    }
    fn walk_options<'a>(
        &'a mut self,
        board_status: &crate::board::BoardShape,
    ) -> (&'a [Option<Position>], usize) {
        let id = self.0.id;
        let cur_pos = self.0.pos;
        let optional_directions: &[Vec2d] = if id > 0 {
            if cur_pos.y <= 4 {
                &[vec2d!(0, 1)]
            } else {
                &RED_WALK_DIRECTIONS
            }
        } else {
            if cur_pos.y >= 5 {
                &[vec2d!(0, -1)]
            } else {
                &BLACK_WALK_DIRECTINOS
            }
        };

        for direction in optional_directions {
            if let Some(pos) = cur_pos.checked_add_vec2d(*direction) {
                let other = board_status[pos.x][pos.y];
                if other == 0 || !same_side(id, other) {
                    // other == 0, nobody is here, can walk
                    // !same_side, an enemy is here, eat him
                    self.0.walk_options[self.0.option_count] = Some(pos);
                    self.0.option_count += 1;
                }
                // teamate is here, in our way , cannot walk
            }
            // not a walkable direction
        }
        (&self.0.walk_options, self.0.option_count)
    }

    fn walk(&mut self, direction: Vec2d) -> bool {
        let new_pos = self.0.pos + direction;
        for pos in self.0.walk_options {
            if let Some(pos_) = pos
                && pos_ == new_pos
            {
                // can walk
                self.0.pos = new_pos;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::generate_board;
    use std::collections::HashSet;

    fn check_options(expected: &[Position], calculated: &[Option<Position>]) {
        let calculated_set: HashSet<Position> = calculated
            .iter()
            .filter_map(|opt| opt.as_ref())
            .copied()
            .collect();
        for pos in expected {
            assert!(
                calculated_set.contains(&pos),
                "calculated does not contains pos: {}",
                pos
            );
        }
    }

    #[test]
    fn empty() {
        // red middle pawn
        let pawn_id = RED_MIDDLE_PAWN_ID;
        let pawn_pos = pos!(4, 3);
        let board = generate_board(vec![(pawn_id, pawn_pos)]);
        let mut pawn = Pawn::new(pawn_id);
        let (walk_options, option_count) = pawn.walk_options(&board);
        assert_eq!(option_count, 1);
        let expected: [Position; 1] = [pos!(4, 4)];
        check_options(&expected, walk_options);

        // black middle pawn
        let pawn_id = BLACK_MIDDLE_PAWN_ID;
        let pawn_pos = pos!(4, 6);
        let board = generate_board(vec![(pawn_id, pawn_pos)]);
        let mut pawn = Pawn::new(pawn_id);
        let (walk_options, option_count) = pawn.walk_options(&board);
        assert_eq!(option_count, 1);
        let expected: [Position; 1] = [pos!(4, 5)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn unreachable() {
        // black middle left pawn
        let pawn_id = BLACK_MIDDLE_LEFT_PAWN_ID;
        let pawn_pos = pos!(2, 4);
        let board = generate_board(vec![
            (pawn_id, pawn_pos),
            (-9, pos!(2, 5)),
            (8, pos!(1, 3)),
            (11, pos!(2, 2)),
        ]);
        let mut pawn = Pawn::new_with_pos(pawn_id, pawn_pos);
        let (walk_options, option_count) = pawn.walk_options(&board);
        assert_eq!(option_count, 3);
        let expected: [Position; 3] = [pos!(2, 3), pos!(1, 4), pos!(3, 4)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn enemy() {
        let pawn_id = RED_RIGHTEST_PAWN_ID;
        let pawn_pos = pos!(8, 4);
        let board = generate_board(vec![
            (pawn_id, pawn_pos),
            (-14, pos!(8, 5)), // enemy here, eat him
            (5, pos!(7, 4)),
        ]);
        let mut pawn = Pawn::new_with_pos(pawn_id, pawn_pos);
        let (walk_options, option_count) = pawn.walk_options(&board);
        assert_eq!(option_count, 1);
        let expected: [Position; 1] = [pos!(8, 5)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn teamate() {
        let pawn_id = BLACK_MIDDLE_RIGHT_PAWN_ID;
        let pawn_pos = pos!(6, 0);
        let board = generate_board(vec![(pawn_id, pawn_pos), (8, pos!(6, 1)), (7, pos!(5, 0))]);
        let mut pawn = Pawn::new_with_pos(pawn_id, pawn_pos);
        let (walk_options, option_count) = pawn.walk_options(&board);
        assert_eq!(option_count, 2);
        let expected: [Position; 2] = [pos!(5, 0), pos!(7, 0)];
        check_options(&expected, walk_options);
    }
}
