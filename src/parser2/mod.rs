#![allow(unused_variables)]
#![allow(dead_code)]

pub mod descent;
pub mod ascent;
pub mod nodes;
pub mod arena;

use crate::node::Node;
use arena::Arena;
use descent::Descent;
use ascent::Ascent;

pub struct Parser<'a, 'b, 'c> {
	pub tokens: &'c Vec<Node<'a, 'b>>,
	descents: &'c Arena<dyn Descent<'a> + 'c>,
	ascents: &'c Arena<dyn Ascent<'a> + 'c>,
	cursor: usize,
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
	fn new(tokens: &'c Vec<Node<'a, 'b>>, descents: &'c Arena<dyn Descent<'a> + 'c>, ascents: &'c Arena<dyn Ascent<'a> + 'c>) -> Self {
		return Self {
			tokens,
			descents,
			ascents,
			cursor: 0,
		};
	}

	fn done(&self) -> bool {
		return self.cursor == self.tokens.len();
	}

	fn next(&mut self) -> Option<Node<'a, 'b>> {
		let option = self.tokens.get(self.cursor);
		if let Some(token) = option {
			self.cursor += 1;
			return Some(token.clone());
		}

		return None;
	}

	fn descent(&mut self, index: usize) -> Option<Vec<Node<'a, 'b>>> {
		let cursor = self.cursor;
		let nodes = self.descents.get(index).descent(self);
		if nodes.is_none() {
			self.cursor = cursor;
		}

		return nodes;
	}

	fn descent_predicate(&mut self, index: usize) -> bool {
		let cursor = self.cursor;
		let nodes = self.descents.get(index).descent(self);
		self.cursor = cursor;
		return nodes.is_some();
	}

	fn ascent(&mut self, index: usize, nodes: Vec<Node<'a, 'b>>) -> Option<Vec<Node<'a, 'b>>> {
		let cursor = self.cursor;
		let nodes = self.ascents.get(index).ascent(self, nodes);
		if nodes.is_none() {
			self.cursor = cursor;
		}

		return nodes;
	}

	fn ascent_predicate(&mut self, index: usize, nodes: Vec<Node<'a, 'b>>) -> bool {
		let cursor = self.cursor;
		let nodes = self.ascents.get(index).ascent(self, nodes);
		self.cursor = cursor;
		return nodes.is_some();
	}
}
