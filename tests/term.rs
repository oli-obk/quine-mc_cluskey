extern crate quine_mc_cluskey;

use quine_mc_cluskey::*;

#[test]
fn cmp() {
    let a = Term::with_dontcare(2, 8);
    let b = Term::with_dontcare(10, 8);
    assert_eq!(a, b)
}

#[test]
fn combine_4_12() {
    let a = Term::new(4);
    let b = Term::new(12);
    assert_eq!(a.combine(&b), Some(Term::with_dontcare(4, 8)))
}

#[test]
fn combine_wikipedia() {
    let a = Term::with_dontcare(6, 8);
    let b = Term::with_dontcare(4, 8);
    assert_eq!(a.combine(&b), Some(Term::with_dontcare(4, 10)));
    let a = Term::with_dontcare(6, 8);
    let b = Term::with_dontcare(6, 9);
    assert_eq!(a.combine(&b), Some(Term::with_dontcare(6, 9)));
    let a = Term::with_dontcare(6, 8);
    let b = Term::with_dontcare(6, 1);
    assert_eq!(a.combine(&b), None);
}
