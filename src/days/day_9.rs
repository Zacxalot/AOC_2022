use crate::Answer;
use std::{collections::HashSet, fmt::Display, fs, time::Instant};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Relation {
    Overlapping,
    Diagonal,
    SameRow,
    SameColumn,
}

struct Movement {
    direction: Direction,
    distance: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn get_relation(&self, target: &Position) -> Relation {
        let share_x = self.x == target.x;
        let share_y = self.y == target.y;

        match (share_x, share_y) {
            (true, true) => Relation::Overlapping,
            (true, false) => Relation::SameColumn,
            (false, true) => Relation::SameRow,
            (false, false) => Relation::Diagonal,
        }
    }

    fn move_to(&mut self, target: &Position) {
        if (self.x - target.x).abs() <= 1 && (self.y - target.y).abs() <= 1 {
            return;
        }

        let relation = self.get_relation(target);

        match relation {
            Relation::Overlapping => (),
            Relation::SameRow => {
                if self.x > target.x {
                    self.x -= 1
                } else {
                    self.x += 1
                }
            }
            Relation::SameColumn => {
                if self.y > target.y {
                    self.y -= 1
                } else {
                    self.y += 1
                }
            }
            Relation::Diagonal => {
                if self.y > target.y {
                    self.y -= 1;
                    if self.x > target.x {
                        self.x -= 1;
                    } else {
                        self.x += 1;
                    }
                } else {
                    self.y += 1;
                    if self.x > target.x {
                        self.x -= 1;
                    } else {
                        self.x += 1;
                    }
                }
            }
        };
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub fn execute() -> Answer {
    let time_before = Instant::now();
    let file = fs::read_to_string("./input/day_9.txt").unwrap();
    let time_no_io = Instant::now();

    let movements = file
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let direction = match split.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                &_ => panic!("Invalid Input Day 9"),
            };
            let distance = split.next().unwrap().parse::<i32>().unwrap();
            Movement {
                direction,
                distance,
            }
        })
        .collect::<Vec<Movement>>();

    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    let mut tail_set: HashSet<Position> = HashSet::new();

    for movement in movements {
        for _i in 0..movement.distance {
            match movement.direction {
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            };

            tail.move_to(&head);

            tail_set.insert(tail.clone());
        }
    }

    let part_1 = tail_set.len().to_string();
    let part_2 = "2".to_string();

    let duration = Instant::now() - time_before;
    let no_io_duration = Instant::now() - time_no_io;

    Answer {
        day: 9,
        part_1,
        part_2,
        duration,
        no_io_duration,
    }
}
