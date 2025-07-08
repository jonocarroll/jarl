use air_r_parser::RParserOptions;
use air_workspace::discovery::discover_r_file_paths;
use air_workspace::discovery::discover_settings;
use air_workspace::discovery::DiscoveredSettings;
use air_workspace::resolve::PathResolver;
use air_workspace::settings::Settings;

use flir::check::check;
use flir::config::build_config;

use clap::{arg, Parser};
// use std::time::Instant;
use anyhow::Result;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(
    author,
    name = "flir",
    about = "Flint: Find and Fix Lints in R Code",
    after_help = "For help with a specific command, see: `flir help <command>`."
)]
pub struct Args {
    #[arg(
        short,
        long,
        default_value = ".",
        help = "The directory in which to check or fix lints."
    )]
    dir: String,
    #[arg(
        short,
        long,
        default_value = "false",
        help = "Automatically fix issues detected by the linter."
    )]
    fix: bool,
    #[arg(
        short,
        long,
        default_value = "",
        help = "Names of rules to include, separated by a comma (no spaces)."
    )]
    rules: String,
}

/// This is my first rust crate
fn main() -> Result<()> {
    // let start = Instant::now();
    let args = Args::parse();

    let mut resolver = PathResolver::new(Settings::default());
    for DiscoveredSettings { directory, settings } in discover_settings(&[args.dir.clone()])? {
        resolver.add(&directory, settings);
    }

    let paths = discover_r_file_paths(&[args.dir.clone()], &resolver, true)
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    // let paths = vec![Path::new("demos/foo.R").to_path_buf()];

    let parser_options = RParserOptions::default();
    let config = build_config(&args.rules, args.fix, parser_options);

    let diagnostics = check(paths, config);

    match diagnostics {
        Ok(diags) => {
            if !args.fix && !diags.is_empty() {
                for message in &diags {
                    println!("{}", message);
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    };

    Ok(())
    // let duration = start.elapsed();
    // println!("Checked files in: {:?}", duration);
}
