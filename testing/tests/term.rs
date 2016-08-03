extern crate quine_mc_cluskey;

use std::str::FromStr;
use quine_mc_cluskey::*;

#[test]
fn cmp() {
    let a = Term::with_dontcare(2, 8);
    let b = Term::with_dontcare(10, 8);
    assert_eq!(a, b);
    assert_eq!(Ok(a), Term::from_str("-010"));
    assert_eq!(Ok(b), Term::from_str("-010"));
}

#[test]
fn combine_4_12() {
    let a = Term::new(4);
    let b = Term::new(12);
    assert_eq!(a.combine(&b), Some(Term::with_dontcare(4, 8)));
    assert_eq!(a.combine(&b).ok_or(TermFromStrError::UnsupportedCharacter('\0')), Term::from_str("-100"));
    assert_eq!(Ok(a), Term::from_str("100"));
    assert_eq!(Ok(b), Term::from_str("1100"));
}

#[test]
fn combine_wikipedia() {
    let a = Term::with_dontcare(6, 8);
    let b = Term::with_dontcare(4, 8);
    assert_eq!(a.combine(&b), Some(Term::with_dontcare(4, 10)));
    assert_eq!(Ok(a), Term::from_str("-110"));
    assert_eq!(Ok(b), Term::from_str("-100"));
    let a = Term::with_dontcare(6, 8);
    let b = Term::with_dontcare(6, 9);
    assert_eq!(a.combine(&b), Some(Term::with_dontcare(6, 9)));
    assert_eq!(Ok(a), Term::from_str("-110"));
    assert_eq!(Ok(b), Term::from_str("-11-"));
    let a = Term::with_dontcare(6, 8);
    let b = Term::with_dontcare(6, 1);
    assert_eq!(a.combine(&b), None);
    assert_eq!(Ok(a), Term::from_str("-110"));
    assert_eq!(Ok(b), Term::from_str("011-"));
}


#[test]
fn contains() {
    assert!(Term::from_str("-110").unwrap().contains(&Term::from_str("1110").unwrap()));
    assert!(Term::from_str("-110").unwrap().contains(&Term::from_str("0110").unwrap()));
    assert!(!Term::from_str("000-").unwrap().contains(&Term::from_str("0010").unwrap()));
    assert!(!Term::from_str("000-").unwrap().contains(&Term::from_str("0101").unwrap()));
    assert!(!Term::from_str("000-").unwrap().contains(&Term::from_str("0110").unwrap()));
    assert!(!Term::from_str("000-").unwrap().contains(&Term::from_str("0111").unwrap()));
    assert!(Term::from_str("000-").unwrap().contains(&Term::from_str("0001").unwrap()));
    assert!(Term::from_str("000-").unwrap().contains(&Term::from_str("0000").unwrap()));
    assert!(!Term::from_str("-01").unwrap().contains(&Term::from_str("0").unwrap()));
    assert!(Term::from_str("-01").unwrap().contains(&Term::from_str("1").unwrap()));
    assert!(Term::from_str("-01").unwrap().contains(&Term::from_str("101").unwrap()));
}

#[test]
fn debug() {
    assert_eq!(&format!("{:?}", Term::from_str("-01").unwrap()), "00000000000000000000000000000-01");
    assert_eq!(&format!("{:?}", Term::from_str("11-").unwrap()), "0000000000000000000000000000011-");
}

#[test]
fn to_bool() {
// 0-0 + -10 + 11-
// 00- + -01 + 1-1

// KNP    expands to    a'b'+ bc'+ ac
// LMQ    expands to    a'c'+ b'c + ab
    assert_eq!(Term::from_str("11-").unwrap().to_bool_expr(3), Bool::And(vec![Bool::Term(1), Bool::Term(2)]));
    assert_eq!(Term::from_str("1-1").unwrap().to_bool_expr(3), Bool::And(vec![Bool::Term(0), Bool::Term(2)]));
    assert_eq!(Term::from_str("-01").unwrap().to_bool_expr(3), Bool::And(vec![Bool::Term(0), Bool::Not(Box::new(Bool::Term(1)))]));
    assert_eq!(Term::from_str("-10").unwrap().to_bool_expr(3), Bool::And(vec![Bool::Not(Box::new(Bool::Term(0))), Bool::Term(1)]));
}
