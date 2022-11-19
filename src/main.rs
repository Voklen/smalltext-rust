use phf_macros::phf_map;
use std::io;

static SUPERSCRIPT: phf::Map<char, char> = phf_map! {
'a' => 'ᵃ',
'b' => 'ᵇ',
'c' => 'ᶜ',
'd' => 'ᵈ',
'e' => 'ᵉ',
'f' => 'ᶠ',
'g' => 'ᵍ',
'h' => 'ʰ',
'i' => 'ᶦ',
'j' => 'ʲ',
'k' => 'ᵏ',
'l' => 'ˡ',
'm' => 'ᵐ',
'n' => 'ⁿ',
'o' => 'ᵒ',
'p' => 'ᵖ',
'q' => 'ᵠ',
'r' => 'ʳ',
's' => 'ˢ',
't' => 'ᵗ',
'u' => 'ᵘ',
'v' => 'ᵛ',
'w' => 'ʷ',
'x' => 'ˣ',
'y' => 'ʸ',
'z' => 'ᶻ',
	};
fn main() -> io::Result<()> {
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
	match SUPERSCRIPT.get(&character) {
		Some(super_character) => super_character.clone(),
		None => character,
	}
}
