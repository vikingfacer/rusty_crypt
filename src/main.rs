
mod left_shift_register;
use left_shift_register::left_shift_register::LeftShiftRegister;

use std::io::{self, Read};


fn main() {
	// let mut _tester = LeftShiftRegister{ _step : 3, buffer : 128};
	let mut input : Vec<u8> = Vec::new();
	let mut lsr = LeftShiftRegister{_step : 4, buffer : 8};
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	handle.read_to_end(&mut input).ok();
	println!("{:?}", input);

	for byte in input.iter_mut(){
		*byte = *byte ^ lsr.buffer;
		lsr.generate(9);
	}
	println!("{:?}", input);
	for byte in input.iter_mut(){
		*byte = *byte ^ lsr.buffer;
		lsr.generate(9);
	}
	println!("{:?}", input);

}

