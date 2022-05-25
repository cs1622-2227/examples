// this derive(Debug) lets us print the tree with {:?} or {:#?}.
#[derive(Debug)]
struct Node<T> {
	#[allow(dead_code)] // shut up rust!
	value: T,
	// Option<> makes it nullable (can be None); Box<> makes it a reference.
	// It looks verbose, but it's rare to need this kind of type.
	left:  Option<Box<Node<T>>>,
	right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
	// A convenience constructor to make a new boxed node with no children.
	// Notice the use of Self to avoid having to type "Node<T>" everywhere.
	fn new(value: T) -> Box<Self> {
		return Box::new(Self {
			value,
			left: None,
			right: None,
		});
	}
}

fn main() {
	// Make a tree with 5 as the root, 2 as its left child, and 7 as its right child.
	let mut a = Node::new(5);
	a.left  = Some(Node::new(2));
	a.right = Some(Node::new(7));
	println!("{:#?}", a);
}