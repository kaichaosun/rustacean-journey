use futures::executor::block_on;

fn main() {
	let future = say_hello();

	// // `block_on` blocks the current thread until the provided future has run to completion.
	block_on(future);
}


async fn say_hello() {
	println!("hello world");
}