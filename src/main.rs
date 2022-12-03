mod arguments;
mod converters;

use std::io::stdin;
use std::io::{self, StdinLock};
use std::vec::IntoIter;

use arguments::*;
use converters::*;

fn main() {
	// Imperative shell
	let command_line_args = get_arguments();
	let converter = if command_line_args.interactive {
		interactive_questions(&command_line_args)
	} else {
		get_converter(&command_line_args)
	};
	let input_lines = if command_line_args.files.is_empty() {
		Lines::Stdin(get_lines_from_terminal())
	} else {
		Lines::File(get_file_input_lines(command_line_args))
	};
	// Functional core
	let line_to_smalltext = |x| convert(x, &converter);
	let output = input_lines.map(line_to_smalltext);
	// Imperative shell
	for line in output {
		println!("{line}")
	}
}

type Stdin = std::iter::Map<
	std::io::Lines<StdinLock<'static>>,
	fn(Result<String, std::io::Error>) -> String,
>;
type File = std::iter::FlatMap<IntoIter<String>, IntoIter<String>, fn(String) -> IntoIter<String>>;

enum Lines {
	Stdin(Stdin),
	File(File),
}

impl Iterator for Lines {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Self::Stdin(iter) => iter.next(),
			Self::File(iter) => iter.next(),
		}
	}
}

fn get_lines_from_terminal() -> Stdin {
	stdin().lines().map(throw_errors)
}

fn get_file_input_lines(arguments: RunArguments) -> File {
	arguments
		.files
		.into_iter()
		.flat_map(|x| file_as_lines(x).into_iter())
}

fn file_as_lines(filename: String) -> Vec<String> {
	std::fs::read_to_string(filename)
		.unwrap()
		.lines()
		.map(|x| x.to_string())
		.collect()
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
		None => throw("No converter specified and not running interactively\nTry 'smalltext --help' for more information."),
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
			let error_string = err.to_string();
			let error_message = format!("Error reading line: {error_string}");
			throw(&error_message)
		}
	}
}

pub fn throw(error: &str) -> ! {
	let program_name = env!("CARGO_PKG_NAME");
	println!("{program_name}: {error}");
	#[cfg(not(debug_assertions))]
	std::process::exit(1);
	#[cfg(debug_assertions)]
	panic!();
}
