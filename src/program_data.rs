const DEFAULT_STARTING_CELLS: usize = 30_000usize;
/// Contains all the data inside the program.
pub struct ProgramData {
  pub memory_cells: Vec<u8>,
  pub pointer: usize,
  pub code_index: usize,
  pub open_and_close_bracket_range: (usize, usize),
}

impl ProgramData {
  pub fn new() -> ProgramData {
    ProgramData {
      memory_cells: vec![0; DEFAULT_STARTING_CELLS],
      pointer: 0,
      code_index: 0,
      open_and_close_bracket_range: (0, 0),
    }
  }
}
