use md4::{Md4, Digest};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() {
  //let guard = pprof::ProfilerGuard::new(100).unwrap();

  let args: Vec<String> = env::args().collect();
  let file = File::open(&args[1]);

  // Specifying the capacity on the buffered reader gives some speedup
  let mut bufrd = io::BufReader::with_capacity(1000, file.unwrap());

  // Pre-allocate the objects we'll reuse to reduce alloc's
  let mut clear = String::with_capacity(100); // cleartext password
  let mut utf16: Vec<u8> = Vec::with_capacity(200); // utf16 encoded string as bytes
  let mut out: Vec<u8> = Vec::with_capacity(8192); // write buffer
  let mut b = [0;2]; // needed for utf16 encoding, but not used

  // iterate by line, don't use lines() as it allocates a String per line
  while bufrd.read_line(&mut clear).unwrap() != 0 {
    // align_to is unsafe, but faster than to_le_bytes
    unsafe {
      // faster to iter & encode chars than the encode_utf16 str iter
      clear.trim_end()
           .chars()
           .for_each(|n| utf16.extend_from_slice(n.encode_utf16(&mut b)
                                                  .align_to::<u8>().1));
    }
    // doing this single Md4 digest is faster than multiple updates() + finalize()
    write!(&mut out,"{:x}",Md4::digest(&utf16)).unwrap();
    // adding the newline byte here is cheaper than having write do it
    out.push(10);
    // clear our reused buffers
    clear.clear();
    utf16.clear();
    // flush the write buffer
    if out.len() >= 8192 { // make sure this comparison aligns with capacity
      io::stdout().write_all(&out).unwrap();
      out.clear();
    }
  }
  // flush what's left
  io::stdout().write_all(&out).unwrap();
  /*
  if let Ok(report) = guard.report().build() {
    let file = File::create("flamegraph.svg").unwrap();
    report.flamegraph(file).unwrap();
  };
  */
}
