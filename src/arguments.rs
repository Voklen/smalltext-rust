use crate::throw;

pub struct RunOptions {
	pub interactive: bool,
	pub convert_to: Option<Converters>,
}

#[derive(PartialEq)]
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
}

pub fn get_run_options() -> RunOptions {
	// Skip the first argument because it's just the executable path
	std::env::args().skip(1).map(parse_argument).fold(
		RunOptions {
			interactive: false,
			convert_to: None,
		},
		add_argument,
	)
}

fn parse_argument(arg: String) -> Argument {
	match arg.as_str() {
		"-i" => Argument::Interactive,
		"--super" => Argument::Superscript,
		"--sub" => Argument::Subscript,
		"--smallcaps" => Argument::Smallcaps,
		"--version" => {
			println!(
				"{program_name} {program_ver}",
				program_name = env!("CARGO_PKG_NAME"),
				program_ver = env!("CARGO_PKG_VERSION")
			);
			println!("Copyright (C) 2022 Alexander Gorichev\nLicense GPL-3.0-only: GNU GPL version 3.0 only <https://gnu.org/licenses/gpl-3.0.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\nWritten by Alexander Gorichev.");
			std::process::exit(0)
		}
		_ => {
			println!(
				"{program_name}: invalid option -- '{argument}'",
				program_name = env!("CARGO_PKG_NAME"),
				argument = arg
			);
			std::process::exit(1)
		}
	}
}

fn add_argument(mut options: RunOptions, arg: Argument) -> RunOptions {
	match arg {
		Argument::Interactive => {
			options.interactive = true;
			options
		}
		Argument::Subscript => change_converter(options, Converters::Subscript),
		Argument::Superscript => change_converter(options, Converters::Superscript),
		Argument::Smallcaps => change_converter(options, Converters::Smallcaps),
	}
}

fn change_converter(mut options: RunOptions, new_converter: Converters) -> RunOptions {
	if options.convert_to == None {
		options.convert_to = Some(new_converter);
		return options;
	}

	if options.convert_to != Some(new_converter) {
		throw("Multiple converters selected")
	} else {
		options
	}
}
