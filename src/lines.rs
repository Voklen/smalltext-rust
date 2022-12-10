use crate::*;

use std::io::{self, StdinLock};
use std::iter::{FlatMap, Map};
use std::vec::IntoIter;

type Stdin = Map<io::Lines<StdinLock<'static>>, fn(Result<String, io::Error>) -> String>;
type File = FlatMap<IntoIter<String>, IntoIter<String>, fn(String) -> IntoIter<String>>;

pub enum Lines {
	Stdin(Stdin),
	File(File),
}

impl Lines {
	pub fn stdin_lines() -> Self {
		Self::Stdin(get_stdin_lines())
	}

	pub fn file_lines(arguments: RunArguments) -> Self {
		Self::File(get_file_lines(arguments))
	}
}

fn get_stdin_lines() -> Stdin {
	stdin().lines().map(throw_errors)
}

fn get_file_lines(arguments: RunArguments) -> File {
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

impl Iterator for Lines {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Self::Stdin(iter) => iter.next(),
			Self::File(iter) => iter.next(),
		}
	}
}
