use std::{os, uint};

fn main() {
	let args: ~[~str] = os::args();
	let mut count = 1;
	while count < args.len(){
		print(fmt!("%s ", args[count]));
		count+=1;

	}
	

}
