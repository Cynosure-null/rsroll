use rand::Rng;
use std::env;
use std::str::FromStr;

pub fn roll(count: i32, die_type: i32) -> i32{
    let mut _tally = 0;
    let mut roll_result;
    let mut roll_sum: i32 = 0;

    for mut _tally in 0..count {
        roll_result = rand::thread_rng().gen_range(1..=die_type);
        roll_sum += roll_result;
        _tally += 1;
    }
    return roll_sum;
}

pub fn advantage(count: i32, die_type:i32) -> i32 {
    let roll1 = roll(count, die_type);
    let roll2 = roll(count, die_type);

    if roll1 > roll2 {
        return roll1;
    }
    else if roll1 < roll2 {
        return roll2;
    }
    else {
        return roll1;
    }
}

pub fn disadvantage(count: i32, die_type:i32) -> i32 {
    let roll1 = roll(count, die_type);
    let roll2 = roll(count, die_type);

    if roll1 < roll2 {
        return roll1;
    }
    else if roll1 > roll2 {
        return roll2;
    }
    else {
        return roll1;
    }
}
