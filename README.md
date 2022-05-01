# ruruby-parse

a Ruby parser written in Rust.

## using as a library

```Rust
pub fn parse_program(code: String, path: impl Into<PathBuf>, context_name: &str) -> Result<ParseResult, ParseErr>
```

## comand line usage

### parse script file and print

```sh
> cargo run -- quine/yamanote.rb
```

* `yamanote.rb` is from <https://github.com/mame/yamanote-quine/blob/master/yamanote-quine-inner-circle.rb>.

### one liner

```sh
> cargo run -- -e 1+1
```
