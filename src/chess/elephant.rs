use crate::chess::{
    BLACK_LEFT_ELEPHANT_ID, BLACK_RIGHT_ELEPHANT_ID, Chess, ChessKind, ChessTrait,
    RED_LEFT_ELEPHANT_ID, RED_RIGHT_ELEPHANT_ID, same_side,
};
use crate::position::{Position, intersection_option};
use crate::vec2d::Vec2d;
use crate::{pos, vec2d};

const RED_WALK_OPTIONAL_POSITIONS: [Position; 7] = [
    pos!(0, 2),
    pos!(2, 0),
    pos!(2, 4),
    pos!(4, 2),
    pos!(6, 0),
    pos!(6, 4),
    pos!(8, 2),
];

const BLACK_WALK_OPTIONAL_POSITIONS: [Position; 7] = [
    pos!(0, 7),
    pos!(2, 5),
    pos!(2, 9),
    pos!(4, 7),
    pos!(6, 5),
    pos!(6, 9),
    pos!(8, 7),
];

const ELEPHANT_WALK_DIRECTIONS: [Vec2d; 4] =
    [vec2d!(2, 2), vec2d!(-2, 2), vec2d!(-2, -2), vec2d!(2, -2)];

const WALK_OPTIONS_COUNT: usize = 4;

#[derive(Debug, Clone)]
pub struct Elephant(Chess<WALK_OPTIONS_COUNT>);

impl Elephant {
    pub fn new(id: i8) -> Self {
        let pos = match id {
            RED_LEFT_ELEPHANT_ID => pos!(2, 0),
            RED_RIGHT_ELEPHANT_ID => pos!(6, 0),
            BLACK_LEFT_ELEPHANT_ID => pos!(2, 9),
            BLACK_RIGHT_ELEPHANT_ID => pos!(6, 9),
            _ => panic!("invalid id for elephant: {}", id),
        };
        let name = if id > 0 { '相' } else { '象' };
        Self(Chess::new(ChessKind::Elephant, id, true, pos, name))
    }

    pub fn new_with_pos(id: i8, pos: Position) -> Self {
        let name = match id {
            RED_LEFT_ELEPHANT_ID | RED_RIGHT_ELEPHANT_ID => '相',
            BLACK_LEFT_ELEPHANT_ID | BLACK_RIGHT_ELEPHANT_ID => '象',
            _ => panic!("invalid id for elephant: {}", id),
        };
        Self(Chess::new(ChessKind::Elephant, id, true, pos, name))
    }
}

impl ChessTrait for Elephant {
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
        let optinal_positions = if id > 0 {
            RED_WALK_OPTIONAL_POSITIONS
        } else {
            BLACK_WALK_OPTIONAL_POSITIONS
        };

        let cur_pos = self.0.pos;
        // NOTE: the directions here must be the same as those in `ELEPHANT_WALK_DIRECTIONS`
        let potential_obstacles_vec2ds: [Vec2d; 4] =
            [vec2d!(1, 1), vec2d!(-1, 1), vec2d!(-1, -1), vec2d!(1, -1)];
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
        let reachable_positions: [Option<Position>; WALK_OPTIONS_COUNT] =
            std::array::from_fn(|i| {
                if potential_obstacles_exists[i] {
                    None
                } else {
                    cur_pos.checked_add_vec2d(ELEPHANT_WALK_DIRECTIONS[i])
                }
            });
        let walkable_positions = intersection_option(&optinal_positions, &reachable_positions);

        for pos in walkable_positions {
            let other = board_status[pos.x][pos.y];
            if other == 0 || !same_side(id, other) {
                // other = 0, nobody is here, can walk
                // !same_size, an enemy is here, eat him
                self.0.walk_options[self.0.option_count] = Some(pos);
                self.0.option_count += 1;
            }
            // teammate is here, in our way, can not walk
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
    use crate::{board::generate_board, chess::elephant};
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
        // red left elephant
        let elephant_id = RED_LEFT_ELEPHANT_ID;
        let elephant_pos = pos!(2, 0);
        let board = generate_board(vec![(elephant_id, elephant_pos)]);
        let mut elephant = Elephant::new(elephant_id);
        let (walk_options, option_count) = elephant.walk_options(&board);
        assert_eq!(option_count, 2);
        // 2 options: (0,2), (4,2)
        let expected: [Position; 2] = [pos!(0, 2), pos!(4, 2)];
        check_options(&expected, walk_options);

        // black right elephant
        let elephant_id = BLACK_RIGHT_ELEPHANT_ID;
        let elephant_pos = pos!(6, 9);
        let board = generate_board(vec![(elephant_id, elephant_pos)]);
        let mut elephant = Elephant::new(elephant_id);
        let (walk_options, option_count) = elephant.walk_options(&board);
        assert_eq!(option_count, 2);
        // 2 options: (0,2), (4,2)
        let expected: [Position; 2] = [pos!(4, 7), pos!(8, 7)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn unreachable_peices() {
        let elephant_id = BLACK_RIGHT_ELEPHANT_ID;
        let elephant_pos = pos!(8, 7);
        let board = generate_board(vec![
            (elephant_id, elephant_pos),
            (-8, pos!(8, 9)),
            (-9, pos!(7, 9)),
            (-11, pos!(7, 7)),
            (-1, pos!(4, 9)),
        ]);
        let mut elephant = Elephant::new_with_pos(elephant_id, elephant_pos);
        let (walk_options, option_count) = elephant.walk_options(&board);
        assert_eq!(option_count, 2);
        let expected: [Position; 2] = [pos!(6, 9), pos!(6, 5)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn enemy() {
        let elephant_id = RED_RIGHT_ELEPHANT_ID;
        let elephant_pos = pos!(4, 2);
        let board = generate_board(vec![
            (elephant_id, elephant_pos),
            (-5, pos!(6, 4)), // enemy here, can walk
            (4, pos!(2, 0)),  // friend here, cannot walk
        ]);
        let mut elephant = Elephant::new_with_pos(elephant_id, elephant_pos);
        let (walk_options, option_count) = elephant.walk_options(&board);
        assert_eq!(option_count, 3);
        let expected: [Position; 3] = [pos!(6, 0), pos!(2, 4), pos!(6, 4)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn teammate() {
        let elephant_id = BLACK_LEFT_ELEPHANT_ID;
        let elephant_pos = pos!(2, 5);
        let board = generate_board(vec![
            (elephant_id, elephant_pos),
            (-12, pos!(1, 6)), // teammate here, an obstacle
            (-11, pos!(2, 6)), // teammate here, but no effect
            (4, pos!(4, 7)),   // enemy here, eat it
        ]);
        let mut elephant = Elephant::new_with_pos(elephant_id, elephant_pos);
        let (walk_options, option_count) = elephant.walk_options(&board);
        assert_eq!(option_count, 1);
        let expected: [Position; 1] = [pos!(4, 7)];
        check_options(&expected, walk_options);
    }
}
