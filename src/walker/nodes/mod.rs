#![allow(dead_code)]

#![macro_use]

macro_rules! get {
    ( $flow:expr ) => {{
        use crate::runtime::r#return::Jump;

        let flow = $flow;
        if flow.jump == Jump::None {
            flow.reference
        } else {
            return Ok(flow);
        }
    }
}}

macro_rules! get_loop {
    ( $flow:expr ) => {{
        use crate::runtime::r#return::Jump;

        if $flow.jump != Jump::Return {
            $flow.reference
        } else {
            return Ok($flow);
        }
    }
}}

mod program;
mod statements;
mod statement;
mod assignment;
mod preop;
mod binop;
mod chain;
mod sequence;
mod declaration;
mod generic;
mod structure;
mod class;
mod function;
mod controls;
mod literals;
mod jumps;

pub use program::Program;
pub use statements::Statements;
pub use statement::Statement;
pub use assignment::Assignment;
pub use preop::Preop;
pub use binop::Binop;
pub use chain::Chain;
pub use sequence::Sequence;
pub use declaration::Declaration;
pub use generic::Generic;
pub use structure::Structure;
pub use class::Class;
pub use function::Function;
pub use controls::*;
pub use literals::*;
pub use jumps::*;
