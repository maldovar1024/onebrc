mod baseline;

use std::env;

use crate::baseline::run_baseline;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next();
    let input_file = args.next().expect("Expect a input file");
    let output_file = args.next().expect("Expect a output file");

    run_baseline(&input_file, &output_file)
}
