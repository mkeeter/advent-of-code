use anyhow::{Context, Result, anyhow, bail};
use chrono::Datelike;
use clap::Parser;

use copypasta::{ClipboardContext, ClipboardProvider};

/// Helper function to wrap a solver into something returning strings
const fn wrap<A, B, F>(f: F) -> impl Fn(&str) -> (String, String)
where
    A: std::fmt::Display,
    B: std::fmt::Display,
    F: Fn(&str) -> (A, B),
{
    move |s: &str| {
        let (a, b) = f(s);
        (a.to_string(), b.to_string())
    }
}

type Solver = &'static dyn Fn(&str) -> (String, String);
const DAYS: [Solver; 12] = [
    &wrap(aoc::day01::solve),
    &wrap(aoc::day02::solve),
    &wrap(aoc::day03::solve),
    &wrap(aoc::day04::solve),
    &wrap(aoc::day05::solve),
    &wrap(aoc::day06::solve),
    &wrap(aoc::day07::solve),
    &wrap(aoc::day08::solve),
    &wrap(aoc::day09::solve),
    &wrap(aoc::day10::solve),
    &wrap(aoc::day11::solve),
    &wrap(aoc::day12::solve),
];

const YEAR: i32 = 2025;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long)]
    day: Option<u8>,

    /// Use `input/$DAY` as input
    #[arg(short, long, conflicts_with_all = ["example", "paste"])]
    input: bool,

    /// Use `./example` as input
    #[arg(short, long, conflicts_with_all = ["paste", "bench"])]
    example: bool,

    /// Use the system clipboard as input
    #[arg(short, long, conflicts_with_all = ["bench"])]
    paste: bool,

    #[arg(long)]
    bench: bool,

    #[arg(long, conflicts_with = "day")]
    all: bool,
}

async fn read_input_for(day: u32) -> Result<String> {
    let path: std::path::PathBuf =
        ["input", &format!("{day:02}")].into_iter().collect();

    // Check for a pre-existing input file
    if path.exists() {
        return std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read input from {path:?}"));
    }

    let target = chrono::NaiveDate::from_ymd_opt(YEAR, 12, day).unwrap();
    let now = chrono::Utc::now().date_naive();
    if target > now {
        bail!("cannot request inputs from the future");
    }
    let jar = {
        let mut cookie_path = dirs::home_dir()
            .ok_or_else(|| anyhow!("could not get home directory"))?;
        cookie_path.push(".aoc-cookie");
        let cookie =
            std::fs::read_to_string(&cookie_path).with_context(|| {
                format!("failed to read cookie from {cookie_path:?}")
            })?;

        let jar = reqwest::cookie::Jar::default();
        let url = "https://adventofcode.com".parse::<url::Url>()?;
        jar.add_cookie_str(&format!("session={cookie}"), &url);
        jar
    };

    let client = reqwest::ClientBuilder::new()
        .user_agent(format!(
            "github.com/mkeeter/advent-of-code/blob/master/{YEAR}/aoc by \
             matt.j.keeter@gmail.com"
        ))
        .cookie_provider(jar.into())
        .build()
        .context("failed to build client")?;
    let r = client
        .get(format!("https://adventofcode.com/{YEAR}/day/{day}/input"))
        .build()
        .context("failed to build request")?;
    let out = client
        .execute(r)
        .await
        .context("failed to execute request")?;

    let text = out.text().await.context("failed to get text")?;

    // Check for certain well-known replies
    if text.contains("Please don't repeatedly request this endpoint") {
        bail!("download failed due to rate-limiting on the server");
    } else if text.contains("Puzzle inputs differ by user") {
        bail!("login failed; perhaps your cookie is stale?");
    }

    std::fs::write(&path, &text)
        .with_context(|| "failed to write output to {path}")?;

    Ok(text)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.bench && cfg!(debug_assertions) {
        bail!("benchmarking in debug mode isn't meaningful");
    }

    let days = match args.day {
        Some(day) => vec![day as u32],
        None if args.all => {
            let t = chrono::Local::now();
            if t.year() > YEAR {
                (0..DAYS.len()).map(|i| i as u32 + 1).collect()
            } else if t.year() < YEAR || t.month() < 12 {
                bail!("cannot use --all before the start of December");
            } else {
                (1..=t.day()).collect()
            }
        }
        None => {
            let t = chrono::Local::now();
            if t.month() != 12 || t.year() != YEAR {
                bail!(
                    "Cannot use 'current day' outside of December {YEAR}, \
                     call with --day=N"
                );
            }
            vec![t.day()]
        }
    };

    if args.bench {
        use criterion::Criterion;
        let mut c = Criterion::default().with_output_color(true);
        for day in days {
            let input = read_input_for(day).await?;
            c.bench_function(&format!("day{day:02}"), |b| {
                b.iter(|| DAYS[day as usize - 1](&input))
            });
        }
        c.final_summary();
        return Ok(());
    }

    let many = days.len() > 1;
    if many {
        if args.example {
            bail!("cannot provide `./example` for multiple days")
        } else if args.paste {
            bail!("cannot use clipboard input for multiple days")
        }
    }

    for day in days {
        let input = if args.example {
            let f = std::fs::read("example")
                .context("failed to read `./example`")?;
            String::from_utf8(f).context("example is not valid UTF-8")?
        } else if args.paste {
            ClipboardContext::new()
                .and_then(|mut ctx| ctx.get_contents())
                .map_err(|e| {
                    anyhow!("failed to create clipboard context: {e:?}")
                })?
        } else {
            read_input_for(day).await?
        };

        let out = (DAYS[day as usize - 1])(&input);
        let indent = if many {
            println!("Day {day}:");
            "  "
        } else {
            ""
        };
        println!("{indent}Part 1: {}", out.0);
        println!("{indent}Part 2: {}", out.1);
    }

    Ok(())
}
