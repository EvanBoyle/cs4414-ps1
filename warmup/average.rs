use std::{os, float};

fn main() {
	let args: ~[~str] = os::args();
	let mut sum = 0.0;
	let mut count = 0.0;
	for args.slice(1, args.len()).iter().advance |s| {
		match float::from_str(*s) {
			Some(num)=>{
				sum += num;
				count+=1.0;
			}
			None => {
				println(fmt!("Bad input: %s", *s));
			}
		}
	
	}
	println(fmt!("Average: %f", sum/(count as float)));

}
