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

Variables and values support UTF-8. 
It is perfectly okay to have an environment file that looks like this:

```ini
🦄=💖
💖=🦄
```

```rust
use env_file_reader::read_file;

fn main() -> std::io::Result<()> {
  let env_variables = read_file("examples/.env.utf8")?;
  
  assert_eq!(&env_variables["🦄"], "💖");
  assert_eq!(&env_variables["💖"], "🦄");

  Ok(())
}
```

Variables (and non-quoted values) support every character except
whitespace characters, quotes (`` ` ``, `'`, or `"`), `#` and `=`, so go
nuts:

```rust
use env_file_reader::read_str;

fn main() -> std::io::Result<()> {
  let env_variables = read_str(
    r"123-_variable\$*-@🦄=sprinkely-sprinkely-💖s-and_🐱s@the🏟️",
  )?;
  
  assert_eq!(
    &env_variables[r"123-_variable\$*-@🦄"],
    "sprinkely-sprinkely-💖s-and_🐱s@the🏟️",
  );

  Ok(())
}
```


### Optional export keyword

`env-file-reader` supports `bash`-like environment files where the
variables are exported to the environment via the `export` command:

```bash
export CLIENT_ID=YOUR_EXPORTED_CLIENT_ID
export CLIENT_SECRET=YOUR_EXPORTED_CLIENT_SECRET
```

```rust
use env_file_reader::read_file;

fn main() -> std::io::Result<()> {
  let env_variables = read_file("examples/.env.exported")?;
  
  assert_eq!(&env_variables["CLIENT_ID"], "YOUR_EXPORTED_CLIENT_ID");
  assert_eq!(
    &env_variables["CLIENT_SECRET"], "YOUR_EXPORTED_CLIENT_SECRET",
  );

  Ok(())
}
```


### Reading multiple environment files

Sometimes your environment is split into multiple files (e.g. one 
environment file with your secrets you want to store in a 
kubernetes secret and one environment file with non-secrets you want
to store in a kubernetes config map).
`env-file-reader` supports reading multiple environment files into one
`HashMap` with all variables with the `read_files` function:

```rust
use env_file_reader::read_files;

fn main() -> std::io::Result<()> {
  let env_variables = read_files(&[
    "examples/.env",
    "examples/.env.utf8",
  ])?;
  
  assert_eq!(&env_variables["CLIENT_ID"], "YOUR_CLIENT_ID");
  assert_eq!(&env_variables["CLIENT_SECRET"], "YOUR_CLIENT_SECRET");
  assert_eq!(&env_variables["🦄"], "💖");
  assert_eq!(&env_variables["💖"], "🦄");

  Ok(())
}
```

The environment files are read consecutively in the order they are 
supplied to `read_files`.
Therefore, variables are overridden by the ones that are defined 
later:

```rust
use env_file_reader::read_files;

fn main() -> std::io::Result<()> {
  let env_variables = read_files(&[
    "examples/.env",
    "examples/.env.exported",
  ])?;
  
  assert_eq!(&env_variables["CLIENT_ID"], "YOUR_EXPORTED_CLIENT_ID");
  assert_eq!(
    &env_variables["CLIENT_SECRET"], "YOUR_EXPORTED_CLIENT_SECRET",
  );

  Ok(())
}
```


### Reading environment variables from string

Besides `read_file` and `read_files` `env-file-reader` offers the 
option to read environment variables directly from a string:

```rust
use env_file_reader::read_str;

const ENV_FILE: &str = "
  CLIENT_ID=YOUR_CLIENT_ID
  CLIENT_SECRET=YOUR_CLIENT_SECRET
";

fn main() -> std::io::Result<()> {
  let env_variables = read_str(ENV_FILE)?;
  
  assert_eq!(&env_variables["CLIENT_ID"], "YOUR_CLIENT_ID");
  assert_eq!(&env_variables["CLIENT_SECRET"], "YOUR_CLIENT_SECRET");

  Ok(())
}
```


### Comments and empty lines

Environment files can contain single line comments beginning with a
`#` and empty lines. 
Imagine this to be your environment file located at 
`examples/.env.comments`:

```ini
# A comment
CLIENT_ID=YOUR_CLIENT_ID # A comment at the end of the line

# Empty lines are fine, too

# Another comment
CLIENT_SECRET=YOUR_CLIENT_SECRET # Another comment behind a value
```

`env-file-reader` can parse this file, ignoring empty lines and 
comments:

```rust
use env_file_reader::read_file;

fn main() -> std::io::Result<()> {
  let env_variables = read_file("examples/.env.comments")?;
  
  assert_eq!(&env_variables["CLIENT_ID"], "YOUR_CLIENT_ID");
  assert_eq!(&env_variables["CLIENT_SECRET"], "YOUR_CLIENT_SECRET");

  Ok(())
}
```


### Quoted and multiline values

If you need a value to be more powerful, e.g. contain whitespaces,
quotes, equal sign, etc. (see 
[the section about unicode support](#variable-names-and-unicode-support)), 
you can wrap them in quotes.
The supported quotes are double quotes (`"`), single quotes (`'`) and
backticks (`` ` ``).
A string wrapped in double quotes can contain single quotes and 
backticks and so on.
They also support escaped quotes, so 
`"a string with \"double quotes\""` will work.

```ini
1="I support whitespaces and = and # and even this: \""
2='single quotes work, too and they can contain "double quotes"'
3=`backticks are "also" valid 'quotes'`
```

```rust
use env_file_reader::read_file;

fn main() -> std::io::Result<()> {
  let env_variables = read_file("examples/.env.quotes")?;
  
  assert_eq!(
    &env_variables["1"], 
    "I support whitespaces and = and # and even this: \"",
  );
  assert_eq!(
    &env_variables["2"], 
    "single quotes work, too and they can contain \"double quotes\"",
  );
  assert_eq!(
    &env_variables["3"], 
    "backticks are \"also\" valid 'quotes'",
  );

  Ok(())
}
```

Multiline strings are supported as well and look like this, either 
with a literal line break or with an explicitly typed `\n`:

```ini
PRIVATE_KEY1="-----BEGIN PRIVATE KEY-----
...
-----END PRIVATE KEY-----"

# PRIVATE_KEY2 is identical to PRIVATE_KEY1
PRIVATE_KEY2="-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"
```

```rust
use env_file_reader::read_file;

fn main() -> std::io::Result<()> {
  let env_variables = read_file("examples/.env.multiline")?;
  
  assert_eq!(
    &env_variables["PRIVATE_KEY1"], 
    "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
  );
  assert_eq!(
    &env_variables["PRIVATE_KEY2"], 
    "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
  );

  Ok(())
}
```

**Note on escaped characters:** A quoted string only supports 
explicitly typed newlines and the escaped quote itself.
Other explicitly typed special characters like `\t` or `\r` are not
supported.
So a value equal to `"hello\n\t\"world\""` will have the following
value when being processed by `env-file-reader`

```txt
hello
\t"world"
```

not

```txt
hello
    "world"
```

If you need support for other explicitly typed special characters,
please open an issue.

 
### Whitespaces

Whitespaces around the equal sign are allowed. 
As are whitespaces before the variable name and after the value.
They are trimmed during parsing.

```rust
use env_file_reader::read_str;

const ENV_FILE: &str = "
      CLIENT_ID =     YOUR_CLIENT_ID
  CLIENT_SECRET =YOUR_CLIENT_SECRET
";

fn main() -> std::io::Result<()> {
  let env_variables = read_str(ENV_FILE)?;
  
  assert_eq!(&env_variables["CLIENT_ID"], "YOUR_CLIENT_ID");
  assert_eq!(&env_variables["CLIENT_SECRET"], "YOUR_CLIENT_SECRET");

  Ok(())
}
```

If you need leading or trailing whitespaces in your value, consider 
wrapping it in quotes:

```rust
use env_file_reader::read_str;

const ENV_FILE: &str = "
      CLIENT_ID = '    YOUR_CLIENT_ID    '
  CLIENT_SECRET =YOUR_CLIENT_SECRET
";

fn main() -> std::io::Result<()> {
  let env_variables = read_str(ENV_FILE)?;
  
  assert_eq!(&env_variables["CLIENT_ID"], "    YOUR_CLIENT_ID    ");
  assert_eq!(&env_variables["CLIENT_SECRET"], "YOUR_CLIENT_SECRET");

  Ok(())
}
```


### Empty values

Your variables can be empty:

```ini
CLIENT_ID=
CLIENT_SECRET=
```

```rust
use env_file_reader::read_file;

fn main() -> std::io::Result<()> {
  let env_variables = read_file("examples/.env.empty")?;
  
  assert_eq!(&env_variables["CLIENT_ID"], "");
  assert_eq!(&env_variables["CLIENT_SECRET"], "");

  Ok(())
}
```


### Errors

Should parsing the environment file fail a `std::io::Error` is
returned.
The error includes all normal `io` mishaps, like a missing file:

```rust
use env_file_reader::read_file;

fn main() {
  let err = read_file(".env.which.does.not.exist")
    .err()
    .unwrap();
    
  assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}
```

Additionally, parsing can fail due to an ill-formatted environment
file.
If that is the case, a custom error, `ParseError`, is returned:

```rust
use env_file_reader::read_str;

fn main() {
  let err = read_str("badly formatted env file")
    .err()
    .unwrap();

  assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
  assert_eq!(err.to_string(), "ParseError");
}
```
