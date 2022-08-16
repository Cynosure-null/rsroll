use rand::Rng;
use std::env;
use std::str::FromStr;

fn roll(count: i32, die_type: i32) -> i32{
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

fn advantage(count: i32, die_type:i32) -> i32 {
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

fn disadvantage(count: i32, die_type:i32) -> i32 {
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
fn main(){
    //TODO: Get args
    //TODO: Process
    let args: Vec<String> = env::args().collect();
    println!("-----");
    println!("0>{}", &args[0]);
    println!("1>{}", &args[1]);
    println!("2>{}", &args[2]);
//    println!("3>{}", &args[3]);

    if &args[1] == "a" || &args[1] == "A"{
        let arg1: i32 = FromStr::from_str(&args[2]).unwrap();
        let arg2: i32 = FromStr::from_str(&args[3]).unwrap();
        println!("{}", advantage(arg1,arg2));
    }

    else if &args[1] == "d" || &args[1] == "D" {
        let arg1: i32 = FromStr::from_str(&args[2]).unwrap();
        let arg2: i32 = FromStr::from_str(&args[3]).unwrap();
        println!("{}", disadvantage(arg1,arg2));
    }

    else {
        let arg1: i32 = FromStr::from_str(&args[1]).unwrap();
        let arg2: i32 = FromStr::from_str(&args[2]).unwrap();
    }

}
