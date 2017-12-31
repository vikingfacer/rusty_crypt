
mod left_shift_register;
use left_shift_register::left_shift_register::LeftShiftRegister;

use std::io::{self, Read};


fn main() {
	// let mut _tester = LeftShiftRegister{ _step : 3, buffer : 128};
	let mut input : Vec<u8> = Vec::new();
	let mut _lsr = LeftShiftRegister{_step : 2, buffer : 0b11100111};
	let mut _lsr2 = LeftShiftRegister{_step : 2, buffer : 0b11100111};
	let stdin = io::stdin();
	let mut handle = stdin.lock();

	handle.read_to_end(&mut input).ok();
	println!("{:?}", input);

	for byte in input.iter_mut(){
		*byte ^= _lsr.buffer;
		_lsr.generate(3);
	}
	println!("{:?}",input);

	for byte in input.iter_mut(){
		*byte ^= _lsr2.buffer;
		_lsr2.generate(3);
	}
	println!("{:?}",input);


}

