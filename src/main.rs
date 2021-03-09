use std::io::{stdin, stdout, Write};

fn main() {
  //Starting
  println!("Team Fortress Price Calculator");
  println!("Source code: https://gitlab.com/cuqerr/tfpc\n");
  println!("Type 'help' to learn how to use TFPC!\n");

  //Main loop
  loop {
    //Command prompt
    print!(">>> ");
    match stdout().flush() {
      Ok(_) => {}
      Err(e) => {
        println!("Error flushing stdout(): {}", e);
        get_input();
        std::process::exit(1);
      }
    }

    //User input
    let input0 = get_input();

    //Matching user input
    let input = input0.as_str(); //Converted to str to not use "help".to_string(); etc.
    let mut unknown: bool = false;
    match input.to_lowercase().trim() {
      "help" => {
        println!();
        println!("Welcome to the TFPC, it is really easy to use.");
        println!("You can type your currency and it will convert it to all available currencies.");
        println!("Example:");
        println!(">>> 1 scr + 1 rec");
        println!("    0.44 REF");
        println!("    1 REC + 1 SCR");
        println!("    4 SCR");
        println!();
        println!("Scrap Metal = SCR  |  Reclaimed Metal = REC  |  Refined Metal = REF");
        println!();
      }
      _ => {
        //Parsing user input
        let mut nums: Vec<f64> = Vec::new();
        let mut types: Vec<Metal> = Vec::new();
        let mut ops: Vec<Operation> = Vec::new();

        ops.push(Operation::Add); //Adding first value as positive.

        let mut words: Vec<&str> = Vec::new();
        for i in input.split_whitespace() {
          words.push(i);
        }
        if words[0] == "-" {
          continue;
        }
        for i in 0..words.len() {
          match words[i].parse::<f64>() {
            Ok(o) => {
              nums.push(o);
            }
            Err(e) => match words[i] {
              "ref" => types.push(Metal::Ref),
              "rec" => types.push(Metal::Rec),
              "scr" => types.push(Metal::Scr),
              "+" => ops.push(Operation::Add),
              "-" => ops.push(Operation::Sub),
              _ => {
                println!("Unknown word: {}", words[i]);
                unknown = true;
                break;
              }
            },
          }
        }

        if unknown {
          continue;
        }

        //calculating
        let mut refined: i64 = 0;
        let mut reclaimed: i64 = 0;
        let mut scrap: i64 = 0;
        loop {
          if !&ops.is_empty() && !&nums.is_empty() && !&types.is_empty() {
            let op = &ops[0];
            let type_ = &types[0];
            let mut num = nums[0];
            match type_ {
              Metal::Scr => {
                if op == &Operation::Add {
                  scrap += num as i64;
                } else {
                  scrap -= num as i64;
                }
              }
              Metal::Rec => {
                if op == &Operation::Add {
                  scrap += (num * 3.0).round() as i64;
                } else {
                  scrap -= (num * 3.0).round() as i64;
                }
              }
              Metal::Ref => {
                while num >= 1.0 {
                  if op == &Operation::Add {
                    scrap += 9;
                  } else {
                    scrap -= 9;
                  }
                  num -= 1.0;
                }
                if num >= 0.11 {
                  if op == &Operation::Add {
                    scrap += (num / 0.11).round() as i64;
                  } else {
                    scrap -= (num / 0.11).round() as i64;
                  }
                }
              }
            }
            &ops.remove(0);
            &types.remove(0);
            &nums.remove(0);
            if ops.is_empty() && types.is_empty() && nums.is_empty() {
              println!("{} SCR", scrap);
              while scrap >= 3 {
                scrap -= 3;
                reclaimed += 1;
              }
              println!("{} REC + {} SCR", reclaimed, scrap);
              while reclaimed >= 3 {
                reclaimed -= 3;
                refined += 1;
              }
              println!("{} REF + {} REC + {} SCR", refined, reclaimed, scrap);
              println!("------------------------");
              println!(
                "NET: {:.2} REF",
                (refined as f64 + (reclaimed as f64 * 0.33) + (scrap as f64 * 0.11))
              );
              break;
            }
          }
        }
      }
    }
  }
}

#[derive(PartialEq)]
enum Metal {
  Ref,
  Rec,
  Scr,
}
#[derive(PartialEq)]
enum Operation {
  Add,
  Sub,
}
fn get_input() -> String {
  let mut input: String = String::new();
  match stdin().read_line(&mut input) {
    Ok(_) => input,
    Err(e) => {
      println!("Can't get user input: {}", e);
      get_input();
      std::process::exit(1);
    }
  }
}
