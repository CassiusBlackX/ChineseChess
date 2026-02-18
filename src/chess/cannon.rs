use crate::chess::{
    BLACK_LEFT_CANNON_ID, BLACK_RIGHT_CANNON_ID, Chess, ChessKind, ChessTrait, RED_LEFT_CANNON_ID,
    RED_RIGHT_CANNON_ID, same_side,
};
use crate::position::Position;
use crate::vec2d::Vec2d;
use crate::{pos, vec2d};

const CANNON_WALK_DIRECTIONS: [Vec2d; 4] =
    [vec2d!(1, 0), vec2d!(0, 1), vec2d!(-1, 0), vec2d!(0, -1)];

const MAX_WALK_OPTIONS_COUNT: usize = 17;

#[derive(Debug, Clone)]
pub struct Cannon(Chess<MAX_WALK_OPTIONS_COUNT>);

impl Cannon {
    pub fn new(id: i8) -> Self {
        let pos = match id {
            RED_LEFT_CANNON_ID => pos!(1, 2),
            RED_RIGHT_CANNON_ID => pos!(7, 2),
            BLACK_LEFT_CANNON_ID => pos!(1, 7),
            BLACK_RIGHT_CANNON_ID => pos!(7, 7),
            _ => panic!("invalid id for cannon: {}", id),
        };
        let name = if id > 0 { '炮' } else { '鞄' };
        Self(Chess::new(ChessKind::Cannon, id, true, pos, name))
    }

    pub fn new_with_pos(id: i8, pos: Position) -> Self {
        let name = match id {
            RED_LEFT_CANNON_ID | RED_RIGHT_CANNON_ID => '炮',
            BLACK_LEFT_CANNON_ID | BLACK_RIGHT_CANNON_ID => '鞄',
            _ => panic!("invalid id for cannon: {}", id),
        };
        Self(Chess::new(ChessKind::Cannon, id, true, pos, name))
    }
}

impl ChessTrait for Cannon {
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
        for direction in CANNON_WALK_DIRECTIONS {
            let mut new_pos = cur_pos;
            while let Some(new_pos_) = new_pos.checked_add_vec2d(direction) {
                let other = board_status[new_pos_.x][new_pos_.y];
                if other == 0 {
                    // nobody here, can walk
                    new_pos = new_pos_;
                    self.0.walk_options[self.0.option_count] = Some(new_pos);
                    self.0.option_count += 1;
                } else {
                    // somebody here, can no longer walk
                    // unless an enemy is furthur
                    let mut forward_pos = new_pos_;
                    while let Some(forward_pos_) = forward_pos.checked_add_vec2d(direction) {
                        let other = board_status[forward_pos_.x][forward_pos_.y];
                        if other == 0 {
                            forward_pos = forward_pos_;
                            continue;
                        } else if same_side(id, other) {
                            break;
                        } else {
                            self.0.walk_options[self.0.option_count] = Some(forward_pos_);
                            self.0.option_count += 1;
                            break;
                        }
                    }
                    // break outer while loop
                    // we should no longer walk current direction anymore
                    break;
                }
            }
        }
        (&self.0.walk_options, self.0.option_count)
    }

    fn walk(&mut self, direction: Vec2d) -> bool {
        let new_pos = self.0.pos + direction;
        for pos in self.0.walk_options {
            if let Some(pos_) = pos
                && pos_ == new_pos
            {
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
        // red left cannon
        let cannon_id = RED_LEFT_CANNON_ID;
        let cannon_pos = pos!(1, 2);
        let board = generate_board(vec![(cannon_id, cannon_pos)]);
        let mut cannon = Cannon::new(cannon_id);
        let (walk_options, option_count) = cannon.walk_options(&board);
        assert_eq!(option_count, 17);
        let expected: [Position; 17] = std::array::from_fn(|i| {
            if i < 7 {
                pos!(i + 2, 2)
            } else if i < 14 {
                pos!(1, i - 4)
            } else if i < 15 {
                pos!(0, 2)
            } else {
                pos!(1, i - 15)
            }
        });
        check_options(&expected, walk_options);
    }

    #[test]
    fn unreachable() {
        // red right cannon
        let cannon_id = RED_RIGHT_CANNON_ID;
        let cannon_pos = pos!(7, 2);
        let board = generate_board(vec![
            (cannon_id, cannon_pos),
            (8, pos!(8, 0)),
            (13, pos!(8, 3)),
            (-8, pos!(7, 7)), // nothing behind, therefore stop at (7,6)
        ]);
        let mut cannon = Cannon::new(RED_RIGHT_CANNON_ID);
        let (walk_options, option_count) = cannon.walk_options(&board);
        assert_eq!(option_count, 1 + 4 + 7 + 2);
        let expected: [Position; 1 + 4 + 7 + 2] = std::array::from_fn(|i| {
            if i < 1 {
                pos!(8, 2)
            } else if i < 5 {
                pos!(7, i + 2)
            } else if i < 12 {
                pos!(i - 5, 2)
            } else {
                pos!(7, i - 12)
            }
        });
        check_options(&expected, walk_options);
    }

    #[test]
    fn enemy() {
        // black left cannon
        let cannon_id = BLACK_LEFT_CANNON_ID;
        let cannon_pos = pos!(4, 7);
        let board = generate_board(vec![
            (cannon_id, cannon_pos),
            (-15, pos!(4, 6)),
            (15, pos!(4, 3)), // enemy here, 1 option
            (-8, pos!(7, 7)), // teammate here, 2 options
            (-1, pos!(4, 9)), // teammate here, 1 optino
        ]);
        let mut cannon = Cannon::new_with_pos(cannon_id, cannon_pos);
        let (walk_options, option_count) = cannon.walk_options(&board);
        assert_eq!(option_count, 1 + 2 + 1 + 4);
        let expected: [Position; 1 + 2 + 1 + 4] = std::array::from_fn(|i| {
            if i < 1 {
                pos!(4, 3)
            } else if i < 3 {
                pos!(i + 4, 7)
            } else if i < 4 {
                pos!(4, 8)
            } else {
                pos!(i - 4, 7)
            }
        });
        check_options(&expected, walk_options);
    }

    #[test]
    fn teammate() {
        // black right cannon
        let cannon_id = BLACK_RIGHT_CANNON_ID;
        let cannon_pos = pos!(7, 7);
        let board = generate_board(vec![
            (cannon_id, cannon_pos),
            (-8, pos!(8, 7)),  // stuck its right
            (-9, pos!(7, 8)),  // stuck its up
            (-10, pos!(6, 7)), // stuck its left
            (8, pos!(7, 2)),   // first enemy, can not eat, but bridge
            (9, pos!(7, 1)),   // second enemy, can eat
            (10, pos!(7, 0)),  // thrid enemy, unreachable
        ]);
        let mut cannon = Cannon::new(cannon_id);
        let (walk_options, option_count) = cannon.walk_options(&board);
        assert_eq!(option_count, 5);
        let expected: [Position; 5] =
            std::array::from_fn(|i| if i < 3 { pos!(7, i + 3) } else { pos!(7, 1) });
        check_options(&expected, walk_options);
    }
}
