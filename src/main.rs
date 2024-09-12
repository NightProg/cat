use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum Expr {
    Sym(String),
    Fun(String, Vec<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Sym(s) => write!(f, "{}", s),
            Expr::Fun(s, args) => {
                write!(f, "{}(", s)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Clone)]
struct Rule {
    head: Expr,
    body: Expr,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} :- {}", self.head, self.body)
    }
}

type Bindings = HashMap<String, Expr>;

struct Pattern {
    bindings: Bindings,
}

impl Pattern {
    fn pattern_match(&mut self, pattern: Expr, expr: Expr) -> Option<Bindings> {
        match (pattern, expr) {
            (Expr::Sym(s1), expr) => {
                if let Some(e) = self.bindings.get(&s1) {
                    if *e == expr {
                        Some(self.bindings.clone())
                    } else {
                        None
                    }
                } else {
                    self.bindings.insert(s1, expr);
                    Some(self.bindings.clone())
                }
    
            }
            (Expr::Fun(f1, args1), Expr::Fun(f2, args2)) => {
                if f1 == f2 && args1.len() == args2.len() {
                for (arg1, arg2) in args1.iter().zip(args2.iter()) {
                        match self.pattern_match(arg1.clone(), arg2.clone()) {
                            Some(b) => {
                                self.bindings.extend(b);
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                    Some(self.bindings.clone())
                } else {
                    None
                }
            }
            _ => None,
        }

    }
}
fn replace(expr: Expr, bindings: Bindings) -> Expr {
    match expr {
        Expr::Sym(s) => {
            match bindings.get(&s) {
                Some(e) => e.clone(),
                None => Expr::Sym(s),
            }
        }
        Expr::Fun(f, args) => {
            Expr::Fun(f, args.into_iter().map(|arg| replace(arg, bindings.clone())).collect())
        }
    }
}

fn main() {
    let expr = Expr::Fun(
        "swap".to_string(),
        vec![
            Expr::Fun("pair".to_string(), vec![Expr::Sym("X".to_string()), Expr::Sym("Y".to_string())]),
        ],
    );
    let body = Expr::Fun("pair".to_string(), vec![Expr::Sym("Y".to_string()), Expr::Sym("X".to_string())]);
    let rule = Rule { head: expr, body: body };

    let x = Expr::Fun(
        "swap".to_string(),
        vec![
            Expr::Fun("pair".to_string(), vec![
                Expr::Sym("a".to_string()),
                Expr::Sym("b".to_string()),
            ]),
        ],
    );
    let bindings = Pattern {
        bindings: HashMap::new(),
    }.pattern_match(rule.head.clone(), x.clone());

    println!("{}", rule);
    println!("{}", replace(rule.body, bindings.unwrap()));
}
