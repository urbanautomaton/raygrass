extern crate clap;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub struct CLI<'a> {
    matches: clap::ArgMatches<'a>,
}

impl<'a> CLI<'a> {
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

    pub fn outfile(&self) -> &str {
        self.matches.value_of("OUTFILE").unwrap()
    }
}
