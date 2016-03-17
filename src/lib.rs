extern crate bit_vec;
use bit_vec::BitVec;

#[derive(Debug, Eq, PartialEq)]
pub enum Bool {
    True,
    False,
    Term(u8),
    And(Box<Bool>, Box<Bool>),
    Or(Box<Bool>, Box<Bool>),
    Not(Box<Bool>),
}

use Bool::*;

fn terms(expr: &Bool) -> u32 {
    match *expr {
        Term(u) => 1 << u,
        Or(ref a, ref b) |
        And(ref a, ref b) => terms(a) | terms(b),
        Not(ref a) => terms(a),
        True | False => 0,
    }
}

fn eval(expr: &Bool, terms: u32) -> bool {
    match *expr {
        True => true,
        False => false,
        Term(i) => (terms & (1 << i)) != 0,
        And(ref a, ref b) => eval(a, terms) && eval(b, terms),
        Or(ref a, ref b) => eval(a, terms) || eval(b, terms),
        Not(ref a) => !eval(a, terms),
    }
}

pub fn simplify(expression: &Bool) -> Vec<Bool> {
    let terms = terms(expression);
    let nterms = terms.count_ones();
    for i in 0..nterms {
        if terms & (1 << i) == 0 {
            panic!("non-continuous naming scheme");
        }
    }
    let truth_table = BitVec::from_fn(1 << nterms, |i| {
        debug_assert_eq!(i as u32 as usize, i);
        let i = i as u32;
        eval(expression, i)
    });
    println!("{:#?}", truth_table);
    unimplemented!()
}
