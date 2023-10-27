use crate::ast::nodes::*;
use crate::runtime::bis::engine::Engine;
use crate::runtime::bis::flow::{Flow, Jump, ResJump};

macro_rules! flow_jump {
    ( $flow:expr ) => {{
        let flow = $flow?;
        match flow {
            Flow::Value(value) => value,
            Flow::Jump(jump) => return Jump::some(jump),
        }
    }}
}

pub(crate) use flow_jump;

impl AStmt {
    pub fn eval_stmt<'a>(&self, engine: &mut Engine<'a>) -> ResJump<'a> {
        match self {
            AStmt::Def(def) => def.eval_stmt(engine),
            AStmt::Expr(expr) => expr.eval_stmt(engine),
        }
    }
}

impl ADef {
    fn eval_stmt<'a>(&self, engine: &mut Engine<'a>) -> ResJump<'a> {
        self.eval_def(engine)?;
        Jump::none()
    }
}

impl AExpr {
    fn eval_stmt<'a>(&self, engine: &mut Engine<'a>) -> ResJump<'a> {
        flow_jump!(self.eval(engine));
        Jump::none()
    }
}
