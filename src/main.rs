use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

pub mod lib;

// sufficiently large output buffer, to minimize syscall overhead
// https://github.com/coreutils/coreutils/blob/master/src/ioblksize.h
const OUTPUT_BUFFER_SIZE: usize = 128 * 1024;
const DASH: &str = "-";
const EOL: &[u8; 1] = b"\n";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program_name: &str = &args[0];
    let input_filenames = if args.len() > 1
        { args[1..].to_vec() } else { [DASH.to_string()].to_vec() };

    let mut output_buffer = BufWriter::with_capacity(OUTPUT_BUFFER_SIZE, io::stdout().lock());

    for filename in input_filenames.iter() {
        let reader: Box<dyn BufRead> = match filename.as_str() {
            DASH => Box::new(BufReader::new(io::stdin())),
            _ => {
                let open_file = match File::open(filename) {
                    Ok(result) => result,
                    Err(_) => {
                        eprintln!("{program_name}: {filename}: no such file");
                        continue
                    },
                };
                Box::new(BufReader::new(open_file))
            }
        };

        for line_result in reader.lines() {
            let line: String = line_result?;
            let romanized_line = lib::romanize(line);
            let _ = output_buffer.write(romanized_line.as_bytes());
            let _ = output_buffer.write(EOL);
        }
    }

    Ok(())
}
