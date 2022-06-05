use std::fmt::{ Display, Formatter, Result as FmtResult };

// This code is based on the code from the ast_math example, so check that out first.

// ------------------------------------------------------------------------------------------------
// AstNode
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum AstNode {
	Const  { val: f64 },
	Ident  { name: String },
	Negate { lhs: Box<AstNode> },
	Binary { op: BinOp, lhs: Box<AstNode>, rhs: Box<AstNode> },
	Call   { callee: Box<AstNode>, arg: Box<AstNode> },
}

impl Display for AstNode {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use AstNode::*;

		match self {
			Const  { val }          => write!(f, "{}", val),
			Ident  { name }         => write!(f, "{}", name),
			Negate { lhs }          => write!(f, "-({})", lhs),
			Binary { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
			Call   { callee, arg }  => write!(f, "({}({}))", callee, arg),
		}
	}
}

impl AstNode {
	pub fn num(val: f64) -> Box<AstNode> {
		return Box::new(AstNode::Const { val });
	}

	pub fn id(name: &str) -> Box<AstNode> {
		return Box::new(AstNode::Ident { name: name.into() });
	}

	pub fn neg(lhs: Box<AstNode>) -> Box<AstNode> {
		return Box::new(AstNode::Negate { lhs });
	}

	pub fn bin(lhs: Box<AstNode>, op: BinOp, rhs: Box<AstNode>) -> Box<AstNode> {
		return Box::new(AstNode::Binary { op, lhs, rhs });
	}

	pub fn add(lhs: Box<AstNode>, rhs: Box<AstNode>) -> Box<AstNode> {
		return Box::new(AstNode::Binary { op: BinOp::Add, lhs, rhs });
	}

	pub fn sub(lhs: Box<AstNode>, rhs: Box<AstNode>) -> Box<AstNode> {
		return Box::new(AstNode::Binary { op: BinOp::Sub, lhs, rhs });
	}

	pub fn mul(lhs: Box<AstNode>, rhs: Box<AstNode>) -> Box<AstNode> {
		return Box::new(AstNode::Binary { op: BinOp::Mul, lhs, rhs });
	}

	pub fn div(lhs: Box<AstNode>, rhs: Box<AstNode>) -> Box<AstNode> {
		return Box::new(AstNode::Binary { op: BinOp::Div, lhs, rhs });
	}

	// mod is a keyword in Rust
	pub fn mod_(lhs: Box<AstNode>, rhs: Box<AstNode>) -> Box<AstNode> {
		return Box::new(AstNode::Binary { op: BinOp::Mod, lhs, rhs });
	}

	pub fn call(callee: Box<AstNode>, arg: Box<AstNode>) -> Box<AstNode> {
		return Box::new(AstNode::Call { callee, arg });
	}
}

// ------------------------------------------------------------------------------------------------
// BinOp
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum BinOp {
	Add, Sub, Mul, Div, Mod
}

impl Display for BinOp {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use BinOp::*;
		match self {
			Add => write!(f, "+"),
			Sub => write!(f, "-"),
			Mul => write!(f, "*"),
			Div => write!(f, "/"),
			Mod => write!(f, "%"),
		}
	}
}