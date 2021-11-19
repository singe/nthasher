# nthasher
A fast wordlist to nthash converter

## Usage

Pass it a UTF8 encoded wordlist, and write the output to a file.

`./nthasher <wordlist> > wordlist.nthashes`

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
  Time (mean ± σ):      2.860 s ±  0.016 s    [User: 2.779 s, System: 0.080 s]
  Range (min … max):    2.851 s …  2.899 s    10 runs
```
