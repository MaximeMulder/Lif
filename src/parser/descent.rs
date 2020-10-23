use crate::node::Node;
use crate::element::Element;
use crate::parser::Parser;

pub trait Descent<'a> {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>>;
}

pub struct DescentAlias {
    descent: usize,
}

impl DescentAlias {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl<'a> Descent<'a> for DescentAlias {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        parser.descent(self.descent)
    }
}

pub struct DescentAscent {
    descent: usize,
    ascent: usize,
}

impl DescentAscent {
    pub fn new(descent: usize, ascent: usize) -> Self {
        Self {
            descent,
            ascent,
        }
    }
}

impl<'a> Descent<'a> for DescentAscent {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        if let Some(nodes) = parser.descent(self.descent) {
            return parser.ascent(self.ascent, nodes);
        }

        None
    }
}

pub struct DescentChoice {
    descents: Box<[usize]>,
}

impl DescentChoice {
    pub fn new<const N: usize>(descents: [usize; N]) -> Self {
        Self {
            descents: Box::new(descents),
        }
    }
}

impl<'a> Descent<'a> for DescentChoice {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        for descent in self.descents.iter() {
            if let Some(nodes) = parser.descent(*descent) {
                return Some(nodes);
            }
        }

        None
    }
}

pub struct DescentSequence {
    descents: Box<[usize]>,
}

impl DescentSequence {
    pub fn new<const N: usize>(descents: [usize; N]) -> Self {
        Self {
            descents: Box::from(descents),
        }
    }
}

impl<'a> Descent<'a> for DescentSequence {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        let mut nodes = Vec::new();
        for descent in self.descents.iter() {
            if let Some(children) = parser.descent(*descent) {
                nodes.extend(children);
            } else {
                return None;
            }
        }

        Some(nodes)
    }
}

pub struct DescentZeroOrMore {
    descent: usize,
}

impl DescentZeroOrMore {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl<'a> Descent<'a> for DescentZeroOrMore {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        let mut nodes = Vec::new();
        while let Some(children) = parser.descent(self.descent) {
            nodes.extend(children);
        }

        Some(nodes)
    }
}

pub struct DescentOneOrMore {
    descent:   usize,
}

impl DescentOneOrMore {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl<'a> Descent<'a> for DescentOneOrMore {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        let mut nodes = Vec::new();
        while let Some(children) = parser.descent(self.descent) {
            nodes.extend(children);
        }

        if !nodes.is_empty() {
            Some(nodes)
        } else {
            None
        }
    }
}

pub struct DescentOption {
    descent: usize,
}

impl DescentOption {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl<'a> Descent<'a> for DescentOption {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        let nodes = parser.descent(self.descent);
        if nodes.is_some() {
            return nodes;
        }

        Some(Vec::new())
    }
}

pub struct DescentPredicateAnd {
    descent: usize,
}

impl DescentPredicateAnd {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl<'a> Descent<'a> for DescentPredicateAnd {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        if parser.descent_predicate(self.descent) {
            Some(Vec::new())
        } else {
            None
        }
    }
}

pub struct DescentPredicateNot {
    descent: usize,
}

impl DescentPredicateNot {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl<'a> Descent<'a> for DescentPredicateNot {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        if parser.descent_predicate(self.descent) {
            None
        } else {
            Some(Vec::new())
        }
    }
}

pub struct DescentElement<'a> {
    descent: usize,
    element: &'a Element,
}

impl<'a> DescentElement<'a> {
    pub fn new(descent: usize, element: &'a Element) -> Self {
        Self {
            descent,
            element,
        }
    }
}

impl<'a> Descent<'a> for DescentElement<'a> {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        if let Some(nodes) = parser.descent(self.descent) {
            Some(vec![Node::new_production(self.element, nodes)])
        } else {
            None
        }
    }
}

pub struct DescentToken<'a> {
    element: &'a Element,
}

impl<'a> DescentToken<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self {
            element,
        }
    }
}

impl<'a> Descent<'a> for DescentToken<'a> {
    fn descent(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        if let Some(token) = parser.next() {
            if token.element == self.element {
                return Some(vec![token]);
            }
        }

        None
    }
}
