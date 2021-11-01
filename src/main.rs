mod program_data;

use crate::program_data::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
  let mut program = program_data::ProgramData::new();
  let max_index = program.memory_cells.len() - 1;

  let mut code_index = 0usize;
  let bf_code_length = bf_code.len();

  let bf_code_final: Vec<(usize, char)> = bf_code.char_indices().collect();
  loop {
    let keep_parsing = code_index < bf_code_length;
    if !keep_parsing {
      break;
    }
    for command in COMMANDS.iter() {
      let index_character = bf_code_final[code_index];
      let are_the_same = None != command.0.matches(index_character.1).next();

      if are_the_same {
        let std_in = std::io::stdin();
        let std_out = std::io::stdout();
        match command.1 {
          BF_OPS::MOVE_POINTER_LEFT => {
            program.pointer = (program.pointer.wrapping_sub(1)) % max_index
          }
          BF_OPS::MOVE_POINTER_RIGHT => {
            program.pointer = (program.pointer.wrapping_add(1)) % max_index
          }
          BF_OPS::DECREMENT_CELL_VAL => {
            let mut temp = program.memory_cells[program.pointer];
            temp = temp.wrapping_sub(1);
            program.memory_cells[program.pointer] = temp;
          }
          BF_OPS::INCREMENT_CELL_VAL => {
            let mut temp = program.memory_cells[program.pointer];
            temp = temp.wrapping_add(1);
            program.memory_cells[program.pointer] = temp;
          }

          BF_OPS::PRINT_CHAR => {
            let mut a = [0];
            a[0] = program.memory_cells[program.pointer];
            let mut handle = std_out.lock();
            let res = handle.write(&a);
            if let Err(error) = res {
              panic!("\n\n\nError : [{}]", error);
            }
          }
          BF_OPS::INPUT_CHAR => {
            let mut final_input = String::new();
            let mut handle = std_in.lock();
            let res = handle.read_line(&mut final_input);
            if let Err(error) = res {
              panic!("\n\n\nError : [{}]", error);
            }

            program.memory_cells[program.pointer] = final_input.as_bytes()[0];
          }
          BF_OPS::JUMP_PAST_RIGHT_BRACE => {}
          BF_OPS::JUMP_BACK_TO_LEFT_BRACE => {}
        }
      }
    }

    code_index += 1;
  }

  //for (index, character) in bf_code.char_indices() {}
  println!("\n");

  if show_cells_after_ops {
    let mut last_value: u8 = program.memory_cells.first().unwrap_or(&0).clone();
    let mut repeated_value_count = 0usize;

    for cell in program.memory_cells {
      if last_value == cell {
        repeated_value_count += 1;
      } else {
        display_bf_cell_data(repeated_value_count, last_value);
        repeated_value_count = 0;
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
    0 | 1 => print!("| {} ", last_value),
    _ => println!(
      "\n value repeated [{} times] value = {} ",
      repeated_value_count, last_value
    ),
  }
}
