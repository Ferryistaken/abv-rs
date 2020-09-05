# ABV-RS - Abbreviate any text to a given lenght

![Rust](https://github.com/Ferryistaken/abv-rs/workflows/Rust/badge.svg)

## Description

This is a tool created to make any text that is too long shorter (expecially useful in statusbars)

I created this because usually the music module in my statusbar occupies too much space, and I needed a clean way to make it shorter

## Getting Started
`This program was tested in linux only, but should work in windows just fine`

## Compile from source
### Dependencies
* `rust` - to have the std and core components of rust
* `cargo` - to compile rust code

### Step by step guide

1.  `git clone https://github.com/Ferryistaken/abv-rs.git`
2.  `cd abv-rs`
3.  `cargo build --release`
4.  Move the binary into your path (Ex. `mv target/release/abv-rs /usr/bin/`)

## Executing program

```
echo "this is a very long string of text" | abv-rs -l 3
```

Should output `thi...`

## Help

If you have any problems [create a new issue.](https://github.com/Ferryistaken/abv-rs/issues/new)

## Authors

Contributors names and contact info

Alessandro Ferrari - 
[Site](http://ferrry.tk) <br>


## License

This project is licensed under the MIT License - see the LICENSE.txt file for details