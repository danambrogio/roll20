#[macro_use]
extern crate clap;
extern crate rand;

use clap::App;
use rand::Rng;

fn main() {
  let yaml = load_yaml!("../cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let die = matches.value_of("die").unwrap_or("20");
  let mut rng = rand::thread_rng();
  let num = rng.gen_range(1, die.parse::<i32>().unwrap());
  println!("You rolled {}! [0 - {}]", num, die);
}
