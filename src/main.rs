#![feature(decl_macro)]

extern crate device_query;

mod input;
mod layout;
mod src;
mod voice;

use crate::{layout::Layouts, src::AnimaleseTyping};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Voice to use (1-8), where 1-4 are "female" and 5-8 are "male"
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=8), default_value_t = 1)]
    voice: u8,

    #[arg(short, long, value_enum, default_value_t = Layouts::AnsiUs)]
    layout: Layouts,
}

fn main() {
    let args = Args::parse();
    AnimaleseTyping::new(args.voice.into(), args.layout.into()).run();
}
