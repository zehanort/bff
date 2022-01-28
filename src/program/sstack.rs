use super::fungetypes::FungeInteger;

#[derive(Default)]
pub(super) struct SStack<T> {
    stacks: Vec<Vec<T>>,
}

impl<T: FungeInteger> SStack<T> {
    /**
    Returns the top of the stack stack (TOSS)
    i.e., the "active" stack.
    */
    fn get_toss(&mut self) -> &mut Vec<T> {
        if self.stacks.is_empty() {
            self.stacks.push(vec![]);
        }
        self.stacks.last_mut().unwrap()
    }

    /// Clears the TOSS, leaving it empty.
    pub fn clear_toss(&mut self) {
        self.get_toss().clear();
    }

    /// Pushes `x` onto the TOSS.
    pub fn push_onto_toss(&mut self, x: T) {
        self.get_toss().push(x);
    }

    /**
    Pops and returns a value from the TOSS,
    or 0 if it is empty.
    */
    pub fn pop_from_toss(&mut self) -> T {
        match self.get_toss().pop() {
            Some(x) => x,
            None => T::zero(),
        }
    }
}
