This is **pgen**, a simple password generator written in Rust.

```
$ pgen
ddubhxjeaznxkzew
```

```
$ pgen --help
pgen 0.2.0
Bradley Gannon <bradley@bradleygannon.com>
Generates random passwords

USAGE:
    pgen [LENGTH]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <LENGTH>
```

To install:

1. `git clone` and `cd` into this repo
2. `cargo build`
3. `cargo run [ARGS]` or `./target/debug/pgen [ARGS]`

I'm writing pgen because I think Rust is cool, and I want to build stuff with
it. I'm not that good at it yet, so this seemed like a decent first project.
I'm basically copying [pwgen](https://linux.die.net/man/1/pwgen), which you
should probably use instead.
