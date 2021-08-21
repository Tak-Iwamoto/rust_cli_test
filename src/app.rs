use anyhow::Result;
use clap::ArgMatches;

use crate::{clap_app, config::Config};

pub struct CliApp {
    pub matches: ArgMatches,
}

impl CliApp {
    pub fn new() -> Result<Self> {
        let cli_args = std::env::args();

        let matches = clap_app::build_clap_app().get_matches_from(cli_args);

        Ok(CliApp { matches })
    }

    pub fn config(&self) -> Result<Config> {
        let show_all = self.matches.is_present("show_all");
        let term_width = match self.matches.value_of("term_width") {
            Some(s) => s.parse::<i32>().unwrap(),
            // default size
            None => 50,
        };
        Ok(Config {
            show_all,
            term_width,
        })
    }
}
