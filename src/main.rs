use clap::Parser;
use hidapi::HidApi;

#[derive(Parser, Debug)]
struct Args {
    /// Duration in Milliseconds to cut power for
    duration: u32,
}

fn main() {
    let args = Args::parse();

    let api = HidApi::new().unwrap();
    let device = api.open(0xc0de, 0xcafe).unwrap();

    // Send 0 as report ID, then duration
    let out_bytes = args.duration.to_le_bytes();
    device
        .write(&[0, out_bytes[3], out_bytes[2], out_bytes[1], out_bytes[0]])
        .unwrap();
}
