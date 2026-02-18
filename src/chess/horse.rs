use crate::chess::{
    BLACK_LEFT_HORSE_ID, BLACK_RIGHT_HORSE_ID, Chess, ChessKind, ChessTrait, RED_LEFT_HORSE_ID,
    RED_RIGHT_HORSE_ID, same_side,
};
use crate::position::Position;
use crate::vec2d::Vec2d;
use crate::{pos, vec2d};

const HORSE_WALK_DIRECTIONS: [Vec2d; 8] = [
    // right
    vec2d!(2, 1),
    // up
    vec2d!(1, 2),
    // left
    vec2d!(-2, 1),
    // down
    vec2d!(-1, -2),
    // right
    vec2d!(2, -1),
    // up
    vec2d!(-1, 2),
    // left
    vec2d!(-2, -1),
    // down
    vec2d!(1, -2),
];

const WALK_OPTIONS_COUNT: usize = 9;

#[derive(Debug, Clone)]
pub struct Horse(Chess<WALK_OPTIONS_COUNT>);

impl Horse {
    pub fn new(id: i8) -> Self {
        let pos = match id {
            RED_LEFT_HORSE_ID => pos!(1, 0),
            RED_RIGHT_HORSE_ID => pos!(7, 0),
            BLACK_LEFT_HORSE_ID => pos!(1, 9),
            BLACK_RIGHT_HORSE_ID => pos!(7, 9),
            _ => panic!("invalid id for servant: {}", id),
        };
        let name = if id > 0 { '马' } else { '馬' };
        Self(Chess::new(ChessKind::Horse, id, true, pos, name))
    }

    pub fn new_with_pos(id: i8, pos: Position) -> Self {
        let name = match id {
            RED_LEFT_HORSE_ID | RED_RIGHT_HORSE_ID => '马',
            BLACK_LEFT_HORSE_ID | BLACK_RIGHT_HORSE_ID => '馬',
            _ => panic!("invalid id for servant: {}", id),
        };
        Self(Chess::new(ChessKind::Horse, id, true, pos, name))
    }
}

impl ChessTrait for Horse {
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
        let potential_obstacles_vec2ds: [Vec2d; 4] =
            [vec2d!(1, 0), vec2d!(0, 1), vec2d!(-1, 0), vec2d!(0, -1)];
        let potential_obstacles_exists: [bool; 4] = std::array::from_fn(|i| {
            if let Some(obstacle) = cur_pos.checked_add_vec2d(potential_obstacles_vec2ds[i])
                && board_status[obstacle.x][obstacle.y] != 0
            // valid position & other piece
            {
                true
            } else {
                false
            }
        });
        for (i, &direct) in HORSE_WALK_DIRECTIONS.iter().enumerate() {
            if potential_obstacles_exists[i % 4] {
                continue;
            }
            if let Some(pos) = cur_pos.checked_add_vec2d(direct) {
                let other = board_status[pos.x][pos.y];
                if other == 0 || !same_side(id, other) {
                    // other == 0, nobody is here, can walk
                    // !same_side, an enemy is here, eat him
                    self.0.walk_options[self.0.option_count] = Some(pos);
                    self.0.option_count += 1;
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
                // can walk
                self.0.pos = new_pos;
                return true;
            }
        }
        return false;
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
        // red left horse
        let horse_id = RED_LEFT_HORSE_ID;
        let horse_pos = pos!(1, 0);
        let board = generate_board(vec![(horse_id, horse_pos)]);
        let mut horse = Horse::new(horse_id);
        let (walk_options, option_count) = horse.walk_options(&board);
        assert_eq!(option_count, 3);
        let expected: [Position; 3] = [pos!(3, 1), pos!(2, 2), pos!(0, 2)];
        check_options(&expected, walk_options);

        // black right horse
        let horse_id = BLACK_RIGHT_HORSE_ID;
        let horse_pos = pos!(7, 9);
        let board = generate_board(vec![(horse_id, horse_pos)]);
        let mut horse = Horse::new(horse_id);
        let (walk_options, option_count) = horse.walk_options(&board);
        assert_eq!(option_count, 3);
        let expected: [Position; 3] = [pos!(8, 7), pos!(6, 7), pos!(5, 8)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn unreachable() {
        // red right horse
        let horse_id = RED_RIGHT_HORSE_ID;
        let horse_pos = pos!(6, 2);
        let board = generate_board(vec![
            (horse_id, horse_pos),
            (2, pos!(3, 2)),
            (13, pos!(6, 4)),
            (10, pos!(4, 2)),
            (-5, pos!(3, 7)),
        ]);
        let mut horse = Horse::new_with_pos(horse_id, horse_pos);
        let (walk_options, option_count) = horse.walk_options(&board);
        assert_eq!(option_count, 8);
        let expected: [Position; 8] = [
            pos!(7, 4),
            pos!(8, 3),
            pos!(5, 4),
            pos!(4, 3),
            pos!(4, 1),
            pos!(5, 0),
            pos!(7, 0),
            pos!(8, 1),
        ];
        check_options(&expected, walk_options);
    }

    #[test]
    fn enemy() {
        let horse_id = BLACK_LEFT_HORSE_ID;
        let horse_pos = pos!(1, 5);
        let board = generate_board(vec![
            (horse_id, horse_pos),
            (11, pos!(2, 3)),  // enemy to eat
            (12, pos!(0, 4)),
            (-11, pos!(2, 5)),  // hinder
            (-12, pos!(0, 6)),
        ]);
        let mut horse = Horse::new_with_pos(horse_id, horse_pos);
        let (walk_options, option_count) = horse.walk_options(&board);
        assert_eq!(option_count, 4);
        let expected: [Position; 4] = [pos!(2, 3), pos!(0, 3), pos!(0, 7), pos!(2, 7)];
        check_options(&expected, walk_options);
    }
}
