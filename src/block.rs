use crate::stmt::Stmt;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, stmts) = if let Ok((s, stmt)) = Stmt::new(s) {
            (s, vec![stmt])
        } else {
            (s, Vec::new())
        };

        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;

        Ok((s, Block { stmts }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::expr::{Expr, Number};

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new() })));
    }

    #[test]
    fn parse_empty_block_with_whitespaces() {
        assert_eq!(Block::new("{    }"), Ok(("", Block { stmts: Vec::new() })));
    }

    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(5)))],
                },
            )),
        );
    }

    //     #[test]
    //     fn parse_block_with_multiple_stmts() {
    //         assert_eq!(
    //             Block::new(
    //                 "{
    //     let a = 10
    //     let b = a
    //     b
    // }",
    //             ),
    //             Ok((
    //                 "",
    //                 Block {
    //                     stmts: vec![
    //                         Stmt::BindingDef(BindingDef {
    //                             name: "a".to_string(),
    //                             val: Expr::Number(Number(10)),
    //                         }),
    //                         Stmt::BindingDef(BindingDef {
    //                             name: "b".to_string(),
    //                             val: ?, // what do we put here?
    //                         }),
    //                         Stmt::Expr(?), // and here?
    //                     ],
    //                 },
    //             )),
    //         );
    //     }
}
