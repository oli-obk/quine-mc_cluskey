#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Bool {
    True,
    False,
    Term(u8),
    And(Vec<Bool>),
    Or(Vec<Bool>),
    Not(Box<Bool>),
}

impl Bool {
    fn terms(&self) -> u32 {
        use self::Bool::*;
        match *self {
            Term(u) => 1 << u,
            Or(ref a) |
            And(ref a) => a.iter().fold(0, |state, item| { state | item.terms() }),
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
            And(ref a) => a.iter().all(|item| item.eval(terms)),
            Or(ref a) => a.iter().any(|item| item.eval(terms)),
            Not(ref a) => !a.eval(terms),
        }
    }
}

#[derive(Clone, Eq)]
pub struct Term {
    dontcare: u32,
    term: u32,
}

impl std::fmt::Debug for Term {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..32 {
            if (self.dontcare & (1 << i)) != 0 {
                try!(write!(fmt, "-"));
            } else if (self.term & (1 << i)) != 0 {
                try!(write!(fmt, "1"));
            } else {
                try!(write!(fmt, "0"));
            }
        }
        Ok(())
    }
}

impl std::cmp::PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        (self.dontcare == other.dontcare) && ((self.term & !self.dontcare) == (other.term & !other.dontcare))
    }
}

impl Term {
    pub fn new(i: u32) -> Self {
        Term {
            dontcare: 0,
            term: i,
        }
    }

    pub fn with_dontcare(term: u32, dontcare: u32) -> Self {
        Term {
            dontcare: dontcare,
            term: term,
        }
    }

    pub fn combine(&self, other: &Term) -> Option<Term> {
        let dc = self.dontcare ^ other.dontcare;
        let term = self.term ^ other.term;
        let dc_mask = self.dontcare | other.dontcare;
        match (dc.count_ones(), (!dc_mask & term).count_ones()) {
            (0, 1) |
            (1, 0) => Some(Term {
                dontcare: dc_mask | term,
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
        println!("{:#?}", essentials);
        println!("{:#?}", minterms);
        let old = std::mem::replace(&mut minterms, Vec::new());
        let mut combined_terms = std::collections::BTreeSet::new();
        for (i, term) in old.iter().enumerate() {
            for (other_i, other) in old[i..].iter().enumerate() {
                if let Some(new_term) = term.combine(other) {
                    println!("combined {} and {}", i, other_i + i);
                    minterms.push(new_term);
                    combined_terms.insert(other_i + i);
                    combined_terms.insert(i);
                }
            }
            if !combined_terms.contains(&i) {
                println!("{} is essential", i);
                essentials.push(term.clone());
            }
        }
        minterms.dedup();
    }
    println!("{:#?}", essentials);
    unimplemented!()
}
