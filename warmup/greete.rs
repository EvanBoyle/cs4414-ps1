use std::{os, uint};

fn main() {
	let args: ~[~str] = os::args();
	println(fmt!("hello %s",args[1]));

}
