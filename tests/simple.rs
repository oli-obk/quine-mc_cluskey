extern crate quine_mc_cluskey;

use quine_mc_cluskey::*;
use quine_mc_cluskey::Bool::*;

#[test]
fn ident() {
    assert_eq!(simplify(&And(vec![Term(0), Term(1)])), vec![And(vec![Term(0), Term(1)])]);
    assert_eq!(simplify(&True), vec![True]);
    assert_eq!(simplify(&False), vec![False]);
    assert_eq!(simplify(&Term(0)), vec![Term(0)]);
}

#[test]
fn wikipedia() {
    let a = || Term(0);
    let b = || Term(1);
    let c = || Term(2);
    let d = || Term(3);
    let not = |x| Not(Box::new(x));
    let expr = Or(vec![
        And(vec![not(a()), b(), not(c()), not(d())]),
        And(vec![a(), not(b()), not(c()), not(d())]),
        And(vec![a(), not(b()), c(), not(d())]),
        And(vec![a(), not(b()), c(), d()]),
        And(vec![a(), b(), not(c()), not(d())]),
        And(vec![a(), b(), c(), d()]),
        And(vec![a(), not(b()), not(c()), d()]),
        And(vec![a(), b(), c(), not(d())]),
    ]);

    assert_eq!(simplify(&expr), vec![]);
}
