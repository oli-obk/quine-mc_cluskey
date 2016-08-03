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
    fn bool_clone(a: Bool) -> bool {
        let b = a.clone();
        a == b
    }
    fn bool_eval_and_neg(a: Bool, b: Bool, terms: u32) -> bool {
        let a2 = a.clone();
        let b2 = b.clone();
        (!(a & b)).eval(terms) == (!a2 | !b2).eval(terms)
    }
    fn bool_eval_or_neg(a: Bool, b: Bool, terms: u32) -> bool {
        let a2 = a.clone();
        let b2 = b.clone();
        (!(a | b)).eval(terms) == (!a2 & !b2).eval(terms)
    }
    fn bool_eval_neg(b: Bool, terms: u32) -> bool {
        let a = !b.clone();
        a.eval(terms) != b.eval(terms)
    }
}
