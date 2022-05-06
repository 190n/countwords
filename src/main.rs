use std::cmp::Ordering;
use std::env;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct WordListNode<T: Ord> {
	key: T,
	count: u64,
	left: Option<Box<WordListNode<T>>>,
	right: Option<Box<WordListNode<T>>>,
}

impl<T: Ord> WordListNode<T> {
	fn new(key: T) -> WordListNode<T> {
		WordListNode {
			key,
			count: 1,
			left: None,
			right: None,
		}
	}

	fn increment(&mut self, key: T) {
		match key.cmp(&self.key) {
			Ordering::Equal => self.count += 1,
			not_equal => {
				let child = match not_equal {
					Ordering::Less => &mut self.left,
					Ordering::Greater => &mut self.right,
					_ => return,
				};

				if let Some(c) = child {
					c.increment(key);
				} else {
					*child = Some(Box::<WordListNode<T>>::new(WordListNode::<T>::new(key)));
				}
			},
		};
	}
}

impl<T: Ord + Display> Display for WordListNode<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		if let Some(left) = &self.left {
			left.fmt(f)?;
		}
		write!(f, "{} {}\n", self.key, self.count)?;
		if let Some(right) = &self.right {
			right.fmt(f)?;
		}

		Ok(())
	}
}

struct WordList<T: Ord> {
	root: Option<WordListNode<T>>,
}

impl<T: Ord> WordList<T> {
	fn new() -> WordList<T> {
		WordList { root: None }
	}

	fn increment(&mut self, key: T) {
		match &mut self.root {
			Some(root) => root.increment(key),
			None => self.root = Some(WordListNode::<T>::new(key)),
		};
	}
}

impl<T: Ord + Display> Display for WordList<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		if let Some(root) = &self.root {
			root.fmt(f)?;
		}

		Ok(())
	}
}

fn process<R: Read>(wl: &mut WordList<String>, file: &mut R) -> io::Result<()> {
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	for w in contents.split_whitespace() {
		wl.increment(String::from(w));
	}
	Ok(())
}

fn main() {
	let mut wl = WordList::<String>::new();
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		process(&mut wl, &mut io::stdin()).expect("error reading stdin");
	} else {
		for filename in &args[1..] {
			let mut file = File::open(filename).expect("error opening file");
			process(&mut wl, &mut file).expect("error reading file");
		}
	}

	print!("{}", wl);
}
