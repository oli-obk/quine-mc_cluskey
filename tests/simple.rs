extern crate quine_mc_cluskey;

use quine_mc_cluskey::*;
use quine_mc_cluskey::Bool::*;

#[test]
fn ident() {
    assert_eq!(essential_minterms(And(vec![Term(0), Term(1)]).minterms()).essentials, vec![Term::new(3)]);
    assert_eq!(essential_minterms(True.minterms()).essentials, vec![Term::new(0)]);
    assert_eq!(essential_minterms(False.minterms()).essentials, vec![]);
    assert_eq!(essential_minterms(Term(0).minterms()).essentials, vec![Term::new(1)]);
}

#[test]
fn debug() {
    assert_eq!(&format!("{:?}", True), "T");
    assert_eq!(&format!("{:?}", False), "F");
    assert_eq!(&format!("{:?}", Term(0)), "a");
    assert_eq!(&format!("{:?}", Term(2)), "c");
    assert_eq!(&format!("{:?}", Not(Box::new(Term(2)))), "c'");
    assert_eq!(&format!("{:?}", And(vec![True, Not(Box::new(Term(2)))])), "Tc'");
}

#[test]
fn wikipedia2() {
    let d = || Term(3);
    let c = || Term(2);
    let b = || Term(1);
    let a = || Term(0);
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
                       .map(|v| Bool::Or(v.into_iter()
                                          .map(|i| essentials.essentials[i as usize].to_bool_expr(4))
                                          .collect()))
                       .collect::<Vec<Bool>>();
    assert_eq!(&format!("{:?}", simple), "[a'b'c + c'd + bd]");
}
