use crate::element::Element;
use crate::parser2::Parser;
use crate::node::Node;

pub trait Filter<'a> {
	fn filter<'b>(&self, parser: &mut Parser<'a, 'b, '_>, nodes: Vec<Node<'a, 'b>>) -> Option<Vec<Node<'a, 'b>>>;
}

pub struct FilterList {
	filters: Vec<usize>,
}

impl FilterList {
	pub fn new(filters: Vec<usize>) -> Self {
		return Self {
			filters,
		};
	}
}

impl<'a> Filter<'a> for FilterList {
	fn filter<'b>(&self, parser: &mut Parser<'a, 'b, '_>, mut nodes: Vec<Node<'a, 'b>>) -> Option<Vec<Node<'a, 'b>>> {
		for filter in self.filters.iter().rev() {
			if let Some(others) = parser.filter(*filter, nodes) {
				nodes = others;
			} else {
				return None;
			}
		}

		return Some(nodes);
	}
}

pub struct FilterExtension {
	rule: usize,
	filter: usize,
}

impl FilterExtension {
	pub fn new(rule: usize, filter: usize) -> Self {
		return Self {
			rule,
			filter,
		};
	}
}

impl<'a> Filter<'a> for FilterExtension {
	fn filter<'b>(&self, parser: &mut Parser<'a, 'b, '_>, mut nodes: Vec<Node<'a, 'b>>) -> Option<Vec<Node<'a, 'b>>> {
		if let Some(children) = parser.rule(self.rule) {
			nodes.extend(children);
			return parser.filter(self.filter, nodes);
		}

		return Some(nodes);
	}
}

pub struct FilterElement<'a> {
	element: &'a Element,
}

impl<'a> FilterElement<'a> {
	pub fn new(element: &'a Element) -> Self {
		return Self {
			element,
		};
	}
}

impl<'a> Filter<'a> for FilterElement<'a> {
	fn filter<'b>(&self, parser: &mut Parser<'a, 'b, '_>, nodes: Vec<Node<'a, 'b>>) -> Option<Vec<Node<'a, 'b>>> {
		return Some(vec![Node::new_production(self.element, nodes)]);
	}
}

pub struct FilterCustom {
	callback: &'static dyn for<'a, 'b> Fn(&mut Parser<'a, 'b, '_>, Vec<Node<'a, 'b>>) -> Option<Vec<Node<'a, 'b>>>,
}

impl FilterCustom {
	pub fn new(callback: &'static dyn for<'a, 'b> Fn(&mut Parser<'a, 'b, '_>, Vec<Node<'a, 'b>>) -> Option<Vec<Node<'a, 'b>>>) -> Self {
		return Self {
			callback,
		};
	}
}

impl<'a> Filter<'a> for FilterCustom {
	fn filter<'b>(&self, parser: &mut Parser<'a, 'b, '_>, nodes: Vec<Node<'a, 'b>>) -> Option<Vec<Node<'a, 'b>>> {
		return (self.callback)(parser, nodes);
	}
}
