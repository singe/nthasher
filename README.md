# nthasher
A fast wordlist to nthash converter

## Usage

Pass it a UTF8 encoded wordlist on stdin, and write the output to a file.

`./nthasher < wordlist > wordlist.nthashes`

## UTF8 Encoding

You can use `iconv`

`iconv -t utf8 -c <wordlist.in> > <wordlist.out>`

## Compiling

It's a vanilla rust cargo program that can be compiled with:

`cargo build --release`

You'll need rust and cargo. The easiest way to get it is from https://rustup.rs .

## Performance

Sample hyperfine output for 10 runs against rockyou on an M1 Pro efficency core are below:

```
  Time (mean ± σ):      2.820 s ±  0.002 s    [User: 2.773 s, System: 0.046 s]
  Range (min … max):    2.817 s …  2.824 s    20 runs
```
