mod arguments;
mod converters;

use std::io;

fn main() -> io::Result<()> {
	let args = arguments::get_arguments;
	let input = get_input()?;
	let converted = convert(input);
	println!("{converted}");
	Ok(())
}

fn get_input() -> io::Result<String> {
	let mut buffer = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut buffer)?;
	Ok(buffer)
}

fn convert(str: String) -> String {
	str.chars().map(char_to_superscript).collect()
}

fn char_to_superscript(character: char) -> char {
	match converters::SUPERSCRIPT.get(&character) {
		Some(super_character) => super_character.clone(),
		None => character,
	}
}
