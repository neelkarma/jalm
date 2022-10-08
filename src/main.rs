use clap::Parser;
use error::Result;
use schedule::Schedule;

mod error;
mod schedule;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    cron_expr: String,
    #[arg(short, long)]
    title: Option<String>,
    #[arg(short, long, default_value_t = 50)]
    width: usize,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let schedule = Schedule::try_new(args.title, &args.cron_expr)?;
    println!("{}", schedule.render(args.width));
    Ok(())
}
