
use parsing_math::*;

fn main() {
	use TokenKind::*;

	// a + b + c
	parse_it(&vec![id("a"), Plus, id("b"), Plus, id("c")]);

	// a * b * c
	parse_it(&vec![id("a"), Times, id("b"), Times, id("c")]);

	// a * b + c
	parse_it(&vec![id("a"), Times, id("b"), Plus, id("c")]);

	// a + b * c (woah, check it out!)
	parse_it(&vec![id("a"), Plus, id("b"), Times, id("c")]);

	// 27 / 3 / 9
	parse_it(&vec![num(27), Divide, num(3), Divide, num(9)]);

	// -f(x)
	parse_it(&vec![Minus, id("f"), LParen, id("x"), RParen]);

	// f(x)(y)
	parse_it(&vec![id("f"), LParen, id("x"), RParen, LParen, id("y"), RParen]);

	// -f(x)(y)
	parse_it(&vec![Minus, id("f"), LParen, id("x"), RParen, LParen, id("y"), RParen]);

	// - - - x
	parse_it(&vec![Minus, Minus, Minus, id("x")]);

	// -3 * x + 5 / y - 10
	parse_it(&vec![Minus, num(3), Times, id("x"), Plus, num(5), Divide, id("y"), Minus, num(10)]);

	// x y
	parse_it(&vec![id("x"), id("y")]);

	// (x
	parse_it(&vec![LParen, id("x")]);

	// x + *
	parse_it(&vec![id("x"), Plus, Times]);
}

fn id(s: &str) -> TokenKind {
	return TokenKind::Id(s.into());
}

fn num(val: i32) -> TokenKind {
	return TokenKind::NumLit(val as f64);
}

fn parse_it(tokens: &[TokenKind]) {
	show_tokens(&tokens);

	match parse_exp(&tokens) {
		Ok(ast)  => println!("AST: {}", ast),
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