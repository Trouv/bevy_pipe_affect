//! A test utility for generating one-way-functions with proptest.

use blake2::digest::consts::U16;
use blake2::{Blake2b, Digest};
use proptest::prelude::*;
use proptest_derive::Arbitrary;

const MAX_COMPLEXITY: u8 = 32;

/// A pseudo one-way-function that can be used as `proptest` input.
///
/// Since `proptest` requires that input types implement `Debug`, it's impossible to have
/// `proptest` use a closure as input.
/// So, instead, we can have `proptest` generate this fairly normal type, which determines the
/// behavior of [`OneWayFn::call`] - the actual "generated" function.
///
/// When `complexity` is 0, `call` is the identity function.
/// In any other case, it involves blake2 hashing, so it is cryptographically a one-way-function.
/// This can be useful for essentially proving that some higher order function actually called its
/// input function - it's very unlikely for it to have "guessed" the correct output.
/// While 0 being the identity provides a reasonable "trivial case" for `proptest` shrinking.
///
/// For now this only provides a function like `Fn(u128) -> u128`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Arbitrary)]
pub struct OneWayFn {
    #[proptest(strategy = "0..MAX_COMPLEXITY")]
    complexity: u8,
}

fn hash_once(input: [u8; 16]) -> [u8; 16] {
    let mut hasher = Blake2b::<U16>::new();
    hasher.update(input);
    hasher.finalize().into()
}

impl OneWayFn {
    /// Performs the function on the input.
    pub fn call(&self, input: u128) -> u128 {
        u128::from_be_bytes((0..self.complexity).fold(input.to_be_bytes(), |acc, _| hash_once(acc)))
    }
}

proptest! {
    #[test]
    fn one_way_fn_of_0_complexity_is_identity(input: u128) {
        let output = OneWayFn { complexity: 0 }.call(input);
        prop_assert_eq!(input, output);
    }

    #[test]
    fn one_way_fn_complexity_changes_output_inductively(input: u128, hypothesis_complexity in 0..MAX_COMPLEXITY - 1) {
        let hypothesis_one_way_fn = OneWayFn { complexity: hypothesis_complexity };
        let step_one_way_fn = OneWayFn { complexity: hypothesis_complexity + 1 };

        prop_assert_ne!(hypothesis_one_way_fn.call(input), step_one_way_fn.call(input));
    }
}
