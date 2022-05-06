# countwords

This is a simple Rust program to count the unique words in one or more files. It uses an unbalanced binary search tree to store how many times each word has occurred, and print the results in alphabetical order.

## Usage

Compile it:

```sh
$ cargo build
```

If you pass any arguments, they are filenames; otherwise, it uses standard input. When multiple files are used, their words are stored in aggregate.

```sh
$ echo 'foo bar bar baz' | ./target/debug/countwords
bar 2
baz 1
foo 1
$ echo 'abc 123' > file1
$ echo 'monkey abc banana banana' > file2
$ ./target/debug/countwords file1 file2
123 1
abc 2
banana 2
monkey 1
```
