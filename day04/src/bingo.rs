use std::error::Error;

type Point = u32;

#[derive(Debug, PartialEq)]
pub struct BingoCard {
    pub width: usize,
    pub height: usize,
    pub values: Vec<Point>,
}

#[derive(Debug)]
pub struct BingoGame {
    pub order: Vec<Point>,
    pub cards: Vec<BingoCard>,
}

#[derive(Debug)]
pub struct BingoResult<'a> {
    pub called: &'a [Point],
    pub card: &'a BingoCard,
}

impl BingoGame {
    pub fn parse(value: &str, width: usize, height: usize) -> Result<BingoGame, Box<dyn Error>> {
        let mut lines = value.lines();

        let order_line = lines.next().ok_or("Missing order")?;
        let mut order = vec![];
        for point in order_line.split(',') {
            order.push(point.parse()?);
        }

        let mut cards = vec![];
        let lines = lines.collect::<Vec<&str>>().join("\n");
        let buffers = lines.split("\n\n");

        for card in buffers {
            let bingo_card = BingoCard::parse(card, width, height)?;
            cards.push(bingo_card);
        }

        Ok(BingoGame { order, cards })
    }

    pub fn play(&self) -> Result<Vec<BingoResult>, Box<dyn Error>> {
        let mut results = vec![];
        let mut used_cards = vec![];

        for index in 0..self.order.len() {
            let called = &self.order[..index];
            for card in &self.cards {
                if used_cards.contains(&card) {
                    continue;
                }
                if card.check(called) {
                    used_cards.push(card);
                    let result = BingoResult { called, card };
                    results.push(result);
                }
            }
        }

        Ok(results)
    }
}

impl BingoCard {
    pub fn parse(value: &str, width: usize, height: usize) -> Result<BingoCard, Box<dyn Error>> {
        let mut values = vec![];
        for line in value.lines() {
            for point in line.split(' ') {
                if !point.is_empty() {
                    values.push(point.parse()?);
                }
            }
        }

        Ok(BingoCard {
            values,
            width,
            height,
        })
    }

    pub fn get_uncalled(&self, values: &[Point]) -> Vec<&Point> {
        let mut uncalled = vec![];
        for point in &self.values {
            if !values.contains(point) {
                uncalled.push(point);
            }
        }
        uncalled
    }

    fn get_value(&self, x: usize, y: usize) -> Option<&Point> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = x + (self.width * y);
        Some(&self.values[index])
    }

    // This could probably done in a muchhhh nicer way
    pub fn check(&self, values: &[Point]) -> bool {
        // Columns
        for x in 0..self.width {
            let mut found = true;
            for y in 0..self.height {
                if let Some(value) = self.get_value(x, y) {
                    if !values.contains(value) {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                return true;
            }
        }

        // Rows
        for y in 0..self.height {
            let mut found = true;
            for x in 0..self.width {
                if let Some(value) = self.get_value(x, y) {
                    if !values.contains(value) {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                return true;
            }
        }

        false
    }
}

impl<'a> BingoResult<'a> {
    pub fn get_result(&self) -> Result<u32, Box<dyn Error>> {
        let last_called = self.get_last_called().ok_or("No last called")?;
        let uncalled_sum = self.get_uncalled_sum();
        Ok(last_called * uncalled_sum)
    }

    pub fn get_last_called(&self) -> Option<&Point> {
        self.called.last()
    }

    pub fn get_uncalled_sum(&self) -> u32 {
        self.card.get_uncalled(self.called).into_iter().sum()
    }
}

#[cfg(test)]
mod test_bingo_game {
    use super::*;

    #[test]
    fn test_example() -> Result<(), Box<dyn Error>> {
        let example = [
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .join("\n");

        let game = BingoGame::parse(&example, 5, 5)?;

        let expected_order = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        assert_eq!(expected_order, game.order);

        let results = game.play()?;
        let first_result = results.first().ok_or("Missing first result")?;

        assert_eq!(first_result.get_last_called(), Some(&24));
        assert_eq!(first_result.get_uncalled_sum(), 188);
        assert_eq!(first_result.get_result()?, 4512);

        let last_result = results.last().ok_or("Missing last result")?;
        assert_eq!(last_result.get_last_called(), Some(&13));
        assert_eq!(last_result.get_uncalled_sum(), 148);
        assert_eq!(last_result.get_result()?, 1924);

        Ok(())
    }
}
