extern crate quine_mc_cluskey;
#[macro_use] extern crate quickcheck;

use quine_mc_cluskey::{Bool, Term};
use quickcheck::TestResult;

quickcheck! {
    fn cmp_bool(b: Bool) -> bool {
        b == b
    }
    fn stringify_term(t: Term) -> bool {
        use std::str::FromStr;
        Term::from_str(&format!("{:?}", t)).unwrap() == t
    }
    fn simplify(b: Bool) -> TestResult {
        if b.terms().count_ones() > 10 {
            TestResult::discard()
        } else {
            TestResult::from_bool(!b.simplify().is_empty())
        }
    }
}
