// mod baseline;
mod single_thread;

use std::env;

use crate::single_thread::run_single_thread;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next();
    let input_file = args.next().expect("Expect a input file");
    let output_file = args.next().expect("Expect a output file");
    run_single_thread(&input_file, &output_file)

    // run_single_thread("./measurements.txt", "./output.txt")
}
