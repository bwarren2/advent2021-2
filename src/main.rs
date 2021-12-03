use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Forward,
    Down,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    amount: i32,
}

impl FromStr for Command {
    type Err = std::str::ParseBoolError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.chars().count() > 7 && &input[..7] == "forward" {
            return Ok(Command {
                direction: Direction::Forward,
                amount: input[7..].trim().parse::<i32>().unwrap(),
            });
        } else if input.chars().count() > 4 && &input[..4] == "down" {
            return Ok(Command {
                direction: Direction::Down,
                amount: input[4..].trim().parse::<i32>().unwrap(),
            });
        } else {
            return Ok(Command {
                direction: Direction::Up,
                amount: input[2..].trim().parse::<i32>().unwrap(),
            });
        }
    }
}

struct Position {
    vert: i32,
    horz: i32,
}

impl Position {
    fn adjust(&mut self, c: &Command) {
        if c.direction == Direction::Up {
            self.vert += c.amount
        }
        if c.direction == Direction::Down {
            self.vert -= c.amount
        }
        if c.direction == Direction::Forward {
            self.horz += c.amount
        }
    }
    fn prod(&self) -> i32 {
        (self.horz * self.vert).abs()
    }
}

fn main() {
    let mut position = Position { vert: 0, horz: 0 };
    let commands = include_str!("commands.txt")
        .lines()
        .map(|val| val.parse().unwrap())
        .collect::<Vec<Command>>();
    for command in commands {
        position.adjust(&command);
    }
    println!("{:?}", position.prod());
}
