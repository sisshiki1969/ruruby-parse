#![feature(const_option)]
mod error;
mod id_table;
mod lvar_collector;
mod node;
mod parser;
mod source_info;
mod token;
pub use error::*;
pub use id_table::*;
pub use lvar_collector::*;
pub use node::*;
pub use parser::*;
pub use source_info::*;
use token::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Annot<T: PartialEq + Default> {
    pub kind: T,
    pub loc: Loc,
}

impl<T: PartialEq + Default> Annot<T> {
    pub fn new(kind: T, loc: Loc) -> Self {
        Annot { kind, loc }
    }

    pub fn loc(&self) -> Loc {
        self.loc
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use crate::parser::*;
        let res =
            Parser::parse_program("nil".to_string(), std::path::PathBuf::from("path")).unwrap();
        eprintln!("{:?}", res)
    }
}
