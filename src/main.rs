mod arguments;
mod converters;

use std::io;

use arguments::*;
use converters::*;

fn main() -> io::Result<()> {
	let run_optoins = get_run_options();
	let converter = question_user_if_interactive(run_optoins);
	let input_lines = std::io::stdin().lines();
	let line_to_smalltext = |x| -> io::Result<String> { Ok(convert(x?, &converter)) };
	let output = input_lines.map(line_to_smalltext);
	for line in output {
		println!("{}", line?)
	}
	Ok(())
}

fn question_user_if_interactive(run_optoins: RunOptions) -> Converters {
	if run_optoins.interactive {
		interactive_questions(run_optoins)
	} else {
		match run_optoins.convert_to {
			Some(x) => x,
			None => throw("No converter specified and not running interactively"),
		}
	}
}

fn interactive_questions(run_optoins: RunOptions) -> Converters {
	let converter = match run_optoins.convert_to {
		Some(converter) => converter,
		None => ask_converter(),
	};
	println!("Enter text to be converted:");
	converter
}

fn ask_converter() -> Converters {
	println!("Enter smalltext type to convert to (subscript, superscript, smallcaps)");
	let input = match get_input() {
		Ok(x) => x,
		Err(_) => {
			println!("Sorry, could not read input");
			return ask_converter();
		}
	};
	match input.to_ascii_lowercase().trim() {
		"sub" => Converters::Subscript,
		"subscript" => Converters::Subscript,
		"super" => Converters::Superscript,
		"superscript" => Converters::Superscript,
		"small" => Converters::Smallcaps,
		"smallcaps" => Converters::Smallcaps,
		other_input => {
			println!("\"{other_input}\" is not a valid converter type");
			ask_converter()
		}
	}
}

fn get_input() -> io::Result<String> {
	let mut buffer = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut buffer)?;
	Ok(buffer)
}

fn convert(str: String, converter: &Converters) -> String {
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
