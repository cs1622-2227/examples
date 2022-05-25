use std::fmt::{ Display, Formatter, Result as FmtResult };

// ------------------------------------------------------------------------------------------------
// Our little AST type
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum AstNode {
	Const  { val: f64 },
	Negate { lhs: Box<AstNode> },
	Binary { op: BinOp, lhs: Box<AstNode>, rhs: Box<AstNode> },
}

// Display is a *trait* - similar to a Java interface.
// Display is like the human-friendly version of Debug, meant for printing
// things out nicely. Implementing Display also gives us a .to_string() method
// for free.
impl Display for AstNode {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use AstNode::*;

		// write!() recursively calls the fmt() methods of the children.
		// this method *works* but it puts a bunch of extra () in the output.
		// to fix that, we'd have to know about the precedences of the operators,
		// but we won't talk about that for another lecture or two...
		match self {
			Const  { val }          => write!(f, "{}", val),
			Negate { lhs }          => write!(f, "-({})", lhs),
			Binary { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
		}
	}
}

impl AstNode {
	// Several constructors here to simplify building ASTs.

	// can't name this 'const' cause that's a keyword in Rust!
	pub fn num(val: f64) -> Box<AstNode> {
		return Box::new(AstNode::Const { val });
	}

	pub fn neg(lhs: Box<AstNode>) -> Box<AstNode> {
		return Box::new(AstNode::Negate { lhs });
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

	// This method evaluates the AST! neat!
	pub fn eval(&self) -> f64 {
		use AstNode::*;

		match self {
			// the *val here is because when you match on &self, all the matched
			// pattern variables are references; so it's a &f64. *val gives me the
			// f64 that it points to.
			Const  { val }          => return *val,
			Negate { lhs }          => return -lhs.eval(),
			Binary { op, lhs, rhs } => return op.eval(lhs.eval(), rhs.eval()),
		}
	}

	pub fn recip(&self) -> Box<AstNode> {
		use AstNode::*;

		match self {
			// special case for division. notice the fun syntax here for matching on a specific
			// value for op! any Binary {} node with op other than BinOp::Div will fail to match
			// this pattern and go to the default case instead.
			Binary { op: BinOp::Div, lhs, rhs } => {
				// by doing #[derive(Clone)] on AstNode, we get the .clone() method.
				// .clone() makes a new copy of the given value; it is also recursive.
				// so this will clone the entire subtrees giving us new identical-looking ones.
				return AstNode::div(rhs.clone(), lhs.clone());
			}

			_ => {
				// for anything else, we create 1 / input
				return AstNode::div(AstNode::num(1.), Box::new(self.clone()));
			}
		}
	}
}

// ------------------------------------------------------------------------------------------------
// The kinds of binary (two-operand) operators
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum BinOp {
	Add, Sub, Mul, Div
}

impl Display for BinOp {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use BinOp::*;
		match self {
			Add => write!(f, "+"),
			Sub => write!(f, "-"),
			Mul => write!(f, "*"),
			Div => write!(f, "/"),
		}
	}
}

impl BinOp {
	pub fn eval(&self, lhs: f64, rhs: f64) -> f64 {
		use BinOp::*;

		match self {
			Add => return lhs + rhs,
			Sub => return lhs - rhs,
			Mul => return lhs * rhs,
			Div => return lhs / rhs,
		}
	}
}