/// This trait adds ergonomic convenience functions to `bool`.
pub trait PowerBool {
    /// Makes sure the value is `true`, and returns `true` if it had changed.
    #[inline(always)]
    fn raise(&mut self) -> bool {
        self.set(true)
    }

    /// Makes sure the value is `false`, and returns `true` if it had changed.
    #[inline(always)]
    fn lower(&mut self) -> bool {
        self.set(false)
    }

    /// Returns `true` if the value changes.
    #[inline(always)]
    fn set(&mut self, val: bool) -> bool;

    /// Returns `true` if the value was `false`.
    ///
    /// ```
    /// use powerbool::PowerBool;
    /// let list = vec![0.23, 3.4, 8.0, 9.6];
    /// let mut found = false;
    /// for x in list {
    ///     if found.trip((x as i32 as f32) == x) {
    ///         println!("There's an integer hiding in our number!");
    ///         break;
    ///     }
    /// }
    /// if !found {
    ///     println!("Nobody here but us floats.");
    /// }
    /// assert!(found);
    /// ```
    #[inline(always)]
    fn trip(&mut self, val: bool) -> bool;

    /// Makes sure the value is 'false', and return 'true' if it didn't change.
    /// (Am I kicking a dead horse?)
    #[inline(always)]
    fn kick(&mut self) -> bool {
        !self.set(false)
    }

    /// Makes sure the value is 'true', and return 'true' if it didn't change.
    /// (The opposite of `kick`: am I punching a sleeping horse?)
    #[inline(always)]
    fn punch(&mut self) -> bool {
        !self.set(true)
    }
}

impl PowerBool for bool {
    #[inline(always)]
    fn set(&mut self, val: bool) -> bool {
        let ret = *self != val;
        *self = val;
        ret
    }
    
    #[inline(always)]
    fn trip(&mut self, val: bool) -> bool {
        if val && !*self {
            *self = true;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::PowerBool;

    fn truth_table<F: Fn(&mut bool) -> bool>(before: bool, f: F, returns: bool, after: bool) {
        let mut v = before;
        assert_eq!(f(&mut v), returns);
        assert_eq!(v, after);
    }

    #[test]
    fn set() {
        fn set_true(v: &mut bool) -> bool { v.set(true) }
        fn set_false(v: &mut bool) -> bool { v.set(false) }
        truth_table(true, set_true, false, true);
        truth_table(true, set_false, true, false);
        truth_table(false, set_true, true, true);
        truth_table(false, set_false, false, false);
    }

    #[test]
    fn raise() {
        truth_table(true, bool::raise, false, true);
        truth_table(false, bool::raise, true, true);
    }

    #[test]
    fn lower() {
        truth_table(true, bool::lower, true, false);
        truth_table(false, bool::lower, false, false);
    }

    #[test]
    fn trip() {
        fn trip_true(v: &mut bool) -> bool { v.trip(true) }
        fn trip_false(v: &mut bool) -> bool { v.trip(false) }
        truth_table(true, trip_true, false, true);
        truth_table(true, trip_false, false, true);
        truth_table(false, trip_true, true, true);
        truth_table(false, trip_false, false, false);
    }

    #[test]
    fn kick() {
        truth_table(false, bool::kick, true, false);
        truth_table(true, bool::kick, false, false);
    }

    #[test]
    fn punch() {
        truth_table(false, bool::punch, false, true);
        truth_table(true, bool::punch, true, true);
    }
}
