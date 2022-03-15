# env-file-reader

[![Build Status](https://github.com/jofas/env_file_reader/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/env_file_reader/actions/workflows/build.yml)
[![Codecov](https://codecov.io/gh/jofas/env_file_reader/branch/master/graph/badge.svg?token=69YKZ1JIBK)](https://codecov.io/gh/jofas/env_file_reader)
[![Latest Version](https://img.shields.io/crates/v/env-file-reader.svg)](https://crates.io/crates/env-file-reader)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/env-file-reader/latest/env_file_reader)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Library for reading environment variables from an environment file in
rust.

## Usage

Imagine this to be the content of your environment file located at
`examples/.env`:

```ini
CLIENT_ID=YOUR_CLIENT_ID
CLIENT_SECRET=YOUR_CLIENT_SECRET
```

Now you want to read this file and expose the environment variables 
to your rust application. 
You can easily do this using the `env-file-reader` crate:

```rust
use env_file_reader::read_file;

fn main() -> std::io::Result<()> {
  let env_variables = read_file("examples/.env")?;
  
  assert_eq!(&env_variables["CLIENT_ID"], "YOUR_CLIENT_ID");
  assert_eq!(&env_variables["CLIENT_SECRET"], "YOUR_CLIENT_SECRET");

  Ok(())
}
```

The `env-file-reader` crate exposes the `read_file` function to which
you can pass the path to your environment file.
The `read_file` function then parses the environment file and extracts
the contained variables, returning them as a 
`HashMap<String, String>`, from which they can be accessed easily by
your rust application.

### Variable names and unicode support

### Optional export keyword

### Quoted and multiline values

### Empty values

### Reading multiple environment files


## TODO

* [ ] documentation: top-level and methods

* [ ] support for single quotes

* [ ] support for multiline values

* [ ] support for escaped quotes

* [ ] test suite

* [ ] release `v0.3.0`

* [ ] type support through `serde`
