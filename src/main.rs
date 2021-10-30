use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
const STARTING_CELLS: usize = 30_000usize;

#[allow(non_camel_case_types)]
pub enum BF_OPS {
  MOVE_POINTER_RIGHT,
  MOVE_POINTER_LEFT,
  INCREMENT_CELL_VAL,
  DECREMENT_CELL_VAL,
  PRINT_CHAR,
  INPUT_CHAR,
  JUMP_PAST_RIGHT_BRACE,
  JUMP_BACK_TO_LEFT_BRACE,
}

const COMMANDS: [(&'static str, BF_OPS); 8] = [
  (">", BF_OPS::MOVE_POINTER_RIGHT),
  ("<", BF_OPS::MOVE_POINTER_LEFT),
  ("+", BF_OPS::INCREMENT_CELL_VAL),
  ("-", BF_OPS::DECREMENT_CELL_VAL),
  (".", BF_OPS::PRINT_CHAR),
  (",", BF_OPS::INPUT_CHAR),
  ("[", BF_OPS::JUMP_PAST_RIGHT_BRACE),
  ("]", BF_OPS::JUMP_BACK_TO_LEFT_BRACE),
];

fn main() {
  let args = env::args();
  println!("\n\tSTART INTERPRETER\n");
  for argument in args {
    let input_string = load_file(&argument);
    if let Some(string) = input_string {
      println!("{}", string);
      interpret(string, true);
    }
  }
}

pub fn load_file(file_name: &String) -> Option<String> {
  if file_name.ends_with(".bf") || file_name.ends_with(".txt") {
    let file = File::open(&file_name);

    if let Ok(opened_file) = file {
      let mut reader = BufReader::new(opened_file);
      let mut result = String::new();

      if let Ok(_) = reader.read_to_string(&mut result) {
        return Some(result);
      }
      eprint!("error : [{}] is not valid", file_name);
    }
  }

  None
}

pub fn interpret(bf_code: String, show_cells_after_ops: bool) {
  let mut memory_cells: Vec<u8> = vec![0; STARTING_CELLS];
  let mut pointer = 0usize;
  for (character, index) in bf_code.char_indices() {}

  if show_cells_after_ops {
    let mut last_value: u8 = memory_cells.first().unwrap_or(&0).clone();
    let mut repeated_value_count = 0usize;

    for cell in memory_cells {
      if last_value == cell {
        repeated_value_count += 1;
      } else {
        display_bf_cell_data(repeated_value_count, last_value);
      }

      last_value = cell;
    }

    if repeated_value_count != 0 {
      display_bf_cell_data(repeated_value_count, last_value);
    }
  }
  println!("\n===========end of program================\n");
}

fn display_bf_cell_data(repeated_value_count: usize, last_value: u8) {
  match repeated_value_count {
    1 => print!("| {}", last_value),
    _ => println!(
      "\nrepeated [x{}] value {} ",
      repeated_value_count, last_value
    ),
  }
}
