fn main() {
	/*
	Hello World
	*/
	println!("Hello World!\nI'm a Rustacean!");


	// format
	println!("{0}, this is {1} and {1}, this is {0}", "Deona", "Cathu");

	// special formating
	// Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    println!("Hello {:1$}!", "x", 5);

    assert_eq!(format!("Hello {:<5}!", "x"),  "Hello x    !");

    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");

    let pi = 3.141592;
    println!("Pi is roughly {:.2}", pi);
}