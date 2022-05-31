
use parsing_lisp::*;

fn main() {
	use TokenKind::*;

	// The simplest expression.
	parse_it(&vec![
		// test
		id("test")
	]);

	// Any number of expressions can come between parens.
	parse_it(&vec![
		// (1 2 3 4 5)
		LParen, IntLit(1), IntLit(2), IntLit(3), IntLit(4), IntLit(5), RParen
	]);

	// Nested expressions.
	parse_it(&vec![
		// (add 3 (sub x y))
		LParen, id("add"), IntLit(3), LParen, id("sub"), id("x"), id("y"), RParen, RParen
	]);

	// This demonstrates why the Eof token exists - to avoid having extra stuff at the
	// end of the input that isn't used.
	parse_it(&vec![
		// (extra stuff after this) oops
		LParen, id("extra"), id("stuff"), id("after"), id("this"), RParen, id("oops")
	]);

	// Another kind of parse error.
	parse_it(&vec![
		// (hi
		LParen, id("hi"),
	]);
}

// shorthand.
fn id(s: &str) -> TokenKind {
	TokenKind::Id(s.into())
}

// &[TokenKind] is to Vec<TokenKind> as &str is to String.
// &[TokenKind] is a slice type, meaning this function can accept any type which can be
// sliced (including Vecs and arrays).
fn parse_it(tokens: &[TokenKind]) {
	show_tokens(&tokens);

	match Parser::parse(&tokens) {
		Ok(ast)  => println!("AST: {:#?}", ast),
		Err(err) => println!("parse error: {}", err),
	}

	println!();
}

fn show_tokens(tokens: &[TokenKind]) {
	print!("Input tokens: ");

	for t in tokens {
		print!("{} ", t);
	}

	println!();
}