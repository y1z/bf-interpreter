use crate::program_data;
use std::io::prelude::*;

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

pub fn interpret(bf_code: String, show_cells_after_ops: bool) {
  let mut program = program_data::ProgramData::new();
  let max_index = program.memory_cells.len() - 1;

  let mut code_index = 0usize;
  let bf_code_length = bf_code.len();

  let mut path_taken = String::with_capacity(40_000usize);

  let bf_code_final: Vec<(usize, char)> = bf_code.char_indices().collect();
  loop {
    let keep_parsing = code_index < bf_code_length;
    if !keep_parsing {
      break;
    }
    let mut dont_advance_past_left_brace = false;

    for command in COMMANDS.iter() {
      let index_character = bf_code_final[code_index];
      let are_the_same = None != command.0.matches(index_character.1).next();

      if are_the_same {
        let std_in = std::io::stdin();
        let std_out = std::io::stdout();
        match command.1 {
          BF_OPS::MOVE_POINTER_LEFT => {
            program.pointer = (program.pointer.wrapping_sub(1)) % max_index;
            break;
          }
          BF_OPS::MOVE_POINTER_RIGHT => {
            program.pointer = (program.pointer.wrapping_add(1)) % max_index;
            break;
          }
          BF_OPS::DECREMENT_CELL_VAL => {
            let mut temp = program.memory_cells[program.pointer];
            temp = temp.wrapping_sub(1);
            program.memory_cells[program.pointer] = temp;
            break;
          }
          BF_OPS::INCREMENT_CELL_VAL => {
            let mut temp = program.memory_cells[program.pointer];
            temp = temp.wrapping_add(1);
            program.memory_cells[program.pointer] = temp;
            break;
          }

          BF_OPS::PRINT_CHAR => {
            let mut a = [0];
            a[0] = program.memory_cells[program.pointer];
            let mut handle = std_out.lock();
            let res = handle.write(&a);
            if let Err(error) = res {
              panic!("\n\n\nError : [{}]", error);
            }
            break;
          }
          BF_OPS::INPUT_CHAR => {
            let mut final_input = String::new();
            let mut handle = std_in.lock();
            let res = handle.read_line(&mut final_input);
            if let Err(error) = res {
              panic!("\n\n\nError : [{}]", error);
            }

            program.memory_cells[program.pointer] = final_input.as_bytes()[0];
            break;
          }
          BF_OPS::JUMP_PAST_RIGHT_BRACE => {
            if 0 == program.memory_cells[program.pointer] {
              let new_index = find_matching_brace(&bf_code_final, code_index);
              if let Some(new_index_value) = new_index {
                code_index = new_index_value;
              }
            }
            break;
          }
          BF_OPS::JUMP_BACK_TO_LEFT_BRACE => {
            if 0 != program.memory_cells[program.pointer] {
              let new_index = find_matching_brace(&bf_code_final, code_index);
              if let Some(new_index_value) = new_index {
                code_index = new_index_value;
                dont_advance_past_left_brace = true;
              }
            }
            break;
          }
        }
      }
    }

    if !dont_advance_past_left_brace {
      //path_taken.push(bf_code_final[code_index].1);
      //eprint!("{}", );
      code_index += 1;
    }
  }

  //for (index, character) in bf_code.char_indices() {}
  println!("\n\n");
  //eprint!("\n path taken by program \n{}\n", path_taken);

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

fn find_matching_brace(code: &Vec<(usize, char)>, current_index: usize) -> Option<usize> {
  let matching_brace = code[current_index].1;
  let advance_forward = if '[' == matching_brace { true } else { false };

  let oppsite_brace = if advance_forward { ']' } else { '[' };

  let mut level = 1;
  let mut index = current_index;

  if advance_forward {
    index = index + 1;
  } else {
    index = index - 1;
  }

  let mut keep_parsing = true;
  while keep_parsing {
    let cant_go_back_further = index == 0 && !advance_forward;

    if cant_go_back_further {
      return None;
    }

    let current_char = code[index].1;
    //eprint!("{}", current_char);

    if oppsite_brace == current_char {
      level = level - 1;
    } else if matching_brace == current_char {
      level = level + 1;
    }

    if level > 0 {
      if advance_forward {
        index = index + 1;
      } else {
        index = index - 1;
      }
    } else {
      keep_parsing = false;
    }
  }

  Some(index)
}
