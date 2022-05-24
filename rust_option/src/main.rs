fn main() {
	unpack_option_1(None);
	unpack_option_1(Some(17));

	unpack_option_2(None);
	unpack_option_2(Some(34));

	str_find("hello, world", "ll");
	str_find("hello, world", "x");

	scream_if_none(Some(12));
	scream_if_none(None);

	use_unwrap(Some(75));
	// Uncomment this line for a crash!
	// use_unwrap(None);
}

fn unpack_option_1(x: Option<i32>) {
	// One way of "unpacking" the value of an option is to use a match.
	match x {
		None => println!("unpack_option_1 got None."),

		// remember 'value' here declares a new variable, set to the value that's inside 'x'.
		Some(value) => println!("unpack_option_1 got {}.", value),
	}
}

fn unpack_option_2(x: Option<i32>) {
	// Another way to unpack it is to use "if let." This looks weird as hell, but really
	// this is just like a match arm with only one pattern.
	// This is used a lot when you don't care about the "None" case, but you could have an "else"
	// here if you wanted to handle that too.
	if let Some(value) = x {
		println!("unpack_option_2 got {}.", value);
	}
}

fn str_find(s: &str, needle: &str) {
	// Many methods return an Option when they need to indicate that something succeeded (Some) or
	// failed (None), but when "failure" isn't a bug or anything, it's just normal.

	// In Java, if you search a string and it isn't found, it returns -1. But that's kinda weird,
	// and nothing prevents you from using that invalid position in subsequent code.

	// Rust's string find() method instead returns an Option, with None meaning "not found." This
	// forces you to check the return value and use it properly.
	match s.find(needle) {
		Some(position) => println!("'{}' is in '{}' at position {}.", needle, s, position),
		None           => println!("'{}' is not in '{}'.", needle, s),
	}
}

fn scream_if_none(x: Option<i32>) {
	// This is like doing "if(x == null)" in Java. (There is also an is_some() method, but usually
	// in that case you'd use "if let" to get the value out of the option anyway.)
	if x.is_none() {
		println!("scream_if_none got a None!! AAAAAAHHHHHH");
	}
}

fn use_unwrap(x: Option<i32>) {
	// If you KNOW that an Option is Some, you can use .unwrap() to get the value out of it.

	// But if you call .unwrap() on a None value, your program crashes! This is the closest thing
	// Rust has to a NullPointerException. For that reason, .unwrap() is kind of frowned upon in
	// cases where you are not absolutely 100% sure that the value is Some.
	println!("use_unwrap: x = {}", x.unwrap());
}