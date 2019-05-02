# `fakedata_generator`
> A rust crate to generate fake datasets

[![Build Status](https://travis-ci.org/kevingimbel/fakedata_generator.svg?branch=master)](https://travis-ci.org/kevingimbel/fakedata_generator)
[![Crates.io](https://img.shields.io/crates/v/fakedata_generator.svg)](https://crates.io/crates/fakedata_generator)


## About

This crate provides functions to generate datasets, e.g. 10 dog breed names or 10 domain names, etc.

It is a very early version and not meant to be used in any production services or third-party tooling. 

## Usage

Add the following to your `Cargo.toml`.

```
[dependencies]
fakedata_generator = "0.1.0"
```

Then load the library with `use fakedata_generator::*` and then call one of the generator functions. 

```rust
extern crate fakedata_generator;
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

`gen_corpora_switch` is a special generator that gets its data in JSON format taken from the [Corpora Project](https://github.com/dariusk/corpora). A copy of the entire Corpora project is included in the `data` directory.
Not all datasets are available as of now. See the [src/corpora/data.rs](https://github.com/kevingimbel/fakedata_generator/blob/master/src/corpora/data.rs) file for all available sets.

Possible input values: 
- `cat`
- `dog`
- `horse`
- `dinosaur`
- `gemstone`
- `mood`
- `fabric`

Each of these will return a random word from the list.

Function signature
```rust
gen_corpora_switch(input: String) -> String
```

Example
```rust
let word: String = gen_corpora_switch("cat".to_string());
// word = "European Shorthair"
```
