pub mod left_shift_register;

#[cfg(test)]
mod tests {
	use left_shift_register::left_shift_register::LeftShiftRegister;

    #[test]
    fn inputs_do_not_mutate() {
	   	let lfr = LeftShiftRegister{_step : 1, buffer : 4};

    	assert_eq!(lfr._step, 1);
    	assert_eq!(lfr.buffer, 4);
    }
    
    #[test]
    fn step_once() {
	   	let mut lfr = LeftShiftRegister{_step : 1, buffer : 4};

	   	lfr.step();
	   	assert_eq!(lfr.buffer, 8);
    }
    
    #[test]
    fn step_twice() {
	   	let mut lfr = LeftShiftRegister{_step : 1, buffer : 4};

	   	lfr.step();
	   	lfr.step();
	   	assert_eq!(lfr.buffer, 16);        
    }

    #[test]
    fn step_eight_buffer_seven() {
        let mut lfr = LeftShiftRegister{_step: 2, buffer : 0b11100111};

		assert_eq!( 0, lfr.step());
		assert_eq!( 0, lfr.step());
		assert_eq!( 0, lfr.step());
		assert_eq!( 0, lfr.step());
		assert_eq!( 0, lfr.step());
		assert_eq!( 1, lfr.step());
		assert_eq!( 1, lfr.step());
		assert_eq!( 1, lfr.step());
    }

    #[test]
    fn two_steps_is_generate_two() {
	   	let mut lfrStep = LeftShiftRegister{_step : 1, buffer : 4};
	   	let mut lfrGenerate = LeftShiftRegister{_step : 1, buffer : 4};

	   	lfrStep.step();
	   	lfrStep.step();
	   	lfrGenerate.generate(2);
	   	assert_eq!(lfrStep.buffer, lfrGenerate.buffer);        	    
    }
    #[test]
    fn transform_byte_vector() {
		let StringInput = String::from("this is a string");

		let mut input : Vec<u8> = Vec::from(StringInput);
		let inputCheck : Vec<u8> = input.clone();

		let mut _lsr = LeftShiftRegister{_step : 2, buffer : 0b11100111};
		let mut _lsr2 = LeftShiftRegister{_step : 2, buffer : 0b11100111};

		for byte in input.iter_mut(){
			*byte ^= _lsr.buffer;
			_lsr.generate(3);
		}

		println!("{:?}",input);

		for byte in input.iter_mut(){
			*byte ^= _lsr2.buffer;
			_lsr2.generate(3);
		}
		assert_eq!(input, inputCheck);
    }

}







