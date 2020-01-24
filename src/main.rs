#[macro_use]
extern crate clap;
extern crate rand;

use clap::App;
use rand::Rng;

fn main() {
  let yaml = load_yaml!("../cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let die = matches.value_of("die").unwrap_or("20").parse::<i32>().unwrap();
  let num = matches.value_of("num").unwrap_or("1").parse::<i32>().unwrap();

  for _ in 0..num {
    println!("You rolled {}! [0 - {}]", roll(die), die);
  }
}

fn roll(die: i32) -> i32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(1, die)
}