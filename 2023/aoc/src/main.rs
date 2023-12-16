use anyhow::{anyhow, bail, Context, Result};
use chrono::Datelike;
use clap::Parser;

#[cfg(not(target_os = "illumos"))]
use copypasta::{ClipboardContext, ClipboardProvider};

type Solver = fn(&str) -> (String, String);
const DAYS: [Solver; 25] = [
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
    day09::solve,
    day10::solve,
    day11::solve,
    day12::solve,
    day13::solve,
    day14::solve,
    day15::solve,
    day16::solve,
    day17::solve,
    day18::solve,
    day19::solve,
    day20::solve,
    day21::solve,
    day22::solve,
    day23::solve,
    day24::solve,
    day25::solve,
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

async fn read_input_for(day: u32) -> Result<String> {
    let path: std::path::PathBuf =
        [&format!("{day:02}"), "input"].into_iter().collect();

    // Check for a pre-existing input file
    if path.exists() {
        return std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read input from {path:?}"));
    }

    let t = chrono::Local::now();
    if day > t.day() {
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

    let year = t.year();
    let client = reqwest::ClientBuilder::new()
        .user_agent(
            "github.com/mkeeter/advent-of-code/blob/master/2023/aoc by \
             matt.j.keeter@gmail.com",
        )
        .cookie_provider(jar.into())
        .build()
        .context("failed to build client")?;
    let r = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
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

    // Download the input, if necessary

    if args.bench {
        use criterion::Criterion;
        let days = if args.all {
            let t = chrono::Local::now();
            if t.year() > 2023 {
                (0..DAYS.len()).map(|i| i as u32 + 1).collect()
            } else {
                (1..=t.day()).collect()
            }
        } else {
            vec![day]
        };
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

    let input = if args.example {
        let f =
            std::fs::read("example").context("failed to read `./example`")?;
        String::from_utf8(f).context("example is not valid UTF-8")?
    } else if args.paste {
        #[cfg(target_os = "illumos")]
        bail!("cannot use clipboard on illumos");

        #[cfg(not(target_os = "illumos"))]
        ClipboardContext::new()
            .and_then(|mut ctx| ctx.get_contents())
            .map_err(|e| anyhow!("failed to create clipboard context: {e:?}"))?
    } else {
        read_input_for(day).await?
    };

    let out = (DAYS[day as usize - 1])(&input);
    println!("Part 1: {}", out.0);
    println!("Part 2: {}", out.1);

    Ok(())
}
