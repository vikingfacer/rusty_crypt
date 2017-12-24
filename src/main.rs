
use std::io::{self, Read};

// this should be a struct 
// with the or_target and item be itterated on

#[derive(Debug)]
struct LeftShiftRegister {
	_step : u8,
    buffer : u8,
}


fn main() {
	let mut _tester = LeftShiftRegister{ _step : 3, buffer : 128};
	println!("{:?}", _tester.buffer);
	_tester.step();
	println!("{:?}", _tester.buffer);
	loop {
		_tester.generate(2);
		println!("{:?}", _tester.buffer);
	}


}

// fn return_bool(nth : u8) bool{

// }
impl LeftShiftRegister {

    // this will be a function to preform a simple step
pub fn step(&mut self) {
	let or_seed : u8 = 1 << self._step;
	let seed_bool : bool = (or_seed & self.buffer) == or_seed;
	let first_bool : bool = (0x80 & self.buffer) == 0x80;
	let mut return_bit :u8 = 0;

	self.buffer = self.buffer << 1;

	if seed_bool ^ first_bool{
		return_bit = 1
	}
    self.buffer = self.buffer | return_bit;
}
	// this will be a function to do a generation of step 
fn generate(&mut self, n : u8) {
	
	let mut i = 0;
	
	while i < n{
		self.step();
		i += 1;
	 } 
}

}














