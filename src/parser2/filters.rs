use crate::element::Element;
use crate::parser2::Parser;
use crate::node::Node;

pub trait Filter<'a> {
	fn filter<'b>(&self, parser: &mut Parser<'a, 'b, '_>, nodes: Vec<Node<'a, 'b>>) -> Vec<Node<'a, 'b>>;
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
	fn filter<'b>(&self, parser: &mut Parser<'a, 'b, '_>, mut nodes: Vec<Node<'a, 'b>>) -> Vec<Node<'a, 'b>> {
		for filter in self.filters.iter() {
			nodes = parser.filter(*filter, nodes);
		}

		return nodes;
	}
}

pub struct FilterRecurse {
	rule: usize,
	filter: usize,
}

impl FilterRecurse {
	pub fn new(rule: usize, filter: usize) -> Self {
		return Self {
			rule,
			filter,
		};
	}
}

impl<'a> Filter<'a> for FilterRecurse {
	fn filter<'b>(&self, parser: &mut Parser<'a, 'b, '_>, mut nodes: Vec<Node<'a, 'b>>) -> Vec<Node<'a, 'b>> {
		while let Some(children) = parser.rule(self.rule) {
			nodes.extend(children);
			nodes = parser.filter(self.filter, nodes);
		}

		return nodes;
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
	fn filter<'b>(&self, parser: &mut Parser<'a, 'b, '_>, nodes: Vec<Node<'a, 'b>>) -> Vec<Node<'a, 'b>> {
		return vec![Node::new_production(self.element, nodes)];
	}
}
