fn main() {
    println!("{}", include_str!("./ex1.txt")
        .lines()
        .filter_map(|s| {
            let s = s
                .replace("eight", "e8t")
                .replace("seven", "s7n")
                .replace("three", "t3e")
                .replace("nine", "n9e")
                .replace("five", "f5e")
                .replace("four", "f4r")
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("six", "s6x");
            let chars = || s.chars().filter(char::is_ascii_digit);
            let first = chars().next()?;
            let last = chars().last()?;
            println!("{first}{last}");
            Some(format!("{first}{last}")
                .parse::<usize>()
                .expect("should be a valid number"))
        })
        .sum::<usize>());
}
