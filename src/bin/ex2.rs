use std::str::FromStr;
#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}
impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}
#[derive(Copy, Clone)]
struct Cubes {
    color: Color,
    cubes: usize,
}

impl FromStr for Cubes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_iter = s.split(' ');
        let cubes = split_iter
            .next()
            .ok_or(())?
            .parse::<usize>()
            .map_err(drop)?;
        let color = split_iter.next().ok_or(())?.parse::<Color>()?;
        Ok(Self { color, cubes })
    }
}
impl Cubes {
    #[cfg(not(feature = "part2"))]
    fn is_possible_with(&self, red_cubes: usize, green_cubes: usize, blue_cubes: usize) -> bool {
        match self.color {
            Color::Blue => self.cubes <= blue_cubes,
            Color::Green => self.cubes <= green_cubes,
            Color::Red => self.cubes <= red_cubes,
        }
    }
}
struct Round {
    cubes: Vec<Cubes>,
}
impl FromStr for Round {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cubes: s.split(", ").filter_map(|s| s.parse().ok()).collect(),
        })
    }
}
impl Round {
    #[cfg(not(feature = "part2"))]
    fn is_possible_with(&self, red_cubes: usize, green_cubes: usize, blue_cubes: usize) -> bool {
        self.cubes
            .iter()
            .all(|c| c.is_possible_with(red_cubes, green_cubes, blue_cubes))
    }
    #[cfg(feature = "part2")]
    fn cubes_with_color(&self, color: Color) -> Cubes {
        self.cubes
            .iter()
            .copied()
            .find(|c| c.color == color)
            .unwrap_or(Cubes { color, cubes: 0 })
    }
}
struct Game {
    #[cfg(not(feature = "part2"))]
    id: usize,
    rounds: Vec<Round>,
}
impl Game {
    #[cfg(not(feature = "part2"))]
    fn is_possible_with(&self, red_cubes: usize, green_cubes: usize, blue_cubes: usize) -> bool {
        self.rounds
            .iter()
            .all(|r| r.is_possible_with(red_cubes, green_cubes, blue_cubes))
    }
    #[cfg(feature = "part2")]
    fn minimum_cubes_required_to_be_possible(&self) -> (usize, usize, usize) {
        self.rounds.iter().fold((0, 0, 0), |(red, green, blue), r| {
            (
                r.cubes_with_color(Color::Red).cubes.max(red),
                r.cubes_with_color(Color::Green).cubes.max(green),
                r.cubes_with_color(Color::Blue).cubes.max(blue),
            )
        })
    }
}
impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colon_split = s.split(": ");
        
        #[cfg(not(feature = "part2"))]
        let id = colon_split
            .next()
            .ok_or(())?
            .trim_start_matches("Game ")
            .parse::<usize>()
            .map_err(drop)?;

        #[cfg(not(feature = "part2"))]
        let game_rounds_string = colon_split.next().ok_or(())?;

        #[cfg(feature = "part2")]
        let game_rounds_string = colon_split.nth(1).ok_or(())?;
        Ok(Game {
            #[cfg(not(feature = "part2"))]
            id,
            rounds: game_rounds_string
                .split("; ")
                .filter_map(|s| s.parse::<Round>().ok())
                .collect(),
        })
    }
}
#[cfg(not(feature = "part2"))]
fn main() {
    let result = include_str!("./ex2.txt")
        .lines()
        .filter_map(|l| l.parse::<Game>().ok())
        .filter_map(|g| g.is_possible_with(12, 13, 14).then_some(g.id))
        .sum::<usize>();
    println!("Result: {result}");
}
#[cfg(feature = "part2")]
fn main() {
    let result = include_str!("./ex2.txt")
        .lines()
        .filter_map(|l| l.parse::<Game>().ok())
        .map(|g| {
            let min_cubes = g.minimum_cubes_required_to_be_possible();
            min_cubes.0 * min_cubes.1 * min_cubes.2
        })
        .sum::<usize>();
    println!("Result: {result}");
}
