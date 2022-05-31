
use std::fmt::{ Debug, Display, Formatter, Result as FmtResult };

// ------------------------------------------------------------------------------------------------
// TokenKind type
// ------------------------------------------------------------------------------------------------

/*
These tokens would be produced by a lexer... IF WE HAD ONE
Well, another example has one. Maybe you can put the two examples together!
*/
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
	Eof,
	LParen,
	RParen,
	Id(String),
	IntLit(i64),
}

impl Display for TokenKind {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use TokenKind::*;

		match self {
			Eof       => write!(f, ""),
			LParen    => write!(f, "("),
			RParen    => write!(f, ")"),
			Id(id)    => write!(f, "{}", id),
			IntLit(i) => write!(f, "{}", i),
		}
	}
}

// ------------------------------------------------------------------------------------------------
// AST type
// ------------------------------------------------------------------------------------------------

/*
Here is the full syntactic grammar for this language. Remember that the "alphabet" (set of symbols
that this grammar operates on) is *tokens* produced by the lexer. This is what the rules for Id,
Num, and Eof mean. '(' and ')' are also the LParen and RParen tokens, but to make the rules more
readable, we just write them explicitly.

	Program:  Exp Eof
	Exp:      Id | Num | ParenExp
	ParenExp: '(' Exp+ ')'

	Id:  <Id token from lexing phase>
	Num: <IntLit token from lexing phase>
	Eof: <'<eof>' token from lexing phase>
*/

#[derive(Clone)]
pub enum Exp {
	Id(String),
	Num(i64),
	Parens(Vec<Box<Exp>>),
}

impl Exp {
	pub fn new_id(s: &str) -> Box<Self> {
		return Box::new(Exp::Id(s.into()));
	}

	pub fn new_num(i: i64) -> Box<Self> {
		return Box::new(Exp::Num(i));
	}

	pub fn new_parens(exps: Vec<Box<Exp>>) -> Box<Self> {
		return Box::new(Exp::Parens(exps));
	}
}

// You can write your own implementations of Debug too, instead of #[derive]ing them.
// I'm doing this to make the output a little more compact than what #[derive] gives me.
impl Debug for Exp {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use Exp::*;

		match self {
			Id(id) => write!(f, "Id({})", id),
			Num(i) => write!(f, "Num({})", i),
			Parens(exps) => {
				write!(f, "Parens")?;
				f.debug_list().entries(exps.iter()).finish()
			}
		}
	}
}

// ------------------------------------------------------------------------------------------------
// ParseError type
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum ParseError {
	ExpectedExpression,
	ExpectedLParen,
	ExpectedRParen,
	ExpectedEof,
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use ParseError::*;

		match self {
			ExpectedExpression => write!(f, "expected an expression"),
			ExpectedLParen     => write!(f, "expected '(' to start an expression"),
			ExpectedRParen     => write!(f, "expected ')' to end an expression"),
			ExpectedEof        => write!(f, "expected end-of-file token at end of input"),
		}
	}
}

impl std::error::Error for ParseError {}

// ------------------------------------------------------------------------------------------------
// The recursive descent parser
// ------------------------------------------------------------------------------------------------

// `type` lets us make a shorthand alias for a longer type. Now wherever I write ParseResult,
// it's the same as writing Result<Box<Exp>, ParseError>.
type ParseResult = Result<Box<Exp>, ParseError>;

pub struct Parser<'t> {
	tokens: &'t [TokenKind],
	pos:    usize,
}

impl<'t> Parser<'t> {
	pub fn parse(tokens: &'t [TokenKind]) -> ParseResult {
		let mut p = Parser::new(tokens);
		return p.parse_program();
	}

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

	// Program: Exp Eof
	fn parse_program(&mut self) -> ParseResult {
		let ret = self.parse_exp()?;
		self.expect_eof()?;
		return Ok(ret);
	}

	// Exp: Id | Num | ParenExp
	fn parse_exp(&mut self) -> ParseResult {
		use TokenKind::*;

		match self.cur() {
			Id(s)     => { self.next(); return Ok(Exp::new_id(&s)); }
			IntLit(i) => { self.next(); return Ok(Exp::new_num(i)); }
			LParen    => return self.parse_paren_exp(),
			_         => return Err(ParseError::ExpectedExpression),
		}
	}

	// ParenExp: '(' Exp+ ')'
	fn parse_paren_exp(&mut self) -> ParseResult {
		// Note the use of ? here. It means, "if expect_lparen() returned an error, then return
		// that error; otherwise, carry on as usual."
		self.expect_lparen()?;

		// These next few lines implement the "Exp+" part of the rule. See how we parse 1,
		// then loop to parse more if there are any. The loop condition of + and * rules can
		// be a little subtle sometimes, but here it's straightforward.
		let mut exps = Vec::new();
		exps.push(self.parse_exp()?); // you can use ? in the middle of a line too.

		while self.cur() != TokenKind::RParen {
			exps.push(self.parse_exp()?); // another possible failure point...
		}

		self.expect_rparen()?; // and the last possible failure point.

		// and if we made it to the end of this method, everything is Ok()!
		return Ok(Exp::new_parens(exps));
	}

	// () is Rust's void.
	// This return type says "returns nothing on success, or ParseError on failure"
	fn expect_lparen(&mut self) -> Result<(), ParseError> {
		// Ok(()) is how you say "everything's Ok, but I don't have a value to return"
		match self.cur() {
			TokenKind::LParen => { self.next(); return Ok(()); }
			_                 => return Err(ParseError::ExpectedLParen),
		}
	}

	fn expect_rparen(&mut self) -> Result<(), ParseError> {
		match self.cur() {
			TokenKind::RParen => { self.next(); return Ok(()); }
			_                 => return Err(ParseError::ExpectedRParen),
		}
	}

	fn expect_eof(&mut self) -> Result<(), ParseError> {
		match self.cur() {
			TokenKind::Eof => return Ok(()),
			_              => return Err(ParseError::ExpectedEof),
		}
	}
}