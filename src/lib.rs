//! Semantically enriched value mutation.

/// This trait adds ergonomic convenience functions to `bool`.
pub trait PowerBool {
    /// Set the value to `true`, and say if it changed.
    #[inline(always)]
    fn raise(&mut self) -> bool {
        self.set(true)
    }

    /// Set the value to `false`, and say if it changed.
    #[inline(always)]
    fn lower(&mut self) -> bool {
        self.set(false)
    }

    /// Set the value, and say if it changed.
    ///
    /// Don't use a boolean literal with this function, `val` should be an expression:
    ///
    /// * Instead of `set(true)`, use `raise()`.
    /// * Instead of `set(false)`, use `lower()`.
    #[inline(always)]
    fn set(&mut self, val: bool) -> bool;

    /// Returns `true` if the value was `false`. This is not the same as `set()`.
    ///
    /// ```
    /// use powerbool::PowerBool;
    /// let list = vec![0.23, 3.4, 8.0, 9.6];
    /// let mut found = false;
    /// for x in list {
    ///     if found.trip((x as i32 as f32) == x) {
    ///         println!("Alert! There's an integer hiding in our number!");
    ///         return;
    ///     }
    /// }
    /// println!("Nobody here but us floats.");
    /// # panic!();
    /// ```
    #[inline(always)]
    fn trip(&mut self, val: bool) -> bool;

    /// Makes sure the value is `false`, and return `true` if it didn`t change.
    /// (Am I kicking a dead horse?)
    #[inline(always)]
    fn kick(&mut self) -> bool {
        !self.set(false)
    }

    /// Makes sure the value is `true`, and return `true` if it didn't change.
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
        let ret = val && !*self;
        if ret {
            *self = true;
        }
        ret
    }
}

#[cfg(test)]
mod test_powerbool {
    //! It would be silly to have two names for the same truth table.
    //!
    //! But some things are acceptable:
    //!
    //!  * Eliminating a bool literal
    //!  * Eliminating a return value negation
    //!  * Eliminating a following value change
    //!
    //! What's not okay?
    //!
    //!  * `foo()` being logically identical to `bar()`. (Unless there's some kind of semantic/usage
    //!  distinction. But maybe that point we'd want actual new types.)
    use super::PowerBool;

    fn truth_table_row<F: Fn(&mut bool) -> bool>(before: bool, f: F, returns: bool, after: bool) {
        let mut v = before;
        assert_eq!(f(&mut v), returns);
        assert_eq!(v, after);
    }

    #[test]
    fn set() {
        fn set_true(v: &mut bool) -> bool { v.set(true) }
        fn set_false(v: &mut bool) -> bool { v.set(false) }
        truth_table_row(true, set_true, false, true);
        truth_table_row(true, set_false, true, false);
        truth_table_row(false, set_true, true, true);
        truth_table_row(false, set_false, false, false);
    }

    #[test]
    fn raise() {
        truth_table_row(true, bool::raise, false, true);
        truth_table_row(false, bool::raise, true, true);
    }

    #[test]
    fn lower() {
        truth_table_row(true, bool::lower, true, false);
        truth_table_row(false, bool::lower, false, false);
    }

    #[test]
    fn trip() {
        fn trip_true(v: &mut bool) -> bool { v.trip(true) }
        fn trip_false(v: &mut bool) -> bool { v.trip(false) }
        truth_table_row(true, trip_true, false, true);
        truth_table_row(true, trip_false, false, true);
        truth_table_row(false, trip_true, true, true);
        truth_table_row(false, trip_false, false, false);
    }

    #[test]
    fn kick() {
        truth_table_row(false, bool::kick, true, false);
        truth_table_row(true, bool::kick, false, false);
    }

    #[test]
    fn punch() {
        truth_table_row(false, bool::punch, false, true);
        truth_table_row(true, bool::punch, true, true);
    }
}

pub trait Change {
    /// If this value does not equal the other value, become that value and return true.
    #[inline(always)]
    fn change(&mut self, v: Self) -> bool;
}
impl<T: PartialEq> Change for T {
    #[inline(always)]
    fn change(&mut self, v: Self) -> bool {
        if self != &v {
            *self = v;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test_change {
    use super::Change;

    #[test]
    fn works() {
        let mut x = 1;
        assert!(x.change(2));
    }

    #[test]
    fn still_works() {
        let mut x = 1;
        assert!(!x.change(1));
    }
}
