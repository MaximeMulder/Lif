mod class;
mod controls;
mod declaration;
mod function;
mod group;
mod literals;
mod operation_binary;
mod sequence;

use crate::element::Element;
use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::structures;
use self::operation_binary::operation_binary;
use self::sequence::sequence;

fn expression_1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	let functions: [&dyn Fn(&mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()>; 11] = [
		&literals::number,
		&literals::string,
		&literals::identifier,
		&declaration::declaration,
		&function::function,
		&class::class,
		&controls::r#continue,
		&controls::r#break,
		&controls::r#return,
		&structures::structure,
		&group::group,
	];

	for function in functions.iter() {
		if let Ok(node) = parser.safe(&|parser| function(parser)) {
			return Ok(node);
		}
	}

	return Err(());
}

fn expression_2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	let expression = expression_1(parser)?;
	return sequence(parser, expression);
}

const OPERATORS_BINARY_1: [&Element; 4] = [
	&elements::symbols::ASTERISK,
	&elements::symbols::SLASH,
	&elements::symbols::PERCENT,
	&elements::symbols::ASTERISK_D,
];

fn expression_3<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_1, &expression_2, &expression_3);
}

const OPERATORS_BINARY_2: [&Element; 2] = [&elements::symbols::PLUS, &elements::symbols::MINUS];

fn expression_4<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_2, &expression_3, &expression_4);
}

const OPERATORS_BINARY_3: [&Element; 4] = [
	&elements::symbols::GUILLEMET_L_D,
	&elements::symbols::GUILLEMET_R_D,
	&elements::symbols::GUILLEMET_L_T,
	&elements::symbols::GUILLEMET_R_T,
];

fn expression_5<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_3, &expression_4, &expression_5);
}

const OPERATORS_BINARY_4: [&Element; 4] = [
	&elements::symbols::GUILLEMET_L,
	&elements::symbols::GUILLEMET_R,
	&elements::symbols::GUILLEMET_L_EQ,
	&elements::symbols::GUILLEMET_R_EQ,
];

fn expression_6<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_4, &expression_5, &expression_6);
}

const OPERATORS_BINARY_5: [&Element; 2] = [
	&elements::symbols::EQUAL_D,
	&elements::symbols::EXCLAMATION_EQ,
];

fn expression_7<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_5, &expression_6, &expression_7);
}

const OPERATORS_BINARY_6: [&Element; 1] = [&elements::symbols::AMPERSAND];

fn expression_8<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_6, &expression_7, &expression_8);
}

const OPERATORS_BINARY_7: [&Element; 1] = [&elements::symbols::CARET];

fn expression_9<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_7, &expression_8, &expression_9);
}

const OPERATORS_BINARY_8: [&Element; 1] = [&elements::symbols::PIPE];

fn expression_10<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_8, &expression_9, &expression_10);
}

const OPERATORS_BINARY_9: [&Element; 1] = [&elements::symbols::AMPERSAND_D];

fn expression_11<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_9, &expression_10, &expression_11);
}

const OPERATORS_BINARY_10: [&Element; 1] = [&elements::symbols::PIPE_D];

fn expression_12<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_10, &expression_11, &expression_12);
}

const OPERATORS_BINARY_11: [&Element; 2] = [&elements::symbols::DOT_D, &elements::symbols::DOT_D_EQ];

pub fn expression_13<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_11, &expression_12, &expression_13);
}

const OPERATORS_BINARY_12: [&Element; 16] = [
	&elements::symbols::EQUAL,
	&elements::symbols::PLUS_EQ,
	&elements::symbols::MINUS_EQ,
	&elements::symbols::ASTERISK_EQ,
	&elements::symbols::SLASH_EQ,
	&elements::symbols::PERCENT_EQ,
	&elements::symbols::ASTERISK_D_EQ,
	&elements::symbols::GUILLEMET_L_D_EQ,
	&elements::symbols::GUILLEMET_R_D_EQ,
	&elements::symbols::GUILLEMET_L_T_EQ,
	&elements::symbols::GUILLEMET_R_T_EQ,
	&elements::symbols::AMPERSAND_EQ,
	&elements::symbols::CARET_EQ,
	&elements::symbols::PIPE_EQ,
	&elements::symbols::AMPERSAND_D_EQ,
	&elements::symbols::PIPE_D_EQ,
];

fn expression_14<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_12, &expression_13, &expression_14);
}

pub fn expression<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return expression_14(parser);
}
