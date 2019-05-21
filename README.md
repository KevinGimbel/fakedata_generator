# `fakedata_generator`
> A rust library to generate fake data

!["data generator for Rust"](assets/brand/github_header.png)

[![Build Status](https://travis-ci.org/kevingimbel/fakedata_generator.svg?branch=master)](https://travis-ci.org/kevingimbel/fakedata_generator)
[![Crates.io](https://img.shields.io/crates/v/fakedata_generator.svg)](https://crates.io/crates/fakedata_generator)
[![Documentation at docs.rs](https://docs.rs/fakedata_generator/badge.svg)](https://docs.rs/fakedata_generator/)

## Table of contents

* [About](#about)
* [Usage](#usage)
* [Generators](#generators)
  * [Generators without arguments](#generators-without-arguments)
  * [Generators with arguments](#generators-with-arguments)
  * [Corpora generator](#corpora-generator)
* [Example](#example)
* [Contributing](#contributing)
* [Code of Conduct](#code-of-conduct)
* [License](#license)

## About
[⬆️ Back to Top](#table-of-contents)

This library provides functions to generate random values ("fake data"). It is in its early stages and some values are not yet fully random. Basic documentation is provided below and on [https://docs.rs/fakedata_generator/](https://docs.rs/fakedata_generator/). 

## Usage
[⬆️ Back to Top](#table-of-contents)

Add the library as dependency to your `Cargo.toml`.

```
[dependencies]
fakedata_generator = "0.1.0"
```

Now the the library can be loaded with `use fakedata_generator::*`.    

```rust
extern crate fakedata_generator;
use fakedata_generator::*;

fn main() {
    let random_word = gen_enum("some,random,words".to_string());
    println!("Random word is: {}", random_word); 
}
```

A full list of available generators and their function signature is shown below. 

## Generators

### Generators without arguments
[⬆️ Back to Top](#table-of-contents)

#### email

Return a random e-Mail address which is a combination of the username and domain generator.

Function signature
```rust
gen_email() -> String
```

Example call
```shell
let email: String = gen_email();
// email = shaneIxD@we.net
```

#### username

Return a random username.

*Note:* predefined list as of `v0.2`.

Function signature
```rust
gen_username() -> String
```

Example call
```rust
let user: String = gen_username();
// user = ahmadajmi
```

#### domain

Return a random domain name.

*Note:* Does not yet support all TDLs and true random host names - it's created by a predefined list.

Function signature
```rust
gen_domain() -> String
```

Example call
```rust
let domain: String = gen_domain();
// domain = "names.us"
```

#### gen_http_method

Return a random HTTP method from a defined list.

Possible values: `"DELETE", "GET", "HEAD", "OPTION", "PATCH", "POST", "PUT"`

Function signature
```rust
gen_http_method() -> String
```

Example call
```rust
let method: String = gen_http_method();
// method = "GET"
```


#### gen_ipv4

Returns a random IP address. Generates four numbers in the range of 0 - 255 which are written out in the format `{}.{}.{}.{}`. 

Function signature
```rust
gen_ipv4() -> String
``` 

Example call
```rust
let ip: String = gen_ipv4();
// ip = "168.11.40.75"
```

### Generators with arguments
[⬆️ Back to Top](#table-of-contents)

#### enum

Return random string from set of specified strings. Specify a comma separated list as argument.

Function signature
```rust
gen_enum(input: String) -> String
```

Example call
```rust
let word: String = gen_enum("hello,hola,hallo".to_string());
// word = "hola"
```


#### int

Return random integer in range. Must specify 1 or 2 numbers separated by comma. If 1 argument is specified it is handled as "highest" value and `0` is used as lowest value. 

Function signature
```rust
gen_int(input: String) -> String
```

Example call
```rust
let num: String = gen_enum("1,100".to_string());
// num = "42"
```

### Corpora generator
[⬆️ Back to Top](#table-of-contents)

`gen_corpora_switch` is a special generator that gets its data in JSON format taken from the [Corpora Project](https://github.com/dariusk/corpora). A copy of the entire Corpora project is included in the `data` directory.
Not all data sets are available as of now. See the [src/corpora/data.rs](https://github.com/kevingimbel/fakedata_generator/blob/master/src/corpora/data.rs) file for all available sets.

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

Example call
```rust
let word: String = gen_corpora_switch("cat".to_string());
// word = "European Shorthair"

let fabric: String = gen_corpora_switch("fabric".to_string());
// word = "longcloth"
```

## Example
[⬆️ Back to Top](#table-of-contents)

The following examples show how `fakedata_generator` can be used in a Rust project.

| Name | Description | Repository |
|------|-------------|------------|
| `fakedata_server` | A HTTP API providing random values based on `fakedata_generator` data. | [View code](https://github.com/kevingimbel/fakedata_server) |


## Contributing
[⬆️ Back to Top](#table-of-contents)

We love and welcome every form of contribution.

### Where to start?

Here are some good places to start:

* Issues with label [Good first issue](https://github.com/kevingimbel/fakedata_generator/labels/good%20first%20issue)
* Issues with label [Documentation](https://github.com/kevingimbel/fakedata_generator/labels/documentation)
* Providing example implementations or usage demos

## Code of Conduct
[⬆️ Back to Top](#table-of-contents)

You are expected to follow our [code of conduct](https://github.com/kevingimbel/fakedata_generator/blob/master/CODE_OF_CONDUCT.md) when interacting with the project via issues, pull requests or in any other form. Many thanks to the awesome [contributor covenant](https://www.contributor-covenant.org/) initiative!

## License
[⬆️ Back to Top](#table-of-contents)

[MIT License](https://github.com/kevingimbel/fakedata_generator/blob/master/LICENSE) Copyright (c) 2019 Kevin Gimbel

Special Thanks to the Rust Community, Rust Language Maintainers, and JetBrains for IntelliJ IDEA. See [NOTICE](https://github.com/kevingimbel/fakedata_generator/blob/master/NOTICE) for full list. 
 

