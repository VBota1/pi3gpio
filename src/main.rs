extern crate rppal;

use std::env;
use std::ffi::OsString;
use rppal::gpio::{GPIO,Mode,Level};

const LAST_PIN_NUMBER: u8 = 27;
const HELP_CMD_STR: &'static str = "help";
const SET_CMD_STR: &'static str = "set";
const GET_CMD_STR: &'static str = "get";
const HIGH_PIN_STATE_STR: &'static str = "high";
const LOW_PIN_STATE_STR: &'static str = "low";
const ALL_PINS_STR: &'static str = "all";
const HELP_EXAMPLE_STR: &'static str = "sudo ./pigpio help";

fn main() {
	handle_cmd ( env::args_os() );
}

fn handle_cmd (mut cmd: env::ArgsOs) {
	match cmd.nth(1) {
		Some (action) => {
			if action == OsString::from(HELP_CMD_STR) {
				print_help_text ();
			}
			else if action == OsString::from(SET_CMD_STR) {
				set_pins (cmd); 
			}
			else if action == OsString::from(GET_CMD_STR) {
				read_pins (cmd);
			}
			else {
				panic! ("Unknown command {:?}! \nFor help run \"{}\"", action, HELP_EXAMPLE_STR );
			}
		},
		None => panic! ("No command found! \nFor help run \"{}\"", HELP_EXAMPLE_STR ),
	}
}

fn print_help_text () {
	println! ("SYNTAX: sudo ./pigpio command [state] [pins]");
	println! ("	command: ");
	println! ("		{}	prints the help text", HELP_CMD_STR );
	println! ("		{}	sets the value of the indicated pins to the indicated value", SET_CMD_STR );
	println! ("		{}	prints the state of the indicated pins", GET_CMD_STR );
	println! ("	state: ");
	println! ("		{}	logical 1 for the indicated pins, equivalent voltage 3.3[V]", HIGH_PIN_STATE_STR );
	println! ("		{}	logical 0 for the indicated pins, equivalent voltage 0[V]", LOW_PIN_STATE_STR );
	println! ("	pins: ");
	println! ("		{}	BCM pins between 0 and {}", ALL_PINS_STR, LAST_PIN_NUMBER );
	println! ("Examples:");
	println! ("	sudo ./pigpio help ");
	println! ("	sudo ./pigpio get all");
	println! ("	sudo ./pigpio get 4");
	println! ("	sudo ./pigpio get 10 11");
	println! ("	sudo ./pigpio set low all");
	println! ("	sudo ./pigpio set high 12");
	println! ("	sudo ./pigpio set low 2 5 7");
}

trait ReadState{
	fn read_state (self, gpio: &mut GPIO);
}

impl ReadState for OsString {
	fn read_state (self, gpio: &mut GPIO) {
		match self.to_u8() {
			Ok (pin) => read_pin ( gpio, pin ),
			Err (e) => {
				panic! ("{} \nFor help run \"{}\"", e,  HELP_EXAMPLE_STR );		
			},
		}		
	}
}

fn read_all_pins ( gpio: &mut GPIO ) {
	for pin in 0..(LAST_PIN_NUMBER+1) {
		read_pin ( gpio, pin );
	}
}

fn read_pin ( gpio: &mut GPIO, pin: u8) {
	gpio.set_mode( pin, Mode::Input );
	match gpio.read(pin) {
		Ok(state) =>	println! ("Pin {} = {:?}", pin, state ),
		Err (_) => println! ("Could not read state of pin {}",pin),
	}
}

fn read_pins (mut cmd: env::ArgsOs) {
	let mut gpio = initiate_gpio_handle();
	
	match cmd.nth(0) {
		Some (pin_as_os_string) => {
			if pin_as_os_string == OsString::from(ALL_PINS_STR) {
				read_all_pins ( &mut gpio );
				return;
			}
			else {
				pin_as_os_string.read_state ( &mut gpio );
			}
		}
		None => panic! ("Pin not found! \nFor help run \"{}\"",  HELP_EXAMPLE_STR ),		
	}

	for argument in cmd {
		argument.read_state ( &mut gpio );
	}
	
}	

trait ToU8 {
	fn to_u8 (self) -> Result<u8, &'static str >;
}

impl ToU8 for OsString {
	fn to_u8 (self) -> Result<u8, &'static str> {
		match self.into_string() {
			Ok(x) => match x.parse::<u8>() {
				Ok (y) => return Ok (y),
				Err(_) => return Err("String indicating pin number could not be converted to u8!" ),
			},
			Err(_) => return Err("OsString indicating pin number could not be converted to String!" ),		
		}
	}
}

fn initiate_gpio_handle () -> GPIO {
    let gpio: GPIO;
    match GPIO::new(){
	    Ok (x) => gpio = x,
	    Err (e) => panic! ("Could not access GPIO memory! {}",e),
    }
    
    gpio
}

fn get_pin_state ( argument: Option<OsString>  ) -> Level {
	let mut pin_state = Level::Low;
	match argument {
		Some (state) => {
			if state == OsString::from(HIGH_PIN_STATE_STR) { 
				pin_state = Level::High;
			}
			else if state != OsString::from(LOW_PIN_STATE_STR) {
				panic! ("State {:?} is invalid! \nFor help run \"{}\"", state, HELP_EXAMPLE_STR );
			}
		},
		None => panic! ("State not found! \nFor help run \"{}\"",  HELP_EXAMPLE_STR ),
	}

    pin_state
}

trait SetState{
	fn set_state (self, pin_state: Level ,gpio: &mut GPIO);
}

impl SetState for OsString {
	fn set_state (self, pin_state: Level, gpio: &mut GPIO) {
		match self.to_u8() {
			Ok (pin) => set_pin ( gpio, pin, pin_state ),
			Err (e) => {
				panic! ("{} \nFor help run \"{}\"", e,  HELP_EXAMPLE_STR );		
			},
		}		
	}
}

fn set_all_pins ( gpio: &mut GPIO, pin_state: Level ) {
	for pin in 0..(LAST_PIN_NUMBER+1) {
		set_pin ( gpio, pin, pin_state );
	}
}

fn set_pin ( gpio: &mut GPIO, pin: u8, pin_state: Level ) {
	println! ("Set pin {} to {:?}", pin, pin_state);
	gpio.set_mode( pin, Mode::Output );
	gpio.write ( pin, pin_state );
}

fn set_pins (mut cmd: env::ArgsOs) {
	let mut gpio = initiate_gpio_handle();
	gpio.set_clear_on_drop ( false );

	let pin_state = get_pin_state( cmd.nth(0) );

	match cmd.nth(0) {
		Some (pin_as_os_string) => {
			if pin_as_os_string == OsString::from(ALL_PINS_STR) {
				set_all_pins ( &mut gpio, pin_state );
				return;
			}
			else {
				pin_as_os_string.set_state( pin_state, &mut gpio );
			}
			
		}
		None => panic! ("Pin not found! \nFor help run \"{}\"",  HELP_EXAMPLE_STR ),		
	}

	for argument in cmd {
		argument.set_state( pin_state, &mut gpio );
	}	
}
