use crate::throw;

#[derive(Clone)]
pub struct RunArguments {
	pub interactive: bool,
	pub convert_to: Option<Converters>,
	pub files: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub enum Converters {
	Subscript,
	Superscript,
	Smallcaps,
}

enum Argument {
	Interactive,
	Subscript,
	Superscript,
	Smallcaps,
	File(String),
}

pub fn get_arguments() -> RunArguments {
	// Skip the first argument because it's just the executable path
	std::env::args().skip(1).map(parse_argument).fold(
		RunArguments {
			interactive: false,
			convert_to: None,
			files: vec![],
		},
		add_argument,
	)
}

fn parse_argument(argument: String) -> Argument {
	if argument.starts_with('-') {
		parse_dash_argument(&argument)
	} else {
		Argument::File(argument)
	}
}

fn parse_dash_argument(argument: &str) -> Argument {
	match argument {
		"-i" => Argument::Interactive,
		"-v" => print_version_info(),
		"-h" => print_help(),
		"--super" => Argument::Superscript,
		"--sub" => Argument::Subscript,
		"--smallcaps" => Argument::Smallcaps,
		"--interactive" => Argument::Interactive,
		"--version" => print_version_info(),
		"--help" => print_help(),
		_ => print_invalid_argument(argument),
	}
}

fn print_version_info() -> ! {
	let program_name = env!("CARGO_PKG_NAME");
	let program_ver = env!("CARGO_PKG_VERSION");
	println!("{program_name} {program_ver}",);
	println!("Copyright (C) 2022 Alexander Gorichev\nLicense GPL-3.0-only: GNU GPL version 3.0 only <https://gnu.org/licenses/gpl-3.0.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\nWritten by Alexander Gorichev.");
	std::process::exit(0)
}

fn print_help() -> ! {
	let program_name = env!("CARGO_PKG_NAME");
	println!("Usage: {program_name} [OPTION]...");
	println!("Convert text to smalltext");
	println!();
	println!("	-i  --interactive run in interactive mode");
	println!("	    --sub         convert text to subscript");
	println!("	    --super       convert text to superscript");
	println!("	    --smallcaps   convert text to smallcaps");
	println!("	-v  --version     output version information and exit");
	println!("	-h  --help        display this help and exit");
	println!();
	println!("To use smalltext interactively launch it with:");
	println!("	{program_name} -i");
	println!("To convert a file to subscript use:");
	println!("	{program_name} --sub <file>");
	println!("Other arguments can still be used in interactive mode such as:");
	println!("	{program_name} -i --sub");
	println!("Text can even be piped in:");
	println!("	cat foo | {program_name} --sub");
	println!();
	println!("Any questions email: <me@voklen.com>");
	println!("Submit bugs at: <https://github.com/Voklen/smalltext-rust/issues>");
	std::process::exit(0)
}

fn print_invalid_argument(argument: &str) -> ! {
	let program_name = env!("CARGO_PKG_NAME");
	throw!("invalid option -- '{argument}'\nTry '{program_name} --help' for more information.")
}

fn add_argument(mut options: RunArguments, arg: Argument) -> RunArguments {
	match arg {
		Argument::Interactive => {
			options.interactive = true;
			options
		}
		Argument::File(filename) => {
			options.files.push(filename);
			options
		}
		Argument::Subscript => change_converter(options, Converters::Subscript),
		Argument::Superscript => change_converter(options, Converters::Superscript),
		Argument::Smallcaps => change_converter(options, Converters::Smallcaps),
	}
}

fn change_converter(mut options: RunArguments, new_converter: Converters) -> RunArguments {
	if options.convert_to == None {
		options.convert_to = Some(new_converter);
		return options;
	}

	if options.convert_to != Some(new_converter) {
		throw!("Multiple converters selected")
	} else {
		options
	}
}
