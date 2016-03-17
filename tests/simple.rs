extern crate quine_mc_cluskey;

use quine_mc_cluskey::*;
use quine_mc_cluskey::Bool::*;

#[test]
fn ident() {
    assert_eq!(simplify(&And(Box::new(Term(0)), Box::new(Term(1)))), vec![And(Box::new(Term(0)), Box::new(Term(1)))]);
    assert_eq!(simplify(&True), vec![True]);
    assert_eq!(simplify(&False), vec![False]);
    assert_eq!(simplify(&Term(0)), vec![Term(0)]);
}
