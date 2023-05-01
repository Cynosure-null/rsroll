use std::{env, path};
use std::str::FromStr;
mod calc;

pub fn prints(args: Vec<String>)
{
    let i: i8 = 0;
    println!("{0}/{1}", args.len(), args.len()-2);
    for i in 0..args.len()-2
    {
        if let Some(_arg) = args.get(i)
        {
            println!("{0}>{1}", &i, &args[0]);
        }
    }
}
pub fn legacy_main(){
    let is_debug = true;
    let args: Vec<String> = env::args().collect();
    println!("-----");

    if &args.len() < &2
        {
            println!("Must provide at least 2 arguments");
            return;
        }

    if is_debug == true
        {
            prints(args.clone());
        }

    if &args[1] == "a" || &args[1] == "A"{
        let arg1: i32 = FromStr::from_str(&args[2]).unwrap();
        let arg2: i32 = FromStr::from_str(&args[3]).unwrap();

        print!("rolling ");
        print!("{}", arg1);
        print!("d");
        print!("{}", arg2);
        println!(" with advantage...");

        println!("{}", calc::advantage(arg1, arg2));
    }

    else if &args[1] == "d" || &args[1] == "D" {
        let arg1: i32 = FromStr::from_str(&args[2]).unwrap();
        let arg2: i32 = FromStr::from_str(&args[3]).unwrap();
        print!("rolling ");
        print!("{}", arg1);
        print!("d");
        print!("{}", arg2);
        println!(" with disadvantage...");
        println!("{}", calc::disadvantage(arg1, arg2));
    }

    else {
        let arg1: i32 = FromStr::from_str(&args[1]).unwrap();
        let arg2: i32 = FromStr::from_str(&args[2]).unwrap();

        print!("rolling ");
        print!("{}", arg1);
        print!("d");
        print!("{}", arg2);
        println!(" ...");       

        println!("{}", calc::roll(arg1, arg2));
    }

}
