#[derive(Debug, Eq, PartialEq)]
pub enum Bool {
    True,
    False,
    Term(u8),
    And(Box<Bool>, Box<Bool>),
    Or(Box<Bool>, Box<Bool>),
    Not(Box<Bool>),
}

impl Bool {
    fn terms(&self) -> u32 {
        use self::Bool::*;
        match *self {
            Term(u) => 1 << u,
            Or(ref a, ref b) |
            And(ref a, ref b) => a.terms() | b.terms(),
            Not(ref a) => a.terms(),
            True | False => 0,
        }
    }

    fn eval(&self, terms: u32) -> bool {
        use self::Bool::*;
        match *self {
            True => true,
            False => false,
            Term(i) => (terms & (1 << i)) != 0,
            And(ref a, ref b) => a.eval(terms) && b.eval(terms),
            Or(ref a, ref b) => a.eval(terms) || b.eval(terms),
            Not(ref a) => !a.eval(terms),
        }
    }
}

#[derive(Debug, Clone)]
struct Term {
    dontcare: u32,
    term: u32,
}

impl Term {
    fn new(i: u32) -> Self {
        Term {
            dontcare: 0,
            term: i,
        }
    }

    fn combine(&self, other: &Term) -> Option<Term> {
        let dc = self.dontcare ^ other.dontcare;
        let term = self.term ^ other.term;
        let dc_mask = self.dontcare | other.dontcare;
        match (dc.count_ones(), (dc_mask & term).count_ones()) {
            (0, 1) |
            (1, 0) => Some(Term {
                dontcare: dc_mask,
                term: self.term,
            }),
            _ => None,
        }
    }
}

pub fn simplify(expression: &Bool) -> Vec<Bool> {
    let terms = expression.terms();
    let nterms = terms.count_ones();
    for i in 0..nterms {
        if terms & (1 << i) == 0 {
            panic!("non-continuous naming scheme");
        }
    }
    // minterms consist of other minterms
    let mut minterms: Vec<Term> = Vec::new();
    // initialize the minterms
    for i in 0..(1 << nterms) {
        if expression.eval(i) {
            minterms.push(Term::new(i));
        }
    }
    let mut essentials: Vec<Term> = Vec::new();
    while !minterms.is_empty() {
        let old = std::mem::replace(&mut minterms, Vec::new());
        for (i, term) in old.iter().enumerate() {
            let mut combined = false;
            for other in &old[i..] {
                if let Some(new_term) = term.combine(other) {
                    minterms.push(new_term);
                    combined = true;
                }
            }
            if !combined {
                essentials.push(term.clone());
            }
        }
    }
    println!("{:#?}", essentials);
    unimplemented!()
}
