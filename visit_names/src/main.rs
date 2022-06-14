
mod ast;
mod visit;

use ast::*;
use visit::*;

fn main() {
	// This is an AST built from scratch. The functions used to build it are declared after main.
	let prog = Program {
		decls: vec![
			// let glob1 = 10;
			vardecl("glob1", intlit(10)),

			// let glob2 = 20;
			vardecl("glob2", intlit(20)),

			// fn main() {
			funcdecl("main", vec![], vec![
				// let local1 = 30;
				letstmt("local1", intlit(30)),
				// foo(local1, glob3);
				callstmt("foo", vec![idexp("local1"), idexp("glob3")]),

				// {
				blockstmt(vec![
					// let local2 = local1;
					letstmt("local2", idexp("local1")),
				// }
				]),
			// }
			]),

			// fn foo(x, y) {
			funcdecl("foo", vec!["x", "y"], vec![
				// glob1 = x;
				assignstmt("glob1", "x"),
				// glob2 = y;
				assignstmt("glob2", "y"),
			// }
			]),

			// let glob3 = glob1;
			vardecl("glob3", idexp("glob1")),
		]
	};

	// uncomment this to see the whole ugly AST printed out. warning, it's kind of a lot.
	// println!("AST: {:#?}", prog);

	// Run the ScopeVisitor on the program!
	ScopeVisitor.visit_program(&prog);
}

// ------------------------------------------------------------------------------------
// AST-building helper functions
// ------------------------------------------------------------------------------------

fn vardecl(name: &str, init: Box<Exp>) -> Box<Decl> {
	return Decl::new_var(VarDecl {
		name: Ident::new(name),
		init,
	});
}

fn funcdecl(name: &str, args: Vec<&str>, code: Vec<Box<Stmt>>) -> Box<Decl> {
	return Decl::new_func(FuncDecl {
		name: Ident::new(name),
		args: args.iter().map(|arg| Ident::new(arg)).collect(),
		code: Stmt::new(StmtKind::Block(code)),
	});
}

fn intlit(value: i64) -> Box<Exp> {
	return Exp::new(ExpKind::IntLit(value));
}

fn idexp(name: &str) -> Box<Exp> {
	return Exp::new(ExpKind::Id(Ident::new(name)));
}

fn letstmt(name: &str, init: Box<Exp>) -> Box<Stmt> {
	return Stmt::new(StmtKind::Let(VarDecl {
		name: Ident::new(name),
		init,
	}));
}

fn callstmt(callee: &str, args: Vec<Box<Exp>>) -> Box<Stmt> {
	return Stmt::new(StmtKind::Exp(Exp::new(ExpKind::Call {
		callee: Exp::new(ExpKind::Id(Ident::new(callee))),
		args,
	})));
}

fn assignstmt(dst: &str, src: &str) -> Box<Stmt> {
	return Stmt::new(StmtKind::Assign {
		dst: Exp::new(ExpKind::Id(Ident::new(dst))),
		src: Exp::new(ExpKind::Id(Ident::new(src))),
	});
}

fn blockstmt(stmts: Vec<Box<Stmt>>) -> Box<Stmt> {
	return Stmt::new(StmtKind::Block(stmts));
}