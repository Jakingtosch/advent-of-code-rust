advent_of_code::solution!(2);
use std::str;

trait Deserialize {
    fn deserialize(data: &str) -> Self;
}

struct Game {
    id: u32,
    plays: Vec<Play>
}

struct Play {
    green: u32,
    red: u32,
    blue: u32
}

static LIMIT_RED: u32 = 12;
static LIMIT_GREEN: u32 = 13;
static LIMIT_BLUE: u32 = 14;

impl Game {

    fn get_green(&self) -> Vec<u32>{
        let greens: Vec<u32>  = self.plays.iter().map( |x| x.green.clone()).collect();
        greens
    }
    fn get_blue(&self) -> Vec<u32>{
        let blues: Vec<u32>  = self.plays.iter().map( |x| x.blue.clone()).collect();
        blues
    }
    fn get_red(&self) -> Vec<u32>{
        let reds: Vec<u32>  = self.plays.iter().map( |x| x.red.clone()).collect();
        reds
    }

    fn get_minimum_of_color(color_vec: Vec<u32>) -> u32 {
        color_vec.iter().max().unwrap().clone()
    }

    fn get_play_power(&self) -> u32 {
        Game::get_minimum_of_color(self.get_blue()) * Game::get_minimum_of_color(self.get_red()) * Game::get_minimum_of_color(self.get_green())
    }

    fn is_green_in_limit(&self) -> bool {
        return self.get_green().iter().all( |x| x.clone() <= LIMIT_GREEN);
    }

    fn is_blue_in_limit(&self) -> bool {
        return self.get_blue().iter().all( |x| x.clone()  <= LIMIT_BLUE);
    }

    fn is_red_in_limit(&self) -> bool {
        return self.get_red().iter().all(|x| x.clone() <= LIMIT_RED);
    }
    fn is_game_in_limit(&self) -> bool {
        let green_ok = self.is_green_in_limit();
        let red_ok = self.is_red_in_limit();
        let blue_ok = self.is_blue_in_limit();
        green_ok & blue_ok & red_ok
    }
}
impl Deserialize for Play {
    fn deserialize(data: &str) -> Self {
        let parts: Vec<&str> = data.split(",").collect();
        let mut tmp_green = 0;
        let mut tmp_red = 0;
        let mut tmp_blue = 0;

        for unparsed_str in parts {
            let colors: Vec<&str> = unparsed_str.trim().split(" ").collect();

            match colors.last().unwrap().as_ref() {
                "blue" => { tmp_blue = colors.first().unwrap().parse::<u32>().unwrap() },
                "green" => { tmp_green = colors.first().unwrap().parse::<u32>().unwrap() },
                "red" => { tmp_red = colors.first().unwrap().parse::<u32>().unwrap()},

                _ => panic!("should not be here!")
            };
        }

        Play{
            blue: tmp_blue,
            green: tmp_green,
            red: tmp_red
        }
    }
}

impl Deserialize for Game {
    fn deserialize(data: &str) -> Self {
        let parts: Vec<&str> = data.split(":").collect();
        let game_data: Vec<&str> = parts.first().unwrap().split(" ").collect();
        let game_id = game_data.last().unwrap().parse::<u32>().unwrap();
        let play_data_string: Vec<&str> = parts.last().unwrap().split(";").collect();
        let play_data: Vec<Play> = play_data_string.iter().map(|x| Play::deserialize(x)).collect();
        Game{
            id: game_id,
            plays: play_data
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let games: Vec<Game> = input.lines().map(|leine| Game::deserialize(leine)).collect();
    let mut valid_games: Vec<u32> = Vec::new();

    for iter_game in games {

        if iter_game.is_game_in_limit() {
            valid_games.push(iter_game.id);
            println!("{}", iter_game.id);
        }
    }
    let summed_value: u32 = valid_games.into_iter().sum();
    Some(summed_value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().map(|line| Game::deserialize(line)).collect();

    let game_powers: Vec<u32> = games.iter().map(|this_game| this_game.get_play_power()).collect();
    Some(game_powers.into_iter().sum())
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
