
use colored::Colorize;

use crate::ast::*;

// You can have structs with no fields in Rust. It seems weird, until it's useful.
pub struct ScopeVisitor;

impl ScopeVisitor {
	// Method to visit a Program AST node.
	pub fn visit_program(&self, prog: &Program) {
		println!();
		println!("note: {} words are symbols are being declared.", "red".red());
		println!("      {} words are symbols that were declared in pass 1 being revisited in pass 2.", "yellow".yellow());
		println!("      {} words are symbols being used (where name resolution would occur).", "green".green());
		println!();

		// pass 1: list all the global names. In the real namechecking, this is where these
		// names are inserted into the global scope, so that they can be used anywhere in
		// the program in pass 2.
		println!("{}", "PASS 1: list all globals. these would be inserted into the global scope.".cyan());
		println!("{}", "------------------------------------------------------------------------".cyan());
		println!();

		for d in &prog.decls {
			match d.as_ref() {
				Decl::Var(vd) => {
					println!("global variable decl: '{}' (node id: {})", vd.name.name.red(), vd.name.id);
				}

				Decl::Func(fd) => {
					println!("function decl: '{}' (node id: {})", fd.name.name.red(), fd.name.id);
				}
			}
		}

		// pass 2: visit the insides of the functions (and global var initializers). This is where
		// name resolution actually happens, as well as building the scope trees of the insides
		// of functions.
		println!();
		println!("{}", "PASS 2: visit function bodies.".cyan());
		println!("{}", "------------------------------------------------------------------------".cyan());
		println!();

		for d in &prog.decls {
			match d.as_ref() {
				Decl::Var(vd)  => self.visit_global_var_decl(vd),
				Decl::Func(fd) => self.visit_func_decl(fd),
			}
		}
	}

	// Method to visit a VarDecl, but only those that appear as global variables.
	// When this is called, the variable's name has already been "recorded" so we only
	// need to visit its initializer.
	pub fn visit_global_var_decl(&self, vd: &VarDecl) {
		println!("Visiting {}'s initializer...", vd.name.name.yellow());
		self.visit_exp(&vd.init, 1);
		println!();
	}

	// Method to visit a FuncDecl.
	pub fn visit_func_decl(&self, fd: &FuncDecl) {
		// Each function has its own scope, and these BEGIN/END SCOPE messages indicate
		// where we would create a new scope and where we would go back to the parent scope.
		println!("BEGIN SCOPE for function {} (node id: {})", fd.name.name.yellow(), fd.name.id);

		// Arguments are really local variables!
		for arg in &fd.args {
			println!("    argument decl: '{}' (node id: {})", arg.name.red(), arg.id);
		}

		// Then we visit the code inside this function.
		self.visit_stmt(&fd.code, 1);

		println!("END SCOPE for function {}", fd.name.name);
		println!();
	}

	// Method to visit an expression node. scope_depth is used to keep track of how many
	// scopes we are nested in, so we can print out the right number of indentations before
	// each line.
	pub fn visit_exp(&self, exp: &Box<Exp>, scope_depth: usize) {
		let indent = "    ".repeat(scope_depth);

		match &exp.kind {
			ExpKind::Id(ident) => {
				// vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
				// this right here is where NAME RESOLUTION would occur!
				println!("{}I see a use of '{}' at AST node {}!", indent, ident.name.green(), exp.id);
				// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
			}

			ExpKind::IntLit(..) => {
				// do nothing. ints aren't names, silly!
			}

			ExpKind::Call { callee, args } => {
				// we just recursively visit the callee and arguments.
				self.visit_exp(callee, scope_depth);

				for arg in args {
					self.visit_exp(arg, scope_depth);
				}
			}
		}
	}

	// Method to visit a statement node. for scope_depth, see the explanation on visit_exp.
	pub fn visit_stmt(&self, stmt: &Box<Stmt>, scope_depth: usize) {
		let indent = "    ".repeat(scope_depth);

		match &stmt.kind {
			StmtKind::Block(stmts) => {
				// Block statements are interesting because they introduce a new scope
				// nested within the current scope. You can see the BEGIN/END SCOPE messages
				// in the output for each block statement.
				println!("{}BEGIN SCOPE for block statement", indent);
				for s in stmts {
					// increment scope_depth for the recursive calls.
					self.visit_stmt(s, scope_depth + 1);
				}
				println!("{}END SCOPE for block statement", indent);
			}

			StmtKind::Exp(e) => {
				// Just visit the expression.
				self.visit_exp(e, scope_depth);
			}

			StmtKind::Assign { dst, src } => {
				// Just visit the two sides.
				self.visit_exp(dst, scope_depth);
				self.visit_exp(src, scope_depth);
			}

			StmtKind::Let(vd) => {
				// This is where we would insert the local variable symbol into the current scope.
				// Well, technically, we'd visit the initializer *first,* *then* insert it, so that
				// you can't do weird stuff like "let x = x;" and have the x on the right side of
				// the = refer to the x on the left side.
				println!("{}local variable decl: '{}' (node id: {})",
					indent, vd.name.name.red(), vd.name.id);
				println!("{}visiting its initializer...", indent);

				self.visit_exp(&vd.init, scope_depth + 1);
			}
		}
	}
}