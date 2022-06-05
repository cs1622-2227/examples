
use std::fmt::{ Display, Formatter, Result as FmtResult };

// this line says that the "ast" module *exists*...
mod ast;

// and this imports all the stuff from it, *and* re-exports it so anyone that
// uses this crate sees all those things as well.
pub use crate::ast::*;

// ------------------------------------------------------------------------------------------------
// TokenKind
// ------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
	Eof,
	LParen,
	RParen,
	Plus,
	Minus,
	Times,
	Divide,
	Modulo,
	Id(String),
	NumLit(f64),
}

impl Display for TokenKind {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use TokenKind::*;

		match self {
			Eof       => write!(f, ""),
			LParen    => write!(f, "("),
			RParen    => write!(f, ")"),
			Plus      => write!(f, "+"),
			Minus     => write!(f, "-"),
			Times     => write!(f, "*"),
			Divide    => write!(f, "/"),
			Modulo    => write!(f, "%"),
			Id(id)    => write!(f, "{}", id),
			NumLit(i) => write!(f, "{}", i),
		}
	}
}

// ------------------------------------------------------------------------------------------------
// Precedence
// ------------------------------------------------------------------------------------------------

// PartialOrd/Ord give us comparison operators (<, >, <=, >=)
// The values in such an enum increase in value, so e.g. Add < Mul.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Precedence {
	// VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV
	// NOTE: because of the way that Ord works, we list the precedences
	// from LOWEST to HIGHEST. This is the opposite order from how it's
	// shown on the slides!!
	// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
	None, // A special value lower than any real precedence.

	Add,  // + and -
	Mul,  // *, /, and %

	// we don't need to list unary operators here, because they
	// are handled separately from binary operators.
}

impl Precedence {
	// This is how you put a constant "inside" a type, so this can be
	// accessed as `Precedence::MIN` elsewhere.
	const MIN: Precedence = Precedence::Add;

	// a more english-y way of testing if self >= other.
	fn is_at_least(&self, other: Precedence) -> bool {
		return *self >= other;
	}

	// same but for self > other.
	fn is_higher_than(&self, other: Precedence) -> bool {
		return *self > other;
	}
}

impl TokenKind {
	// the Precedence of this token (or Precedence::None if it's not an operator).
	fn precedence(&self) -> Precedence {
		use TokenKind::*;

		match self {
			// wait -- isn't the Minus token used for both subtraction AND negation?
			// yes, but due to the way the algorithm works, this method will only be
			// called in cases where it's unambiguously being used as a subtraction.

			Plus | Minus            => return Precedence::Add,
			Times | Divide | Modulo => return Precedence::Mul,

			// all other tokens have no precedence. this is how the expression
			// parser knows when to stop parsing.
			_ => return Precedence::None,
		}
	}

	// returns the BinOp enumeration value this token corresponds to. panics if it's
	// not an operator token. this just simplifies the code in parse_binops.
	fn to_binop(&self) -> BinOp {
		use TokenKind::*;

		match self {
			Plus   => return BinOp::Add,
			Minus  => return BinOp::Sub,
			Times  => return BinOp::Mul,
			Divide => return BinOp::Div,
			Modulo => return BinOp::Mod,
			_      => panic!("to_binop() called on a {:?} token", self),
		}
	}
}

// ------------------------------------------------------------------------------------------------
// The bottom-up expression parser
// ------------------------------------------------------------------------------------------------

// I'm keeping the code smaller by having the parser give a String for errors;
// this is bad design, don't follow my lead here lolol
type ParseResult = Result<Box<AstNode>, String>;

pub fn parse_exp(tokens: &[TokenKind]) -> ParseResult {
	let mut p = Parser::new(tokens);
	let ret = p.parse_exp()?;
	p.expect_eof()?;
	return Ok(ret);
}

struct Parser<'t> {
	tokens: &'t [TokenKind],
	pos:    usize,
}

impl<'t> Parser<'t> {
	fn new(tokens: &'t [TokenKind]) -> Self {
		return Parser { tokens, pos: 0 };
	}

	fn next(&mut self) {
		assert!(self.pos < self.tokens.len());
		self.pos += 1;
	}

	fn cur(&self) -> TokenKind {
		if self.pos < self.tokens.len() {
			return self.tokens[self.pos].clone();
		} else {
			return TokenKind::Eof;
		}
	}

	// Exp: Term (BinOp Term)*
	fn parse_exp(&mut self) -> ParseResult {
		// this line just does the first Term in the rule,
		let lhs = self.parse_term()?;

		// and this does the (BinOp Term)*.
		return self.parse_binops(lhs, Precedence::MIN);
	}

	// what's really cool about this algorithm is that we can add more operators,
	// change precedence levels etc. and this code doesn't change at all!
	fn parse_binops(&mut self, mut lhs: Box<AstNode>, min_prec: Precedence) -> ParseResult {
		// for tokens which are binary operators, .precedence() returns their precedence.
		// so this loop is indirectly saying, "while we are looking at a binary operator."
		// for tokens that aren't operators, .precedence() will return Precedence::None,
		// which is always less than min_prec, causing the loop to terminate.
		while self.cur().precedence().is_at_least(min_prec) {
			let op = self.cur();

			// parse the rhs, but we don't actually know if it's *our* rhs, or the next
			// operator's lhs!
			self.next();
			let mut rhs = self.parse_term()?;

			// this is a 'while' instead of an 'if', because there could be a decreasing
			// chain of higher-precedence operators here. it can't happen with our piddly
			// two levels of precedence, but it can happen in more complex grammars, like
			// a < b ** c * d + e, which should parse as (a < (((b ** c) * d) + e)).
			// ** is higher than * is higher than + is higher than <.
			while self.cur().precedence().is_higher_than(op.precedence()) {
				rhs = self.parse_binops(rhs, self.cur().precedence())?;
			}

			// glob the lhs and rhs together into an AST node!
			lhs = AstNode::bin(lhs, op.to_binop(), rhs);
		}

		// when done, the lhs variable contains the parsed expression tree.
		return Ok(lhs);
	}

	// Term: UnaryOp* PrimaryExp PostfixOp*
	fn parse_term(&mut self) -> ParseResult {
		// this match is for unary operators. there's only one in this language.
		match self.cur() {
			TokenKind::Minus => {
				self.next();
				// negation is right-associative, so we have to recurse to get the operand.
				// recursion is a Spicy Loop, so this handles UnaryOp*.
				let operand = self.parse_term()?;
				return Ok(AstNode::neg(operand));
			}

			_ => {
				// PrimaryExp
				let pri = self.parse_primary()?;
				// PostfixOp*
				return self.parse_postfix(pri);
			}
		}
	}

	// PrimaryExp: IdExp | NumExp | ParenExp
	fn parse_primary(&mut self) -> ParseResult {
		match self.cur() {
			// IdExp: <TokenKind::Id>
			TokenKind::Id(name) => {
				self.next();
				return Ok(AstNode::id(&name));
			}

			// NumExp: <TokenKind::NumLit>
			TokenKind::NumLit(val) => {
				self.next();
				return Ok(AstNode::num(val));
			}

			// ParenExp: '(' Exp ')'
			TokenKind::LParen => {
				self.next();
				let ret = self.parse_exp()?;
				self.expect_rparen()?;
				return Ok(ret);
			}

			t => return Err(format!(
				"expected an identifier, number, or parenthesized expression, not '{}'", t
			))
		}
	}

	// Term: UnaryOp* PrimaryExp PostfixOp*
	// PostfixOp: CallOp
	fn parse_postfix(&mut self, mut lhs: Box<AstNode>) -> ParseResult {
		// this loop implements the whole PostfixOp* part of the Term rule.
		loop {
			match self.cur() {
				// this language only has function calls with exactly one argument.
				// CallOp: '(' Exp ')'
				TokenKind::LParen => {
					self.next();
					let arg = self.parse_exp()?;
					self.expect_rparen()?;
					lhs = AstNode::call(lhs, arg);
				}

				// this is not an error case; there just might not be any postfix
				// operator here. this breaks out of the loop; you cannot break
				// out of a match in Rust.
				_ => break,
			}
		}

		return Ok(lhs);
	}

	fn expect_rparen(&mut self) -> Result<(), String> {
		match self.cur() {
			TokenKind::RParen => { self.next(); return Ok(()); }
			_ => return Err("expected a right parenthesis".into()),
		}
	}

	fn expect_eof(&mut self) -> Result<(), String> {
		match self.cur() {
			TokenKind::Eof => return Ok(()),
			_ => return Err("expected eof (there's extra stuff after the expression)".into()),
		}
	}
}