use crate::chess::{
    BLACK_LEFT_CAR_ID, BLACK_RIGHT_CAR_ID, Chess, ChessKind, ChessTrait, RED_LEFT_CAR_ID,
    RED_RIGHT_CAR_ID, same_side,
};
use crate::position::Position;
use crate::vec2d::Vec2d;
use crate::{pos, vec2d};

const CAR_WALK_DIRECTIONS: [Vec2d; 4] = [vec2d!(1, 0), vec2d!(0, 1), vec2d!(-1, 0), vec2d!(0, -1)];

const MAX_WALK_OPTIONS_COUNT: usize = 17;

#[derive(Debug, Clone)]
pub struct Car(Chess<MAX_WALK_OPTIONS_COUNT>);

impl Car {
    pub fn new(id: i8) -> Self {
        let pos = match id {
            RED_LEFT_CAR_ID => pos!(0, 0),
            RED_RIGHT_CAR_ID => pos!(8, 0),
            BLACK_LEFT_CAR_ID => pos!(9, 0),
            BLACK_RIGHT_CAR_ID => pos!(8, 9),
            _ => panic!("invalid id for car: {}", id),
        };
        let name = if id > 0 { '车' } else { '单' };
        Self(Chess::new(ChessKind::Car, id, true, pos, name))
    }

    pub fn new_with_pos(id: i8, pos: Position) -> Self {
        let name = match id {
            RED_LEFT_CAR_ID | RED_RIGHT_CAR_ID => '车',
            BLACK_LEFT_CAR_ID | BLACK_RIGHT_CAR_ID => '单',
            _ => panic!("invalid id for car: {}", id),
        };
        Self(Chess::new(ChessKind::Car, id, true, pos, name))
    }
}

impl ChessTrait for Car {
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
        for direction in CAR_WALK_DIRECTIONS {
            let mut new_pos = cur_pos;
            while let Some(new_pos_) = new_pos.checked_add_vec2d(direction) {
                let other = board_status[new_pos_.x][new_pos_.y];
                if other == 0 || !same_side(id, other) {
                    // nobody or enemy here, can walk
                    new_pos = new_pos_;
                    self.0.walk_options[self.0.option_count] = Some(new_pos);
                    self.0.option_count += 1;
                    if other != 0 {
                        // enemy here, can walk to here, but no more
                        break;
                    }
                } else {
                    // teammate here, unable to walk
                    break;
                }
            }
            // unable to walk along with this direction anymore
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
        // red left car
        let car_id = RED_LEFT_CAR_ID;
        let car_pos = pos!(0, 0);
        let board = generate_board(vec![(car_id, car_pos)]);
        let mut car = Car::new(car_id);
        let (walk_options, option_count) = car.walk_options(&board);
        assert_eq!(option_count, 17);
        let expected: [Position; 17] = std::array::from_fn(|i| {
            if i < 9 {
                pos!(0, i + 1)
            } else {
                pos!(i - 8, 0)
            }
        });
        check_options(&expected, walk_options);
    }

    #[test]
    fn unreachable() {
        // red right car
        let car_id = RED_RIGHT_CAR_ID;
        let car_pos = pos!(4, 2);
        let board = generate_board(vec![
            (car_id, car_pos),
            (-2, pos!(3, 3)),
            (8, pos!(7, 7)),
            (11, pos!(6, 8)),
            (-6, pos!(1, 1)),
        ]);
        let mut car = Car::new_with_pos(car_id, car_pos);
        let (walk_options, option_count) = car.walk_options(&board);
        assert_eq!(option_count, 17);
        let expected: [Position; 17] = std::array::from_fn(|i| {
            if i < 4 {
                pos!(i+5,2)
            } else if i < 11 {
                pos!(4, i-1)
            } else if i < 15 {
                pos!(i-11, 2)
            } else {
                pos!(4, i-15)
            }
        });
        check_options(&expected, walk_options);
    }

    #[test]
    fn enemy() {
        let car_id = BLACK_LEFT_CAR_ID;
        let car_pos = pos!(3, 6);
        let board = generate_board(vec![
            (car_id, car_pos),
            (11, pos!(2, 6)),
            (3, pos!(3, 1)),
            (8, pos!(7, 6)),
            (-8, pos!(4, 7)),
        ]);
        let mut car = Car::new_with_pos(car_id, car_pos);
        let (walk_options, option_count) = car.walk_options(&board);
        assert_eq!(option_count, 1+5+4+3);
        let expected: [Position; 1+5+4+3] = std::array::from_fn(|i| {
            if i < 1 {
                pos!(2, 6)
            } else if i < 1 + 5 {
                pos!(3, i)
            } else if i < 1+5+4 {
                pos!(i-2,6)
            } else {
                pos!(3, i-3)
            }
        });
        check_options(&expected, walk_options);
    }

    #[test]
    fn teammate() {
        let car_id = BLACK_RIGHT_CAR_ID;
        let car_pos = pos!(4,4);
        let board = generate_board(vec![
            (car_id, car_pos),
            (12, pos!(4, 3)),  // 1
            (-11, pos!(4, 6)),  // 1
            (-6, pos!(3, 4)),  // 0
        ]);
        let mut car = Car::new_with_pos(car_id, car_pos);
        let(walk_options, option_count) = car.walk_options(&board);
        assert_eq!(option_count, 1+1+4);
        let expected: [Position; 1+1+4] = std::array::from_fn(|i| {
            if i < 1 {
                pos!(4,3)
            } else if i < 2 {
                pos!(4,5)
            } else {
                pos!(5+i-2,4)
            }
        });
        check_options(&expected, walk_options);
    }
}
