mod arguments;
mod converters;
mod errors;
mod lines;

use std::io::{self, stdin};

use arguments::*;
use converters::*;
use lines::*;

fn main() {
	// Imperative shell
	let command_line_args = get_arguments();
	let converter = ask_or_get_converter(&command_line_args);
	let input_lines = get_files_or_stdin(command_line_args);
	// Functional core
	let line_to_smalltext = |x| convert(x, &converter);
	let output = input_lines.map(line_to_smalltext);
	// Imperative shell
	for line in output {
		println!("{line}")
	}
}

fn ask_or_get_converter(arguments: &RunArguments) -> Converters {
	if arguments.interactive {
		interactive_questions(arguments)
	} else {
		get_converter(arguments)
	}
}

fn get_files_or_stdin(arguments: RunArguments) -> Lines {
	if arguments.files.is_empty() {
		Lines::stdin_lines()
	} else {
		Lines::file_lines(arguments)
	}
}

fn interactive_questions(arguments: &RunArguments) -> Converters {
	let converter = match &arguments.convert_to {
		Some(converter) => converter.clone(),
		None => ask_converter(),
	};
	println!("Enter text to be converted (ctrl-c to exit):");
	converter
}

fn get_converter(arguments: &RunArguments) -> Converters {
	match &arguments.convert_to {
		Some(x) => x.clone(),
		None => throw!("No converter specified and not running interactively\nTry 'smalltext --help' for more information."),
	}
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
	let stdin = stdin();
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

fn throw_errors<T>(value: io::Result<T>) -> T {
	match value {
		Ok(x) => x,
		Err(err) => {
			throw!("Error reading line: {err}")
		}
	}
}
