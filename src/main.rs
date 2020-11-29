use rand::distributions::{Distribution, Uniform};

enum Result {
  Critical,
  MessyCritical,
  Success,
  Failure,
  TotalFailure,
  BestialFailure,
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  assert!(args.len() == 4, "Usage: ./v5prob <pool> <hunger> <difficulty>");

  let pool: u8 = args[1].parse().expect("Pool must be a number");
  let hunger: u8 = args[2].parse().expect("Hunger must be a number");
  let difficulty: u8 = args[3].parse().expect("Difficulty must be a number");

  let mut criticals = 0.0;
  let mut messies = 0.0;
  let mut successes = 0.0;
  let mut failures = 0.0;
  let mut total_failures = 0.0;
  let mut bestials = 0.0;

  const TRIALS: u32 = 1_000_000;

  for _ in 0..TRIALS {
    match roll(pool, hunger, difficulty) {
      Result::Critical => criticals += 1.0,
      Result::MessyCritical => messies += 1.0,
      Result::Success => successes += 1.0,
      Result::Failure => failures += 1.0,
      Result::TotalFailure => total_failures += 1.0,
      Result::BestialFailure => bestials += 1.0,
    }
  }

  let f_trials = TRIALS as f64;
  criticals /= f_trials;
  messies /= f_trials;
  successes /= f_trials;
  failures /= f_trials;
  total_failures /= f_trials;
  bestials /= f_trials;

  println!("Pool {}, hunger {}, difficulty {} stats", pool, hunger, difficulty);
  println!("Critical:        {}", criticals);
  println!("Messy Critical:  {}", messies);
  println!("Success:         {}", successes);
  println!("Failure:         {}", failures);
  println!("Total Failure:   {}", total_failures);
  println!("Bestial Failure: {}", bestials);
}

fn roll(pool: u8, hunger: u8, difficulty: u8) -> Result {
  let mut normal_tens = 0;
  let mut hunger_tens = 0;
  let mut successes = 0;
  let mut bestials = 0;

  let mut rng = rand::thread_rng();
  let die = Uniform::from(1..=10);

  for n in 0..pool {
    let throw = die.sample(&mut rng);

    if n < hunger {
      match throw {
        10 => hunger_tens += 1,
        6..=9 => successes += 1,
        1 => bestials += 1,
        _ => continue,
      }
    } else {
      match throw {
        10 => normal_tens += 1,
        6..=9 => successes += 1,
        _ => continue,
      }
    }
  }

  let mut criticals = normal_tens + hunger_tens;
  criticals /= 2;
  let total_successes = criticals * 2 + successes;

  if total_successes == 0 {
    if bestials == 0 {
      return Result::TotalFailure;
    } else {
      return Result::BestialFailure;
    }
  }

  if total_successes >= difficulty {
    if criticals > 0 {
      if hunger_tens > 0 {
        return Result::MessyCritical;
      }
      return Result::Critical;
    }
    return Result::Success;
  }
  return Result::Failure;
}

