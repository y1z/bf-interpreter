use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
const STARTING_CELLS: usize = 30_000usize;

#[allow(non_camel_case_types)]
pub enum BF_OPS {
  move_pointer_right,
  move_pointer_left,
  increment_cell_val,
  decrement_cell_val,
  print_char,
  input_char,
  jump_past_right_brace,
  jump_back_left_brace,
}

fn main() {
  let args = env::args();
  println!("\n\t START INTERPRETER\n");
  for argument in args {
    let input_string = load_file(&argument);
    if let Some(string) = input_string {
      println!("{}", string);
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
