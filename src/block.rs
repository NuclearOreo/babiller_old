use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub exprs: Vec<Expr>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { exprs: Vec::new() })));
    }
}
