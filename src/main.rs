extern crate phantom_tank;

use phantom_tank::PhantomTank;
use clap::Parser;

/// Combine 2 pictures into 1 picture
#[derive(Parser)]
#[clap(version)]
struct Args {

    /// foreground picture path
    #[clap(short, long)]
    foreground: String,

    /// background picture path
    #[clap(short, long)]
    background: String,
}

fn main() {
    let args = Args::parse();

    let pt = PhantomTank::new(&args.foreground,&args.background).unwrap();

    // TADAA!!
    pt.generate_grey().save("PhantomTank.png").unwrap();
}