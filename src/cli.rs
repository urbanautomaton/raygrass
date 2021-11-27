extern crate clap;

use crate::config::*;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub fn config() -> Config {
    let cli = Cli::new();

    Config {
        samples: cli.samples(),
        resolution: cli.resolution(),
        time: cli.time(),
        outfile: cli.outfile().to_string(),
    }
}

struct Cli<'a> {
    matches: clap::ArgMatches<'a>,
}

impl<'a> Cli<'a> {
    pub fn new() -> Self {
        let matches: clap::ArgMatches<'a> = clap::App::new(PKG_NAME)
            .version(PKG_VERSION)
            .about(PKG_DESCRIPTION)
            .author(PKG_AUTHORS)
            .arg(
                clap::Arg::with_name("OUTFILE")
                    .help("The output filename")
                    .required(true)
                    .index(1),
            )
            .arg(
                clap::Arg::with_name("samples")
                    .short("s")
                    .long("samples")
                    .value_name("SAMPLES")
                    .help("The number of samples per pixel (default: 100)")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("resolution")
                    .short("r")
                    .long("resolution")
                    .value_name("RESOLUTION")
                    .help("The size of the image (WxH, default: 1600x1200)")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("time")
                    .short("t")
                    .long("time")
                    .value_name("TIME")
                    .help("The simulation time (seconds, default: 0.0)")
                    .takes_value(true),
            )
            .get_matches();

        Self { matches }
    }

    pub fn samples(&self) -> u32 {
        let val = self.matches.value_of("samples").unwrap_or("100");

        val.parse().unwrap_or_else(|_| {
            println!("Invalid samples value '{}'", val);
            100
        })
    }

    pub fn resolution(&self) -> (u32, u32) {
        let val = self.matches.value_of("resolution").unwrap_or("1600x1200");
        let vals: Vec<&str> = val.split('x').collect();

        if vals.len() != 2 {
            println!("Invalid resolution '{}'", val);
            return (1600, 1200);
        }

        let x: u32 = vals[0].parse().unwrap_or_else(|_| {
            println!("Invalid resolution '{}'", val);
            1600
        });
        let y: u32 = vals[1].parse().unwrap_or_else(|_| {
            println!("Invalid x value '{}'", val);
            1200
        });

        (x, y)
    }

    pub fn time(&self) -> f64 {
        let val = self.matches.value_of("time").unwrap_or("0.0");

        val.parse().unwrap_or_else(|_| {
            println!("Invalid time value '{}'", val);
            0.0
        })
    }

    pub fn outfile(&self) -> &str {
        self.matches.value_of("OUTFILE").unwrap()
    }
}
