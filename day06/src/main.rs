use std::error::Error;
use std::io::Read;

#[derive(Debug)]
struct LanternFishGroup {
    count: u64,
    days: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;
    let buffer = buffer.strip_suffix('\n').ok_or("invalid value")?;

    let mut fish = vec![];
    for value in buffer.split(',') {
        let days = value.parse::<u32>()?;
        fish.push(LanternFishGroup { count: 1, days });
    }

    run_days(&mut fish, 80);
    println!("Part 1: {}", get_fish_count(&fish));

    // we already ran 80 days, don't run that again
    run_days(&mut fish, 256 - 80);
    println!("Part 2: {}", get_fish_count(&fish));

    Ok(())
}

fn get_fish_count(fish: &[LanternFishGroup]) -> u64 {
    let mut count = 0;
    for group in fish {
        count += group.count;
    }
    count
}

fn run_days(fish: &mut Vec<LanternFishGroup>, days: u32) {
    for _ in 0..days {
        run_day(fish);
    }
}

fn run_day(fish: &mut Vec<LanternFishGroup>) {
    let mut newborn = 0;
    for group in fish.iter_mut() {
        if group.days == 0 {
            group.days = 6;
            newborn += group.count;
        } else {
            group.days -= 1;
        }
    }
    if newborn > 0 {
        fish.push(LanternFishGroup {
            count: newborn,
            days: 8,
        });
    }
}
