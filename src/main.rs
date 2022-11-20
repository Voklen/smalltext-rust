mod arguments;
mod converters;

use std::io;

use arguments::*;
use converters::*;

fn main() -> io::Result<()> {
	let run_optoins = get_run_options();
	let converter = question_user_if_interactive(run_optoins);
	let input = get_input()?;
	let converted = convert(input, converter);
	println!("{converted}");
	Ok(())
}

fn question_user_if_interactive(run_optoins: RunOptions) -> Converters {
	if run_optoins.interactive {
		interactive_questions()
	} else {
		match run_optoins.convert_to {
			Some(x) => x,
			None => throw("No converter specified and not running interactively"),
		}
	}
}

fn interactive_questions() -> Converters {
	todo!()
}

fn get_input() -> io::Result<String> {
	let mut buffer = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut buffer)?;
	Ok(buffer)
}

fn convert(str: String, converter: Converters) -> String {
	let convert_yeah = match converter {
		Converters::Subscript => |chr| convert_char(chr, &SUBSCRIPT),
		Converters::Superscript => |chr| convert_char(chr, &SUPERSCRIPT),
		Converters::Smallcaps => |chr| convert_char(chr, &SMALLCAPS),
	};
	str.chars().map(convert_yeah).collect()
}

fn convert_char(character: char, converter: &phf::Map<char, char>) -> char {
	match converter.get(&character) {
		Some(super_character) => super_character.clone(),
		None => character,
	}
}

pub fn throw<T>(error: &str) -> T {
	let program_name = env!("CARGO_PKG_NAME");
	println!("{program_name}: {error}");
	#[cfg(not(debug_assertions))]
	std::process::exit(1);
	#[cfg(debug_assertions)]
	panic!();
}
