extern crate assert_approx_eq;
extern crate core;
#[macro_use]
extern crate itertools;
extern crate console;
extern crate indicatif;
extern crate rand;
extern crate rayon;
#[macro_use]
extern crate clap;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate typetag;

mod cli;
mod colouriser;
mod config;
mod data;
mod imager;
mod io;
mod view;
mod world;

use cli::{get_cli_config, CliCommand};
use colouriser::build_colouriser;
use config::{build_book_cover_config, Config};
use console::style;
use imager::build_image;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use io::{load_config, save_config};
use std::error::Error;
use std::process;
use std::time::Instant;

const NUM_OF_THREADS: usize = 4;
const PROGRESS_BAR_STYLE: &str = "[{elapsed_precise}] [{bar:60.cyan/blue}] {percent}% ({eta})";

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<Error>> {
    let cli_config = get_cli_config();

    match cli_config.command {
        CliCommand::RENDER { width, output_path } => {
            render(&cli_config.config_path, width, &output_path)?;
        }
        CliCommand::GENERATE => {
            generate(&cli_config.config_path)?;
        }
    };

    Ok(())
}

fn render(config_path: &str, width: u64, output_path: &str) -> Result<(), Box<Error>> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(NUM_OF_THREADS)
        .build_global()
        .unwrap();

    let started = Instant::now();
    let config = Config::from_save(load_config(config_path), width);
    let colouriser = build_colouriser();

    println!("{} Rendering...", style("[1/2]").bold().dim());
    let test_image = build_image(colouriser, &config, &progress_bar(&config));

    println!("{} Printing image...", style("[2/2]").bold().dim());
    io::write_image_as_ppm(test_image, output_path)?;

    println!("Done in {}", HumanDuration(started.elapsed()));

    Ok(())
}

fn generate(config_path: &str) -> Result<(), Box<Error>> {
    let config_save = build_book_cover_config();
    save_config(config_path, config_save);
    Ok(())
}

fn progress_bar(config: &Config) -> ProgressBar {
    let progress_style = ProgressStyle::default_bar()
        .template(PROGRESS_BAR_STYLE)
        .progress_chars("##-");
    let bar_size = config.height * config.width;
    let progress_bar = ProgressBar::new(bar_size);
    progress_bar.set_style(progress_style.clone());
    progress_bar.tick();
    progress_bar.set_draw_delta(bar_size / 1000);

    progress_bar
}
