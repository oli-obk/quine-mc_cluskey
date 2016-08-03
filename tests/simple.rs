extern crate quine_mc_cluskey;

use quine_mc_cluskey::*;
use quine_mc_cluskey::Bool::{True, False, And, Not, Or};

#[test]
fn ident() {
    assert_eq!(essential_minterms(And(vec![Bool::Term(0), Bool::Term(1)]).minterms()).essentials, vec![Term::new(3)]);
    assert_eq!(essential_minterms(True.minterms()).essentials, vec![Term::new(0)]);
    assert_eq!(essential_minterms(False.minterms()).essentials, vec![]);
    assert_eq!(essential_minterms(Bool::Term(0).minterms()).essentials, vec![Term::new(1)]);
    assert_eq!(True.simplify(), vec![True]);
    assert_eq!(False.simplify(), vec![False]);
    assert_eq!(Bool::Term(0).simplify(), vec![Bool::Term(0)]);
    assert_eq!(Not(Box::new(Bool::Term(0))).simplify(), vec![Not(Box::new(Bool::Term(0)))]);
    assert_eq!(And(vec![Bool::Term(0), Bool::Term(1)]).simplify(), vec![And(vec![Bool::Term(0), Bool::Term(1)])]);
    assert_eq!(And(vec![Not(Box::new(Bool::Term(0))), Bool::Term(1)]).simplify(), vec![And(vec![Not(Box::new(Bool::Term(0))), Bool::Term(1)])]);
    assert_eq!(Or(vec![Bool::Term(0), Bool::Term(1)]).simplify(), vec![Or(vec![Bool::Term(0), Bool::Term(1)])]);
}

#[test]
fn simple() {
    assert_eq!(Not(Box::new(False)).simplify(), vec![True]);
    assert_eq!(And(vec![Bool::Term(0), Or(vec![Bool::Term(1), Bool::Term(0)])]).simplify(), vec![Bool::Term(0)]);
    assert_eq!(And(vec![Bool::Term(0), False]).simplify(), vec![False]);
    assert_eq!(And(vec![Bool::Term(0), True]).simplify(), vec![Bool::Term(0)]);
    assert_eq!(Or(vec![Bool::Term(0), False]).simplify(), vec![Bool::Term(0)]);
    assert_eq!(Or(vec![Bool::Term(0), True]).simplify(), vec![True]);
    assert_eq!(Or(vec![Bool::Term(0), Not(Box::new(And(vec![Bool::Term(0), Bool::Term(1)])))]).simplify(), vec![True]);
    assert_eq!(Or(vec![Bool::Term(0), Not(Box::new(Or(vec![Bool::Term(0), Bool::Term(1)])))]).simplify(), vec![Or(vec![Not(Box::new(Bool::Term(1))), Bool::Term(0)])]);
}

#[test]
fn debug() {
    assert_eq!(&format!("{:?}", True), "T");
    assert_eq!(&format!("{:?}", False), "F");
    assert_eq!(&format!("{:?}", Bool::Term(0)), "a");
    assert_eq!(&format!("{:?}", Bool::Term(2)), "c");
    assert_eq!(&format!("{:?}", Not(Box::new(Bool::Term(2)))), "c'");
    assert_eq!(&format!("{:?}", And(vec![True, Not(Box::new(Bool::Term(2)))])), "Tc'");
}

#[test]
fn wikipedia2() {
    let d = || Bool::Term(3);
    let c = || Bool::Term(2);
    let b = || Bool::Term(1);
    let a = || Bool::Term(0);
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
    let simple = expr.simplify();
    assert_eq!(simple, vec![
        Or(vec![
            And(vec![b(), not(c()), not(d())]),
            And(vec![a(), c()]),
            And(vec![a(), not(b())]),
        ]),
    ]);
}

#[test]
fn wikipedia() {
    let mut minterms: Vec<Term> = [4u32, 8, 9, 10, 12, 11, 14, 15].iter().map(|&i| Term::new(i)).collect();
    minterms.sort();

    let essentials = essential_minterms(minterms);
    assert_eq!(
        essentials.essentials,
        vec![
            Term::with_dontcare(4, 8),
            Term::with_dontcare(8, 3),
            Term::with_dontcare(10, 5),
            Term::with_dontcare(8, 6),
        ]
    );
    println!("{:#?}", essentials);
    let expr = essentials.prime_implicant_expr();
    println!("{:?}", expr);
    let simple = simplify_prime_implicant_expr(expr);
    let shortest = simple.iter().map(Vec::len).min().unwrap();

    let simple = simple.into_iter()
                       .filter(|v| v.len() == shortest)
                       .map(|v| Or(v.into_iter()
                                          .map(|i| essentials.essentials[i as usize].to_bool_expr(4))
                                          .collect()))
                       .collect::<Vec<Bool>>();
    assert_eq!(&format!("{:?}", simple), "[a'b'c + c'd + bd]");
}
