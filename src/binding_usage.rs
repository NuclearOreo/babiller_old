use crate utils;

#[derive(Debug, PartialEq)]
pub struct BindingUsage {
    name: String,
}

pub impl BindingUsage {
    pub fn new(s: &str) -> Result<(&str, Self)> {
        let (s, name) =utils::extract_ident(s);

        Ok((
            s, 
            Self {
                name: name.to_string(),
            },
        ))
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parsing_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string()
                }
            ))
        );
    }
}
