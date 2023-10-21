pub struct core6502 {
	accumulator: u8,
	x_reg: u8,
	y_reg: u8,
	status_reg: u8,
	prog_counter: u16,
	stack_pointer: u8,

	bus::Bus,

}

enum AddressingMode {
	Accumulator,
	Absolute,
	AbsoluteX,
	AbsoluteY,
	Immediate,
	Implied,
	Indirect,
	X_Indexed_Indirect,
	Y_Indexed_Indirect,
	Relative,
	ZeroPage,
	ZeroPageX,
	ZeroPageY,

}

impl core6502 {

	pub fn new(bus: Bus) -> Self {

		core6502{
			accumulator: 0,
			x_reg: 0,
			y_reg: 0,
			status_reg: 0,
			prog_counter: 0,
			stack_pointer: 0,

			bus: Bus,
		}
	}


	fn execute_ADC(&mut self, mode: AddressingMode) {

	}


	pub fn reset(&mut self) {
		// TODO: Implement logic to reset CPU
	}
	
	pub fn execute_instruction(&mut self) {
		// TODO: Implement instruction execution logic
	}



}