pub struct Interpreter<'program> {
    /// The program data.
    pub ram: [i32; 30_000],
    /// A pointer to access to the program data.
    pub ram_ptr: usize,
    /// Our input (a file, for example).
    pub program: &'program [u8],
    /// A pointer to read each character.
    pub program_ptr: usize,
    /// To resolve the loops.
    pub stack: Vec<usize>,
}
