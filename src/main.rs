#[macro_use]
extern crate clap;
extern crate rand;

use clap::App;
use rand::Rng;

fn main() {
  let yaml = load_yaml!("../cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let die_opt = matches.value_of("die").unwrap_or("20").parse::<i32>().unwrap();
  let num_opt = matches.value_of("num").unwrap_or("1").parse::<i32>().unwrap();
  let sum_opt = matches.is_present("sum");
  let advangage_opt = matches.is_present("advantage");
  let disadvantage_opt = matches.is_present("disadvantage");

  if advangage_opt {
    let (result, dropped) = roll_with_advantage(die_opt);
    println!("You rolled {}! (dropped {}) [0 - {}]", result, dropped, die_opt);
  }
  else if disadvantage_opt {
    let (result, dropped) = roll_with_disadvantage(die_opt);
    println!("You rolled {}! (dropped {}) [0 - {}]", result, dropped, die_opt);
  }
  else {
    let mut rolls: Vec<i32> = Vec::new();
    for _ in 0..num_opt {
      let result = roll(die_opt);
      rolls.push(result);
      println!("You rolled {}! [0 - {}]", result, die_opt);
    }
    if sum_opt {
      println!("---");
      println!("Total: {}", rolls.iter().sum::<i32>());
    }
  }
}

fn roll(die: i32) -> i32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(1, die)
}

fn roll_with_advantage(die: i32) -> (i32, i32) {
  let mut rng = rand::thread_rng();
  let roll1 = rng.gen_range(1, die);
  let roll2 = rng.gen_range(1, die);
  return if roll1 > roll2 { (roll1, roll2) } else { (roll2, roll1) }
}

fn roll_with_disadvantage(die: i32) -> (i32, i32) {
  let mut rng = rand::thread_rng();
  let roll1 = rng.gen_range(1, die);
  let roll2 = rng.gen_range(1, die);
  return if roll1 < roll2 { (roll1, roll2) } else { (roll2, roll1) }
}