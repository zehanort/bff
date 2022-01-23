use super::Program;

trait InstructionSet<T> {}

impl InstructionSet<i32> for Program<i32> {}
