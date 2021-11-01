mod bf_interpret;
mod program_data;

use bf_interpret::interpret;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
