# `fakedata_generator`
> A rust crate to generate fake datasets

[![Build Status](https://travis-ci.org/kevingimbel/fakedata_generator.svg?branch=master)](https://travis-ci.org/kevingimbel/fakedata_generator)

# IMPORTANT NOTE

The `gen_corpora_switch` generator will not work when compiling the library into a binary or including it in your project. The generator is not able to find the JSON files it needs to read and I haven't figured out how to include the files yet.
The data comes from the [Corpora Project](https://github.com/dariusk/corpora).

## About

This crate provides functions to generate datasets, e.g. 10 dog breed names or 10 domain names, etc.

It is a very early version and not meant to be used in any production services or third-party tooling. 

## Usage

Load the library with `use fakedata_generator::*` and then call one of the generator functions. 

```rust
use fakedata_generator::*;

fn main() {
    let random_word = gen_enum("some,random,words".to_string());
    println!("Random word is: {}", random_word); 
}
```

A list of all available generators can be found below.

## Generators

### Generators without arguments

#### email

Return a random e-Mail address which is a combination of the username and domain generator. 

Rust function signature
```rust
gen_email() -> String
```

Example
```shell
let word: String = gen_email();
// word = shaneIxD@we.net
```

#### username

Return a random username.

*Note:* predefined list as of `v0.1`.

Function signature
```rust
gen_username() -> String
```

Example
```rust
let word: String = gen_username();
// word = ahmadajmi
```

#### domain

Return a random domain name.

*Note:* Does not yet support all TDLs and true random host names - it's created by a predefined list.

```rust
gen_domain() -> String
```

Example
```rust
let word: String = gen_domain();
// word = "names.us"
```

#### gen_http_method

Return a random HTTP method from a defined list.

Possible values: `"DELETE", "GET", "HEAD", "OPTION", "PATCH", "POST", "PUT"`

```rust
gen_http_method() -> String
```

Example
```rust
let word: String = gen_http_method();
// word = "GET"
```


#### gen_ipv4

Returns a random IP address. Generates four numbers in the range of 0 - 255.

```rust
gen_ipv4() -> String
``` 

Exmaple
```rust
let ip: String = gen_ipv4();
// ip = "168.11.40.75"
```

### Generators with arguments

#### enum

Return random string from set of specified strings. Specify a comma seperated list as argument.

Function signature
```rust
gen_enum(input: String) -> String
```

Example
```rust
let word: String = gen_enum("hello,hola,hallo".to_string());
// word = "hola"
```


#### int

Return random integer in range. Must specify 1 or 2 numbers separated by comma.

Function signature
```rust
gen_int(input: String) -> String
```

Example
```rust
let num: String = gen_enum("1,100".to_string());
// num = "42"
```

### Corpora generator

`gen_corpora_switch` is a special generator that reads JSON files from the [Corpora Project](https://github.com/dariusk/corpora) and returns a random value. As noted above this will not work as of now.

Possible values: 
- `cat`
- `dog`
- `horse`
- `dinosaur`

Each of these will return a random breed of the specified animal.

Function signature
```rust
gen_corpora_switch(input: String) -> String
```

Example
```rust
let cat: String = gen_corpora_switch("cat");
// cat = "European Shorthair"

```