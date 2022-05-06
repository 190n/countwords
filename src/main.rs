use std::cmp::Ordering;
use std::env;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct BSTNode<T: Ord> {
	key: T,
	count: u64,
	left: Option<Box<BSTNode<T>>>,
	right: Option<Box<BSTNode<T>>>,
}

impl<T: Ord> BSTNode<T> {
	fn new(key: T) -> BSTNode<T> {
		BSTNode {
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
					*child = Some(Box::<BSTNode<T>>::new(BSTNode::<T>::new(key)));
				}
			},
		};
	}
}

impl<T: Ord + Display> Display for BSTNode<T> {
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

struct BST<T: Ord> {
	root: Option<BSTNode<T>>,
}

impl<T: Ord> BST<T> {
	fn new() -> BST<T> {
		BST { root: None }
	}

	fn increment(&mut self, key: T) {
		match &mut self.root {
			Some(root) => root.increment(key),
			None => self.root = Some(BSTNode::<T>::new(key)),
		};
	}
}

impl<T: Ord + Display> Display for BST<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		if let Some(root) = &self.root {
			root.fmt(f)?;
		}

		Ok(())
	}
}

fn process<R: Read>(bst: &mut BST<String>, file: &mut R) -> io::Result<()> {
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	for w in contents.split_whitespace() {
		bst.increment(String::from(w));
	}
	Ok(())
}

fn main() {
	let mut bst = BST::<String>::new();
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		process(&mut bst, &mut io::stdin()).expect("error reading stdin");
	} else {
		for filename in &args[1..] {
			let mut file = File::open(filename).expect("error opening file");
			process(&mut bst, &mut file).expect("error reading file");
		}
	}

	print!("{}", bst);
}
