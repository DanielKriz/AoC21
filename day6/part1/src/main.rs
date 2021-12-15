use std::io::stdin;

const MAX_DAYS: u32 = 80;
const NEW_SPAN: u32 = 8;
const OLD_SPAN: u32 = 6;

fn simulate_fish(mut day: u32, mut days_to_breed: u32) -> u32 {
    if day == MAX_DAYS {
        return 1;
    } else if days_to_breed == 0 {
        day += 1;
        return simulate_fish(day, OLD_SPAN) + simulate_fish(day, NEW_SPAN);
    } else {
        day += 1;
        days_to_breed -= 1;
        return simulate_fish(day, days_to_breed);
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let fish_herd = buf
        .split(",")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap());

    let mut total_fish: u32 = 0;
    for fish in fish_herd {
        total_fish += simulate_fish(0, fish);
    }
    println!("{}", total_fish);
}
