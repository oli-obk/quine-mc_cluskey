[![Clippy Linting Result](https://clippy.bashy.io/github/oli-obk/quine-mc_cluskey/master/badge.svg)](https://clippy.bashy.io/github/oli-obk/quine-mc_cluskey/master/log)
[![Current Version](http://meritbadge.herokuapp.com/quine-mc_cluskey)](https://crates.io/crates/quine-mc_cluskey)

An algorithm to automatically minimize boolean expressions.

#Example

```rust
extern crate quine_mc_cluskey;

use quine_mc_cluskey::*;
use quine_mc_cluskey::Bool::*;

fn main() {
    // !false => true
    assert_eq!(Not(Box::new(False)).simplify(), vec![True]);
    // a && (b || a) => a
    assert_eq!(And(vec![Term(0), Or(vec![Term(1), Term(0)])]).simplify(), vec![Term(0)]);
}
```

#Obtaining a minimal "and of or" form

Sometimes an expression of the form `a && (b || c)` is shorter than the `a && b || a && c` form.
We can simply negate the original expression and negate all the resulting simplified expressions to obtain that form.

```rust
let a: Bool = ....;
let simplified: Vec<Bool> = Not(Box::new(a)).simplify().map(simple_negate).collect();

fn simple_negate(b: Bool) -> Bool {
    use quine_mc_cluskey::Bool::*;
    match b {
        True => False,
        False => True,
        t @ Term(_) => Not(Box::new(t)),
        And(mut v) => {
            for el in &mut v {
                *el = simple_negate(::std::mem::replace(el, True));
            }
            Or(v)
        },
        Or(mut v) => {
            for el in &mut v {
                *el = simple_negate(::std::mem::replace(el, True));
            }
            And(v)
        },
        Not(inner) => *inner,
    }
}
```
