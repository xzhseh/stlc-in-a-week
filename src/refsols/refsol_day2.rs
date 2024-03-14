use std::collections::HashSet;

use crate::{
    expr::{app::App, cond::Cond, decr::Decr, incr::Incr, is_zero::IsZero, lambda::Lambda},
    Exp,
};

impl Exp {
    pub fn ref_appears_free_in(&self, var: &str) -> bool {
        /// A context based solution
        fn appears_free_in_inner(exp: Exp, var: &str, context: &mut HashSet<String>) -> bool {
            match exp.clone() {
                Exp::Lambda(lambda) => {
                    // update context before diving in
                    context.insert(lambda.arg.clone());
                    let result = appears_free_in_inner(lambda.exp, var, context);
                    // clear the context before exiting
                    // note that here we do not need to consider accidentially clearing
                    // the outer context, per the assumption above.
                    // i.e., case like `\x. y (\x. x) x` will not be included
                    context.remove(&lambda.arg);
                    result
                }
                Exp::Var(v) => {
                    if v == var.to_string() {
                        !context.contains(&v)
                    } else {
                        // not the variable we are interested in
                        false
                    }
                }
                Exp::App(app) => {
                    appears_free_in_inner(app.t1, var, context)
                        || appears_free_in_inner(app.t2, var, context)
                }
                Exp::Cond(cond) => {
                    appears_free_in_inner(cond.r#if, var, context)
                        || appears_free_in_inner(cond.r#then, var, context)
                        || appears_free_in_inner(cond.r#else, var, context)
                }
                Exp::IsZero(e) | Exp::Incr(e) | Exp::Decr(e) => {
                    appears_free_in_inner(*e, var, context)
                }
                Exp::Nat(_) | Exp::True | Exp::False => false,
            }
        }

        appears_free_in_inner(self.clone(), var, &mut HashSet::new())
    }

    pub fn ref_is_value(&self) -> bool {
        match self.clone() {
            Exp::Lambda(_) | Exp::True | Exp::False | Exp::Nat(_) => true,
            _ => false,
        }
    }

    pub fn ref_substitute(self, var: String, s: Exp) -> Exp {
        match self.clone() {
            // [x := s] x && [x := s] y
            Exp::Var(v) => {
                if v == var {
                    s
                } else {
                    self
                }
            }
            // [x := s] (\x. t) && [x := s] (\y. t)
            Exp::Lambda(lambda) => {
                if lambda.arg == var {
                    self
                } else {
                    Lambda::build(&lambda.arg, lambda.exp.ref_substitute(var, s))
                }
            }
            // [x := s] (t1 t2)
            Exp::App(app) => App::build(
                app.t1.ref_substitute(var.clone(), s.clone()),
                app.t2.ref_substitute(var, s),
            ),
            // [x := s] (if t1 then t2 else t3)
            Exp::Cond(cond) => Cond::build(
                cond.r#if.ref_substitute(var.clone(), s.clone()),
                cond.r#then.ref_substitute(var.clone(), s.clone()),
                cond.r#else.ref_substitute(var, s),
            ),
            Exp::IsZero(e) => IsZero::build(e.ref_substitute(var, s)),
            // [x := s] (inc t)
            Exp::Incr(e) => Incr::build(e.ref_substitute(var, s)),
            // [x := s] (dec t)
            Exp::Decr(e) => Decr::build(e.ref_substitute(var, s)),
            // [x := s] true && [x := s] false && [x := s] n
            e => e,
        }
    }
}
