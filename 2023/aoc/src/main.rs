use anyhow::{anyhow, bail, Context, Result};
use chrono::Datelike;
use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};

type Solver = fn(&str) -> (String, String);
const DAYS: [Solver; 9] = [
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
    day09::solve,
];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long)]
    day: Option<u8>,

    /// Use `$DAY/input` as input
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

    #[arg(long, requires = "bench", conflicts_with = "day")]
    all: bool,
}

fn read_input_for(day: u32) -> Result<String> {
    let file = format!("{day:02}/input");
    let f = std::fs::read(&file)
        .with_context(|| format!("failed to read input from {file}"))?;
    let out = String::from_utf8(f).context("example is not valid UTF-8")?;
    Ok(out)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.bench && cfg!(debug_assertions) {
        bail!("benchmarking in debug mode isn't meaningful");
    }

    let day = match args.day {
        Some(day) => day as u32,
        None => {
            let t = chrono::Local::now();
            if t.month() != 12 {
                bail!(
                    "Cannot use 'current day' outside of December, \
                     call with --day=N"
                );
            }
            t.day()
        }
    };

    if args.bench {
        use criterion::Criterion;
        let days = if args.all {
            (0..DAYS.len()).map(|i| i as u32 + 1).collect()
        } else {
            vec![day]
        };
        let mut c = Criterion::default();
        for day in days {
            let input = read_input_for(day)?;
            c.bench_function(&format!("day{day:02}"), |b| {
                b.iter(|| DAYS[day as usize - 1](&input))
            });
        }
        c.final_summary();
        return Ok(());
    }

    let input = if args.example {
        let f =
            std::fs::read("example").context("failed to read `./example`")?;
        String::from_utf8(f).context("example is not valid UTF-8")?
    } else if args.paste {
        ClipboardContext::new()
            .and_then(|mut ctx| ctx.get_contents())
            .map_err(|e| anyhow!("failed to create clipboard context: {e:?}"))?
    } else {
        read_input_for(day)?
    };

    let out = (DAYS[day as usize - 1])(&input);
    println!("Part 1: {}", out.0);
    println!("Part 2: {}", out.1);

    Ok(())
}
