extern crate quine_mc_cluskey;

use quine_mc_cluskey::*;
use std::str::FromStr;

#[test]
fn wikipedia() {
    /*
    | 0 1 2 5 6 7
    ---------------|------------
    K (0,1) a'b' | X X
    L (0,2) a'c' | X   X
    M (1,5) b'c  |   X   X
    N (2,6) bc'  |     X   X
    P (5,7) ac   |       X   X
    Q (6,7) ab   |         X X
    */

    let minterms = [0, 1, 2, 5, 6, 7].iter().cloned().map(Term::new).collect();
    let variables = 3;
    let essentials = essential_minterms(minterms);
    assert_eq!(essentials.essentials, vec![
        Term::from_str("000-").unwrap(),
        Term::from_str("011-").unwrap(),
        Term::from_str("00-0").unwrap(),
        Term::from_str("01-1").unwrap(),
        Term::from_str("0-01").unwrap(),
        Term::from_str("0-10").unwrap(),
    ]);
    let expr = essentials.prime_implicant_expr();
    assert_eq!(expr, vec![
        vec![vec![0], vec![1]],
        vec![vec![4], vec![5]],
        vec![vec![0], vec![2]],
        vec![vec![3], vec![5]],
        vec![vec![1], vec![3]],
        vec![vec![2], vec![4]],
    ]);
    let simple = simplify_prime_implicant_expr(expr);
    assert_eq!(simple, vec![
        vec![0, 3, 4],
        vec![1, 2, 3, 4],
        vec![0, 1, 4, 5],
        vec![1, 2, 5],
        vec![0, 2, 3, 5],
    ]);
    let shortest = simple.iter().map(|v| v.len()).min().unwrap();
    let solutions = simple.into_iter()
                          .filter(|v| v.len() == shortest)
                          .map(|v| {
                              Bool::Or(v.iter().map(|&i| {
                                  essentials.essentials[i as usize].to_bool_expr(variables)
                              }).collect())
                          })
                          .collect::<Vec<Bool>>();
    assert_eq!(&format!("{:?}", solutions), "[a'b' + ac + b'c, ab + a'c' + bc']");
}
