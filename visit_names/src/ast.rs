
// This file contains a minimal AST with a Program root node, Decls, Stmts, Exps, and Idents.
// It mirrors the Truss AST from the projects, but is much simpler for the purpose of demonstartion.

use std::sync::atomic::{ AtomicUsize, Ordering };

// ------------------------------------------------------------------------------------
// Node IDs
// ------------------------------------------------------------------------------------

// This looks scary, but it's a global variable that we increment.
// It has to be done this way with AtomicUsize because globals could be accessed
// from multiple threads, so Rust forces us to be safe here.
static NODE_ID: AtomicUsize = AtomicUsize::new(1);

// Returns a new, unique node ID every time it's called.
pub fn new_node_id() -> usize {
	return NODE_ID.fetch_add(1, Ordering::SeqCst);
}

// ------------------------------------------------------------------------------------
// Ident
// ------------------------------------------------------------------------------------

// Idents are used a lot in the namechecking. They have a node id field, so that they
// can be referred to by the symbol tables, decl map, and use map.
#[derive(Debug)]
pub struct Ident {
	pub id:   usize,
	pub name: String,
}

impl Ident {
	pub fn new(name: &str) -> Self {
		return Self { id: new_node_id(), name: name.into() };
	}
}

// ------------------------------------------------------------------------------------
// Program
// ------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Program {
	pub decls: Vec<Box<Decl>>,
}

// ------------------------------------------------------------------------------------
// Decl
// ------------------------------------------------------------------------------------

// We don't need a separate node ID for the VarDecl because its name is an Ident and
// already has a node ID.
#[derive(Debug)]
pub struct VarDecl {
	pub name: Ident,
	pub init: Box<Exp>
}

// Same for FuncDecl.
#[derive(Debug)]
pub struct FuncDecl {
	pub name: Ident,
	pub args: Vec<Ident>,
	pub code: Box<Stmt>
}

#[derive(Debug)]
pub enum Decl {
	Var(VarDecl),
	Func(FuncDecl),
}

impl Decl {
	pub fn new_var(vd: VarDecl) -> Box<Self> {
		return Box::new(Self::Var(vd));
	}

	pub fn new_func(fd: FuncDecl) -> Box<Self> {
		return Box::new(Self::Func(fd));
	}
}

// ------------------------------------------------------------------------------------
// Stmt
// ------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum StmtKind {
	Block  (Vec<Box<Stmt>>),
	Exp    (Box<Exp>),
	Assign { dst: Box<Exp>, src: Box<Exp> },
	Let    (VarDecl),
}

#[derive(Debug)]
pub struct Stmt {
	pub id: usize,
	pub kind: StmtKind,
}

impl Stmt {
	pub fn new(kind: StmtKind) -> Box<Self> {
		return Box::new(Self { id: new_node_id(), kind });
	}
}

// ------------------------------------------------------------------------------------
// Exp
// ------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum ExpKind {
	Id     (Ident),
	IntLit (i64),
	Call   { callee: Box<Exp>, args: Vec<Box<Exp>> },
}

#[derive(Debug)]
pub struct Exp {
	pub id:   usize,
	pub kind: ExpKind,
}

impl Exp {
	pub fn new(kind: ExpKind) -> Box<Self> {
		return Box::new(Self { id: new_node_id(), kind });
	}
}