use crate::file_reader;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
struct Game {
    id: u32,
    possible: bool,
    red: u32,
    green: u32,
    blue: u32,
}

#[allow(dead_code)]
impl Game {
    fn new(id: u32) -> Game {
        Game {
            id,
            possible: true,
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn update_color(&mut self, color: &str, amount: u32) {
        let color = match color {
            "red" => &mut self.red,
            "green" => &mut self.green,
            "blue" => &mut self.blue,
            _ => panic!("Invalid color"),
        };

        if amount > *color {
            return *color = amount;
        }
    }

    fn check_possible(&mut self) {
        if self.red > MAX_RED || self.green > MAX_GREEN || self.blue > MAX_BLUE {
            self.possible = false;
        }
    }

    fn play_round(&mut self, color: &str, amount: u32) {
        self.update_color(color, amount);
        self.check_possible();
    }

    fn print_round(&self) {
        println!(
            "{} Game {}:\n {} red,\n {} green,\n {} blue\n",
            if self.possible { "✅" } else { "❎" },
            self.id,
            self.red,
            self.green,
            self.blue
        )
    }

    fn get_power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub fn solve() {
    let file_path = "./day2.txt";
    let data = file_reader::lines_to_vec(file_path).unwrap();
    let mut games: Vec<Game> = vec![];

    data.into_iter().enumerate().for_each(|(i, l)| {
        let mut game = Game::new((i as u32) + 1);

        let line: Vec<&str> = l.split(":").collect::<Vec<&str>>();
        let str_rounds: Vec<&str> = line[1].split(";").collect::<Vec<&str>>();

        let array_rounds: Vec<Vec<&str>> = str_rounds
            .into_iter()
            .map(|round| {
                round
                    .trim()
                    .split(",")
                    .map(str::trim)
                    .collect::<Vec<&str>>()
            })
            .collect::<Vec<Vec<&str>>>();
        let rounds: Vec<Vec<Vec<&str>>> = array_rounds
            .into_iter()
            .map(|round| {
                round
                    .iter()
                    .map(|cubes_shown| cubes_shown.split(" ").collect::<Vec<&str>>())
                    .collect::<Vec<Vec<&str>>>()
            })
            .collect::<Vec<Vec<Vec<&str>>>>();
        println!("\tRounds: {:?}", rounds);

        for cubes_shown in rounds {
            println!("\t\tcubes_shown: {:?}", cubes_shown);
            for cubes in cubes_shown {
                println!("\t\t\tcubes: {:?}", cubes);
                let color = cubes[1];
                let amount = cubes[0].parse::<u32>().unwrap_or(0);

                println!("\t\t\t\tcolor: {}, amount: {}", color, amount);
                game.play_round(color, amount);
            }
        }

        game.check_possible();
        game.print_round();
        games.push(game);
    });

    println!("\n\nGAMES:\n {:?}", games);

    let possible_games: Vec<u32> = games
        .iter()
        .filter_map(|game| {
            if game.possible {
                println!("Game {} is possible", game.id);
                return Some(game.id);
            } else {
                println!("Game {} is not possible", game.id);
                return None;
            }
        })
        .collect::<Vec<u32>>();
    println!("\nPossible games: {:?}", possible_games);

    // 2551
    let sum_possible_ids: u32 = possible_games.iter().sum();
    println!("Sum of possible ids: {}", sum_possible_ids);

    let powers: Vec<u32> = games
        .iter()
        .map(|game| game.get_power())
        .collect::<Vec<u32>>();
    println!("\nPowers: {:?}", powers);

    let part_two_key: u32 = powers.iter().sum();
    println!("\nPart two key: {}", part_two_key);
}
