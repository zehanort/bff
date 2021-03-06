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

    /// Returns the SOSS, if it exists.
    fn get_soss(&mut self) -> Option<&mut Vec<T>> {
        if self.stacks.len() < 2 {
            None
        } else {
            let n_stacks = self.stacks.len();
            Some(&mut self.stacks[n_stacks - 2])
        }
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
        self.get_toss().pop().unwrap_or_default()
    }

    /// Creates a new stack (i.e., a new TOSS).
    pub fn create_stack(&mut self, n: T, so: (T, T)) {
        let soss = self.get_toss();
        let soss_len = soss.len();
        let mut toss = vec![];
        match n.cmp(&T::zero()) {
            std::cmp::Ordering::Less => {
                soss.append(&mut vec![T::zero(); n.abs().to_usize().unwrap()])
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                let n_u = n.to_usize().unwrap();
                if n_u > soss_len {
                    toss.append(soss);
                } else {
                    toss = soss.split_off(soss_len - n_u);
                }
            }
        }
        // step 2: push storage offset as a vector to the old TOSS, now SOSS
        soss.push(so.0);
        soss.push(so.1);
        // step 3:
        self.stacks.push(toss);
    }

    pub fn destroy_stack(&mut self, n: T) -> Option<(T, T)> {
        if self.stacks.len() < 2 {
            None
        } else {
            let mut toss = self.stacks.pop()?;
            let toss_len = toss.len();
            let soss = self.stacks.last_mut()?;
            // step 1: pop (previous) storage offset
            let (y, x) = (
                soss.pop().unwrap_or_default(),
                soss.pop().unwrap_or_default(),
            );
            let soss_len = soss.len();

            // step 2: move n elements from TOSS to SOSS
            match n.cmp(&T::zero()) {
                std::cmp::Ordering::Less => {
                    soss.truncate(soss_len - n.abs().to_usize().unwrap());
                }
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    let n_u = n.to_usize().unwrap();
                    if n_u > toss_len {
                        soss.append(&mut [vec![T::zero(); n_u - toss_len], toss.to_vec()].concat());
                    } else {
                        soss.append(&mut toss.split_off(toss_len - n_u));
                    }
                }
            };
            // step 3: return the previous (and now current) storage offset
            Some((x, y))
        }
    }

    pub fn transfer(&mut self, count: T) -> Option<()> {
        if self.stacks.len() == 1 {
            None
        } else {
            let from = if count > T::zero() {
                self.get_soss()?
            } else {
                self.get_toss()
            };
            let n = count.abs().to_usize().unwrap_or_default();
            if n > 0 {
                let mut popped = vec![];
                for _ in 0..n {
                    let x = from.pop().unwrap_or_default();
                    popped.push(x);
                }
                let to = if count > T::zero() {
                    self.get_toss()
                } else {
                    self.get_soss()?
                };
                to.append(&mut popped);
            }
            Some(())
        }
    }

    pub fn get_stacks(&self) -> &Vec<Vec<T>> {
        &self.stacks
    }

    /// Returns the `n`th element of the TOSS,
    /// counting from the top (the topmost element has index 0).
    pub fn get(&mut self, n: usize) -> T {
        let toss_len = self.get_toss().len();
        if n >= toss_len {
            T::zero()
        } else {
            self.get_toss()[toss_len - n - 1]
        }
    }
}
