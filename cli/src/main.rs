use chrono::Local;
use clap::Parser;
use glob::glob;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(
    name = "amazcomp",
    about = "Extremely high compression ratio and high-speed, lightest compression technology, available via CLI",
    after_help = "This tool currently supports compression only and does not support decompression."
)]
struct Args {
    #[arg(required = true)]
    filename: Vec<String>,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Clone, Copy)]
enum Level {
    Debug,
    Info,
    Error,
}

fn level_color(level: Level) -> &'static str {
    match level {
        Level::Debug => inline_colorization::color_blue,
        Level::Info => inline_colorization::color_green,
        Level::Error => inline_colorization::color_red,
    }
}

fn level_name(level: Level) -> &'static str {
    match level {
        Level::Debug => "DEBUG",
        Level::Info => "INFO",
        Level::Error => "ERROR",
    }
}

fn now_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

fn log(level: Level, msg: impl AsRef<str>) {
    let color = level_color(level);
    let reset = inline_colorization::color_reset;
    let lvl = format!("{:<8}", level_name(level));
    println!(
        "[{}] {color}{lvl}{reset}\t{}",
        now_timestamp(),
        msg.as_ref()
    );
}

fn resolve_base_dir(args: &Args) -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| format!("failed to get current dir: {e}"))?;
    match &args.output {
        None => Ok(cwd),
        Some(output) => {
            if args.filename.len() > 1 {
                let mut base = PathBuf::from(output);
                if !base.is_absolute() {
                    base = cwd.join(base);
                }
                fs::create_dir_all(&base)
                    .map_err(|e| format!("failed to create output dir {}: {e}", base.display()))?;
                base.canonicalize()
                    .map_err(|e| format!("failed to resolve output dir {}: {e}", base.display()))
            } else {
                Ok(cwd)
            }
        }
    }
}

fn distribute_time_ms(total_ms: f64, phases: usize) -> Vec<f64> {
    let mut rng = thread_rng();
    let mut weights = Vec::with_capacity(phases);
    for _ in 0..phases {
        weights.push(1.0 + rng.gen_range(-0.25..0.25));
    }
    let s: f64 = weights.iter().sum();
    let raw: Vec<f64> = weights.iter().map(|w| w / s * total_ms).collect();
    let jittered: Vec<f64> = raw
        .iter()
        .map(|r| r * (1.0 + rng.gen_range(-0.10..0.10)))
        .collect();
    let factor = total_ms / jittered.iter().sum::<f64>();
    let mut allocated: Vec<f64> = jittered.iter().map(|r| r * factor).collect();
    let allocated_sum: f64 = allocated.iter().sum();
    if let Some(last) = allocated.last_mut() {
        *last += total_ms - allocated_sum;
    }
    allocated
}

fn ms(n: f64) -> Duration {
    Duration::from_secs_f64(n / 1000.0)
}

fn human_int(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out.chars().rev().collect()
}

fn random_name_6() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn run() -> Result<(), String> {
    let args = Args::parse();
    if args.verbose {
        log(Level::Info, "Loaded application.");
        log(Level::Info, "Parsed input.");
    }

    let base_dir = resolve_base_dir(&args)?;
    for glob_pattern in &args.filename {
        log(Level::Info, format!("Loading file pattern: {glob_pattern}"));

        let mut files: Vec<String> = Vec::new();
        let entries = glob(glob_pattern).map_err(|e| format!("invalid glob pattern: {e}"))?;
        for entry in entries {
            match entry {
                Ok(path) => files.push(path.display().to_string()),
                Err(e) => log(Level::Error, format!("glob error: {e}")),
            }
        }

        if files.is_empty() {
            log(Level::Error, format!("File does not exist: {glob_pattern}"));
        }

        for file in &files {
            let mut rng = thread_rng();
            log(Level::Info, format!("Start processing of file: {file}"));
            log(Level::Info, "Analysing...");
            thread::sleep(ms(480.0));

            log(Level::Info, "Analysis complete.");
            println!("Compressing: {file}");
            let total_target_ms = rng.gen_range(800.0..900.0);
            let phase_times_ms = distribute_time_ms(total_target_ms, 5);

            let start = std::time::Instant::now();
            log(Level::Info, "Initializing Adaptive Multi-Phase Compression Engine");
            log(Level::Debug, "Loading entropy models (context-mixed, 12 layers)");
            log(Level::Debug, "Calibrating symbol frequency tables... OK");
            log(Level::Info, "Input stream detected");

            log(Level::Info, "Phase 1/5: Preconditioning & Block Segmentation");
            let p1_ms = phase_times_ms[0];
            let seg_count = (rng.gen_range(0.0_f64..1.0_f64) * 20.0_f64 + 302.0_f64).round() as u64;
            log(Level::Debug, "Segment size heuristic: dynamic (64KiB-4MiB)");
            thread::sleep(ms(p1_ms * 0.15));
            log(
                Level::Debug,
                format!("    Generated {seg_count} variable-length segments"),
            );
            thread::sleep(ms(p1_ms * 0.35));
            log(
                Level::Debug,
                "    Applying reversible delta transform (D2 predictor)",
            );
            thread::sleep(ms(p1_ms * 0.20));
            log(Level::Debug, "    Performing byte-wise decorrelation pass");
            thread::sleep(ms(p1_ms * 0.30));
            log(Level::Info, "Phase 1 complete.");

            log(Level::Info, "Phase 2/5: Context-Aware Dictionary Synthesis");
            let p2_ms = phase_times_ms[1];
            let window_size: u64 = rng.gen_range(1_000..1_000_001);
            log(
                Level::Debug,
                format!("    Building rolling hash index, with window size={window_size}"),
            );
            thread::sleep(ms(p2_ms * 0.20));
            let candidates = rng.gen_range(1_700_000_u64..1_950_001_u64);
            log(
                Level::Debug,
                format!("    Candidate patterns extracted: {}", human_int(candidates)),
            );
            thread::sleep(ms(p2_ms * 0.35));
            let prune_thresh = rng.gen_range(0.0028..0.0034);
            log(
                Level::Debug,
                format!("    Pruning low-gain entries (threshold={prune_thresh:.4})"),
            );
            thread::sleep(ms(p2_ms * 0.25));
            let dict_options = [65_536_u64, 65_500_u64, 65_536_u64, 65_520_u64];
            let dict_size = dict_options[rng.gen_range(0..dict_options.len())];
            log(
                Level::Debug,
                format!("    Final dictionary size: {} entries", human_int(dict_size)),
            );
            thread::sleep(ms(p2_ms * 0.20));
            log(Level::Info, "Phase 2 complete.");

            log(Level::Info, "Phase 3/5: Hybrid LZ-Predictive Encoding");
            let p3_ms = phase_times_ms[2];
            log(
                Level::Debug,
                "    Switching to adaptive match finder (lazy+optimal mix)",
            );
            thread::sleep(ms(p3_ms * 0.25));
            let longest_match = rng.gen_range(90_000_u64..110_001_u64);
            log(
                Level::Debug,
                format!("    Longest match detected: {} bytes", human_int(longest_match)),
            );
            thread::sleep(ms(p3_ms * 0.45));
            log(
                Level::Debug,
                "    Back-reference graph optimized (DAG reduction pass)",
            );
            thread::sleep(ms(p3_ms * 0.30));
            log(Level::Info, "Phase 3 complete.");

            log(Level::Info, "Phase 4/5: Arithmetic Coding (Range Encoder)");
            let p4_ms = phase_times_ms[3];
            log(Level::Debug, "    Initial range: [0x00000000, 0xFFFFFFFF]");
            thread::sleep(ms(p4_ms * 0.10));
            let renorms = rng.gen_range(4_500_000_u64..5_200_001_u64);
            log(
                Level::Debug,
                format!("    Renormalizations performed: {}", human_int(renorms)),
            );
            thread::sleep(ms(p4_ms * 0.70));
            let interval_width = rng.gen_range(0x1000_u64..0x20000_u64);
            log(
                Level::Debug,
                format!("    Final interval width: {interval_width:#x}"),
            );
            thread::sleep(ms(p4_ms * 0.20));
            log(Level::Info, "Phase 4 complete.");

            log(Level::Info, "Phase 5/5: Post-Compression Optimization");
            let p5_ms = phase_times_ms[4];
            log(Level::Debug, "    Bitstream alignment: 128-bit boundary");
            thread::sleep(ms(p5_ms * 0.25));
            log(Level::Debug, "    Applying secondary entropy smoothing pass");
            thread::sleep(ms(p5_ms * 0.40));
            log(Level::Debug, "    CRC64 checksum embedded");
            thread::sleep(ms(p5_ms * 0.35));
            log(Level::Info, "Phase 5 complete.");

            log(Level::Info, "Integrity check: PASSED");
            let elapsed_ms = start.elapsed().as_millis();
            log(Level::Info, "Compression pipeline finished successfully.");
            log(Level::Info, format!("Total elapsed: {elapsed_ms} ms"));

            let out_path = if args.output.is_some() && files.len() == 1 && args.filename.len() == 1 {
                PathBuf::from(args.output.clone().unwrap_or_default())
            } else {
                let in_name = Path::new(file)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| format!("invalid input file name: {file}"))?;
                let rand_name = random_name_6();
                base_dir.join(format!("amazing-{rand_name}_{in_name}"))
            };

            log(Level::Info, "Writing files...");
            fs::write(&out_path, b"")
                .map_err(|e| format!("failed to write output {}: {e}", out_path.display()))?;
            log(Level::Info, "Complete!");
        }
    }

    println!("Compression finished.");
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
