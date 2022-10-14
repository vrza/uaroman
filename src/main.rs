use std::io;
use std::io::prelude::*;

mod uaroman;

// sufficiently large output buffer, to minimize syscall overhead
// https://github.com/coreutils/coreutils/blob/master/src/ioblksize.h
const OUT_BUF_SIZE: usize = 128 * 1024;
const EOL: &[u8; 1] = b"\n";

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut outbuf = io::BufWriter::with_capacity(OUT_BUF_SIZE, stdout.lock());

    for line_result in stdin.lock().lines() {
        let line: String = line_result?;
        let romanized_line = uaroman::romanize(line);
        let _ = outbuf.write(romanized_line.as_bytes());
        let _ = outbuf.write(EOL);
    }

    Ok(())
}
