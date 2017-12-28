
pub mod left_shift_register{
#[derive(Debug, Clone)]
pub struct LeftShiftRegister {
	pub _step : u8,
    pub buffer : u8,
}
impl LeftShiftRegister {

    // this will be a function to preform a simple step
pub fn step(&mut self) -> u8{
	let or_seed : u8 = 1 << self._step;
	let seed_bool : bool = (or_seed & self.buffer) == or_seed;
	let first_bool : bool = (0x80 & self.buffer) == 0x80;
	let mut return_bit :u8 = 0;

	self.buffer = self.buffer << 1;

	if seed_bool ^ first_bool{
		return_bit = 1
	}
    self.buffer |= return_bit;
    return_bit
}
	// this will be a function to do a generation of step 
pub fn generate(&mut self, n : u8) -> u8 {
	
	let mut i = 1;
	while i < n{
		self.buffer = self.buffer << 1;
		self.buffer |= self.step();
		i += 1;
	 } 
	 self.buffer
}
}
}