use std::cmp::Ordering;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct BSTNode<T: Ord + Display> {
	key: T,
	count: u64,
	left: Option<Box<BSTNode<T>>>,
	right: Option<Box<BSTNode<T>>>,
}

impl<T: Ord + Display> BSTNode<T> {
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

	fn print(&self) {
		if let Some(left) = &self.left {
			left.print();
		}
		println!("{} {}", self.key, self.count);
		if let Some(right) = &self.right {
			right.print();
		}
	}
}

struct BST<T: Ord + Display> {
	root: Option<BSTNode<T>>,
}

impl<T: Ord + Display> BST<T> {
	fn new() -> BST<T> {
		BST {
			root: None,
		}
	}

	fn increment(&mut self, key: T) {
		match &mut self.root {
			Some(root) => root.increment(key),
			None => self.root = Some(BSTNode::<T>::new(key)),
		};
	}

	fn print(&self) {
		if let Some(root) = &self.root {
			root.print();
		}
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

	bst.print();
}
