
use std::io::{self, Read};

fn main() {

	let mut buffer = String::new();
	let mut cryted : Vec<u8> = Vec::new();

	let stdin = io::stdin();
	let mut handle = stdin.lock();

	handle.read_to_string(&mut buffer).expect("something went wrong");
		println!("{:?}", buffer );	

	for character in buffer.into_bytes() {
	    println!("{:?}",character );
	    cryted.push(character);
	}

	println!("{:?}",cryted );

	let mut _byte : u8 = 0x80;
	let byte_stepped : u8 = step(&_byte);
	println!("before {}, after {}", _byte, byte_stepped );

	for byte in cryted.iter(){
		_byte = *byte;
		byte = step(_byte);
	}

}

fn step(byte : &u8) -> u8
{
	let mut return_byte : u8 = byte << 1;
	let left_bit :u8 =  0x80;

	if byte & left_bit == left_bit {
		return_byte |= 0b1;
	}
	return_byte
}