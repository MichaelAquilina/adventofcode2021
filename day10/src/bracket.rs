use std::error::Error;

#[derive(Debug, PartialEq)]
enum Bracket {
    Round,
    Curly,
    Square,
    Pointed,
}

#[derive(Debug)]
enum BracketType {
    Open(Bracket),
    Close(Bracket),
}

#[derive(Debug, thiserror::Error)]
enum BracketError {
    #[error("Invalid bracket: {0}")]
    InvalidBracket(char),
}

#[derive(Debug)]
enum LineType {
    Corrupted(u64),
    Incomplete(u64),
}

#[derive(Debug)]
pub struct LineScores {
    pub corrupted: u64,
    pub incomplete: u64,
}

impl Bracket {
    fn get_corrupted_score(&self) -> u64 {
        match self {
            Bracket::Pointed => 25137,
            Bracket::Curly => 1197,
            Bracket::Square => 57,
            Bracket::Round => 3,
        }
    }

    fn get_incomplete_score(&self) -> u64 {
        match self {
            Bracket::Pointed => 4,
            Bracket::Curly => 3,
            Bracket::Square => 2,
            Bracket::Round => 1,
        }
    }
}

impl BracketType {
    fn from_char(character: &char) -> Result<Self, BracketError> {
        Ok(match character {
            '<' => BracketType::Open(Bracket::Pointed),
            '(' => BracketType::Open(Bracket::Round),
            '[' => BracketType::Open(Bracket::Square),
            '{' => BracketType::Open(Bracket::Curly),
            '>' => BracketType::Close(Bracket::Pointed),
            ')' => BracketType::Close(Bracket::Round),
            ']' => BracketType::Close(Bracket::Square),
            '}' => BracketType::Close(Bracket::Curly),
            value => return Err(BracketError::InvalidBracket(*value)),
        })
    }
}

pub fn calculate_scores(value: &str) -> Result<LineScores, Box<dyn Error>> {
    let mut corrupted = 0;
    let mut incomplete = vec![];

    for line in value.lines() {
        match get_line_type(line)? {
            LineType::Corrupted(score) => corrupted += score,
            LineType::Incomplete(score) => incomplete.push(score),
        }
    }

    incomplete.sort_unstable();
    let incomplete = incomplete[incomplete.len() / 2];
    Ok(LineScores {
        corrupted,
        incomplete,
    })
}

fn get_line_type(value: &str) -> Result<LineType, Box<dyn Error>> {
    let mut stack: Vec<Bracket> = vec![];

    for character in value.chars() {
        match BracketType::from_char(&character)? {
            BracketType::Open(bracket) => {
                stack.push(bracket);
            }
            BracketType::Close(bracket) => {
                if let Some(value) = stack.pop() {
                    if value != bracket {
                        let score = bracket.get_corrupted_score();
                        return Ok(LineType::Corrupted(score));
                    }
                }
            }
        };
    }

    // calculate incomplete
    let mut score = 0;
    while let Some(bracket) = stack.pop() {
        score *= 5;
        score += bracket.get_incomplete_score();
    }

    Ok(LineType::Incomplete(score))
}

#[cfg(test)]
mod test_brackets {
    use super::*;

    #[test]
    fn test_example() -> Result<(), Box<dyn Error>> {
        let example = [
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
        .join("\n");

        let result = calculate_scores(&example)?;
        assert_eq!(result.corrupted, 26397);
        assert_eq!(result.incomplete, 288957);

        Ok(())
    }
}
