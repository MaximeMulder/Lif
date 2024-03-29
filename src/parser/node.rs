use crate::parser::Code;
use crate::parser::Element;
use crate::memory::Ref;

#[derive(Clone)]
pub struct CNode {
    pub code: Ref<Code>,
    pub element: &'static Element,
    children: Box<[CNode]>,
    left: usize,
    right: usize,
}

impl CNode {
    pub fn new(code: Ref<Code>, element: &'static Element, children: Box<[CNode]>, left: usize, right: usize) -> Self {
        debug_assert!(right >= left);
        Self {
            code,
            element,
            children,
            left,
            right,
        }
    }

    pub fn new_token(code: Ref<Code>, element: &'static Element, left: usize, right: usize) -> Self {
        Self::new(code, element,  Box::new([]), left, right)
    }

    pub fn new_production(code: Ref<Code>, element: &'static Element, children: Box<[CNode]>) -> Self {
        let (left, right) = if !children.is_empty() {
            (children.first().unwrap().left(), children.first().unwrap().right())
        } else {
            (0, 0)
        };

        Self::new(code, element, children, left, right)
    }

    pub fn left(&self) -> usize {
        self.left
    }

    pub fn right(&self) -> usize {
        self.right
    }

    pub fn text(&self) -> Box<str> {
        Box::from(self.code.node_str(self))
    }

    pub fn at(&self, index: usize) -> &CNode {
        &self.children()[index]
    }

    pub fn children(&self) -> &[CNode] {
        &self.children
    }
}
