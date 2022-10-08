use anyhow::Result;
use clap::Parser;

use crate::args::Cli;
use crate::schedule::{RenderOpts, Schedule};

mod args;
mod schedule;

fn main() -> Result<()> {
    let args = Cli::parse();
    let schedule = Schedule::try_new(args.title, &args.cron_expr)?;
    println!(
        "{}",
        schedule.render(RenderOpts {
            width: args.width,
            color: args.color.into()
        })
    );
    Ok(())
}
