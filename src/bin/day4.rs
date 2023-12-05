use std::collections::HashSet;
fn part1() {
    let input = include_str!("ex4.txt");
    let result = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Card::parse_line)
        .map(|g| g.score())
        .sum::<usize>();
    println!("{result}");
}
fn part2() {
    let input = include_str!("ex4.txt");
    let mut cards = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Card::parse_line)
        .collect::<Vec<Card>>();
    dbg!(cards.len());
    for card_index in 0..cards.len() {
        for next_card_index in 0..cards[card_index].winning_numbers_count() {
            dbg!(
                card_index,
                next_card_index,
                cards[card_index].winning_numbers_count(),
                cards[card_index].instances
            );
            cards[card_index + 1 + next_card_index].instances += cards[card_index].instances;
        }
    }
    let result = cards.iter().map(|card| dbg!(card.instances)).sum::<usize>();
    println!("{result}");
}
fn main() {
    part1();
    part2();
}
#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
    instances: usize,
}
impl Card {
    fn winning_numbers_count(&self) -> usize {
        self.numbers.intersection(&self.winning_numbers).count()
    }
    fn score(&self) -> usize {
        let winning_numbers_count = self.winning_numbers_count() as u32;
        if winning_numbers_count > 0 {
            2usize.pow(winning_numbers_count - 1)
        } else {
            0
        }
    }
    fn parse_line(str: &str) -> Card {
        let mut colon_split = str.split(':');
        let id = colon_split.next().unwrap()[4..]
            .trim()
            .parse::<usize>()
            .unwrap();
        let (numbers, winning_numbers) = Card::parse_card_numbers(colon_split.next().unwrap());
        Card {
            numbers,
            winning_numbers,
            id,
            instances: 1,
        }
    }
    fn parse_card_numbers(str: &str) -> (HashSet<usize>, HashSet<usize>) {
        let mut bar_split = str.split('|');
        let winning_numbers_str = bar_split.next().unwrap();
        let numbers_str = bar_split.next().unwrap();
        (
            numbers_str
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect(),
            winning_numbers_str
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect(),
        )
    }
}
