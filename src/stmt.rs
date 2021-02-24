use crate::binding_def::BindingDef;
use crate::expr::Expr;

#[cfg(Debug, PartialEq)]
mod Test {
    use super::*;
    use crate::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Stmt::new("let a = 10"),
            Ok((
                "",
                Stmt::BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(10)),
                }
            ))
        );
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Stmt::new("1 + 1"),
            Ok((
                "",
                Stmt::Expr {
                    lhs: Number(1),
                    rhs: Number(1),
                    op: Op::Add,
                }
            ))
        );
    }
}
