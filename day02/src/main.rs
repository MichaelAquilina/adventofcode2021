use std::error::Error;
use std::io::Read;

#[derive(Debug)]
enum Command {
    Up(u32),
    Down(u32),
    Forward(u32),
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, thiserror::Error)]
enum CommandError {
    #[error("Invalid command")]
    InvalidCommand,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let commands = read_commands(&buffer)?;

    let position = execute_part1(&commands);
    println!("Part 1: {} ({:?})", position.x * position.y, position);

    let position = execute_part2(&commands);
    println!("Part 2: {} ({:?})", position.x * position.y, position);

    Ok(())
}

fn read_commands(content: &str) -> Result<Vec<Command>, Box<dyn Error>> {
    let mut commands = vec![];
    for line in content.lines() {
        let mut tokens = line.split(' ');
        let command_type = tokens.next().ok_or("Missing command type")?;
        let value: u32 = tokens.next().ok_or("Missing command value")?.parse()?;

        let command = match command_type {
            "forward" => Command::Forward(value),
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            _ => return Err(Box::new(CommandError::InvalidCommand)),
        };
        commands.push(command);
    }

    Ok(commands)
}

fn execute_part1(commands: &[Command]) -> Position {
    let mut position = Position { x: 0, y: 0 };
    for command in commands {
        match command {
            Command::Up(value) => position.y -= *value as i32,
            Command::Down(value) => position.y += *value as i32,
            Command::Forward(value) => position.x += *value as i32,
        }
    }

    position
}

fn execute_part2(commands: &[Command]) -> Position {
    let mut aim = 0;
    let mut position = Position { x: 0, y: 0 };
    for command in commands {
        match command {
            Command::Up(value) => aim -= *value as i32,
            Command::Down(value) => aim += *value as i32,
            Command::Forward(value) => {
                position.x += *value as i32;
                position.y += (*value as i32) * aim;
            }
        }
    }

    position
}
