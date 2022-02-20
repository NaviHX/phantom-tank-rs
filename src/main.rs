extern crate phantom_tank;

use clap::Parser;
use phantom_tank::PhantomTank;

/// Combine 2 pictures into 1 picture
#[derive(Parser)]
#[clap(version)]
struct Args {
    /// enable color mode
    #[clap(short, long)]
    color: bool,

    /// foreground picture path
    #[clap(short, long)]
    foreground: String,

    /// background picture path
    #[clap(short, long)]
    background: String,

    /// output path
    #[clap(short, long, default_value_t = String::from("PhantomTank.png"))]
    output: String,
}

fn main() {
    let args = Args::parse();

    let pt_generator = PhantomTank::new(&args.foreground, &args.background).unwrap();

    let phantom_tank = match args.color {
        false => pt_generator.generate_grey(),
        true => pt_generator.generate_color(),
    };

    phantom_tank.save(args.output).unwrap();
}
