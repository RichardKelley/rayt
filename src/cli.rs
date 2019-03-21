use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use failure::Error;
use std::str::FromStr;

pub enum CliCommand {
    RENDER {
        width: u64,
        output_path: String,
        num_of_rays: u64,
        num_of_threads: usize,
    },
    GENERATE,
}

pub struct CliConfig {
    pub command: CliCommand,
    pub config_path: String,
}

#[derive(Debug, Fail)]
enum CliParsingError {
    #[fail(display = "invalid value <{}> for arg <{}>", value, arg)]
    InvalidValue { arg: String, value: String },
}

pub fn get_cli_config() -> Result<CliConfig, Error> {
    let matches = App::new("Ray tracer")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .version(crate_version!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .required(true)
                .help("path to image config yaml"),
        )
        .subcommands(vec![
            SubCommand::with_name("render")
                .about("renders an image")
                .arg(
                    Arg::with_name("width")
                        .short("w")
                        .long("width")
                        .takes_value(true)
                        .required(true)
                        .help("the output image width"),
                )
                .arg(
                    Arg::with_name("output_path")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .required(true)
                        .default_value("image.ppm")
                        .help("the output image path"),
                )
                .arg(
                    Arg::with_name("rays")
                        .short("r")
                        .long("rays")
                        .takes_value(true)
                        .required(true)
                        .default_value("100")
                        .help("the number of rays to generate per pixel"),
                )
                .arg(
                    Arg::with_name("threads")
                        .short("t")
                        .long("threads")
                        .takes_value(true)
                        .required(true)
                        .default_value("4")
                        .help("the number of threads to create for the renderer"),
                ),
            SubCommand::with_name("generate").about("generate a random image config yaml"),
        ])
        .get_matches();

    let config_path = String::from(matches.value_of("config").unwrap());
    ensure!(
        config_path.ends_with(".yaml"),
        "Config path <{}> must end in .yaml",
        config_path,
    );

    if let Some(subcommand) = matches.subcommand_matches("render") {
        let width = parse::<u64>(subcommand, "width")?;
        let output_path = String::from(subcommand.value_of("output_path").unwrap());
        let num_of_rays = parse::<u64>(subcommand, "rays")?;
        let num_of_threads = parse::<usize>(subcommand, "threads")?;

        ensure!(
            output_path.ends_with(".ppm"),
            "Output path <{}> must end in .ppm",
            output_path,
        );

        return Ok(CliConfig {
            command: CliCommand::RENDER {
                width,
                output_path,
                num_of_rays,
                num_of_threads,
            },
            config_path,
        });
    }
    if matches.subcommand_matches("generate").is_some() {
        return Ok(CliConfig {
            command: CliCommand::GENERATE,
            config_path,
        });
    }

    // Clap should have errored before we get here
    panic!("Unable to parse CLI args")
}

fn parse<T: FromStr>(matches: &ArgMatches, arg: &str) -> Result<T, CliParsingError> {
    let raw = matches.value_of(arg).unwrap();
    match raw.parse::<T>() {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(CliParsingError::InvalidValue {
            arg: String::from(arg),
            value: String::from(raw),
        }),
    }
}
