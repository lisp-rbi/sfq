# fqltz
Fast(Q/A) compression and random access library

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/RobertBakaric/susq-rust/blob/master/LICENSE)

Short description

## Installation

To install fqlzt, first install Rust (> v1.38) and g++ (> v... ). lzt is currently tested on Rust 1.39.0, but it is likely to work on other subsequent versions as well.

To install library:

```
// not applicible at this time
cargo install fqlzt
```

To create run the app (debuf by def):


```
cargo run 

// or 

cargo run -- -h
```
To create binary:

```
cargo build fqlzt
```

## Usage (temporary)
```

  fqlzt -h

# make index

  fqlzt -i file.fq -o file.fq.lzt -a c

# de(compress/index)
  fqlzt -i file.fq.lzt -a d -o file.fq

#extract (random access)
  fqlzt -i file.fq.lzt -a e -o file.fq -l list.csv -d bi
  fqlzt -i file.fq.lzt -a e -o file.fq -l "rand(15)" -d fwd


```


## License

The software is licensed under the  [MIT license](http://opensource.org/licenses/MIT).

