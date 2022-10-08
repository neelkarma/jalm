use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use colored::{Color, Colorize};
use cron::Schedule as CronSchedule;

const BLOCK: char = '\u{2588}';

pub struct RenderOpts {
    pub width: usize,
    pub color: Color,
}

pub struct Schedule {
    title: Option<String>,
    cron: CronSchedule,
}

impl Schedule {
    pub fn new(title: Option<String>, cron: CronSchedule) -> Self {
        Self { title, cron }
    }

    pub fn try_new(title: Option<String>, cron_expr: &str) -> Result<Self> {
        Ok(Self::new(title, CronSchedule::from_str(cron_expr).context("Invalid cron expression. The required format is \"sec min day(month) month day(week) year\"")?))
    }

    pub fn next(&self) -> Option<DateTime<Local>> {
        self.cron.upcoming(Local).next()
    }

    pub fn prev(&self) -> Option<DateTime<Local>> {
        self.cron.upcoming(Local).next_back()
    }

    pub fn render(&self, opts: RenderOpts) -> String {
        let mut out = String::new();

        let next_opt = self.next();
        let prev_opt = self.prev();

        out.push_str(&self.title.clone().unwrap_or(format!(
            "{} - {}",
            prev_opt.map_or(String::new(), |prev| prev.to_rfc2822()),
            next_opt.map_or(String::new(), |next| next.to_rfc2822())
        )));
        out.push_str("\n  ");

        if let (Some(next), Some(prev)) = (next_opt, prev_opt) {
            let total = next.timestamp() - prev.timestamp();
            let elapsed = Local::now().timestamp() - prev.timestamp();
            let percent = elapsed as f64 / total as f64;

            let blocks_filled = ((opts.width as f64 * percent).floor()) as usize;
            let blocks_unfilled = opts.width - blocks_filled;

            out.push_str(
                &BLOCK
                    .to_string()
                    .repeat(blocks_filled)
                    .color(opts.color)
                    .to_string(),
            );
            out.push_str(
                &BLOCK
                    .to_string()
                    .repeat(blocks_unfilled)
                    .color(opts.color)
                    .dimmed()
                    .to_string(),
            );
            out.push_str(&format!(" {:.0}%", percent * 100.).bold().to_string());
            out.push('\n');
        } else {
            out.push_str(
                &BLOCK
                    .to_string()
                    .repeat(opts.width)
                    .color(opts.color)
                    .to_string(),
            );
            out.push_str(&" 100%\n".bold().to_string());
        }

        out
    }
}
