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

fn roll(die: i32) -> i32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(1, die)
}