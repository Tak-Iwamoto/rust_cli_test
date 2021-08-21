use clap::{crate_name, crate_version, App, AppSettings, Arg};

pub fn build_clap_app() -> App<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::HidePossibleValuesInHelp)
        .global_setting(AppSettings::ArgsNegateSubcommands)
        .global_setting(AppSettings::AllowExternalSubcommands)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .global_setting(AppSettings::VersionlessSubcommands)
        .about("test cli tools written in rust")
        .arg(Arg::new("show_all"))
        .arg(Arg::new("out").long("output").takes_value(true))
        .arg(Arg::new("debug").short('d').multiple(true))
        .arg(Arg::new("cfg").short('c').takes_value(true))
        .arg(Arg::new("term_width").short('t').takes_value(true))
        .subcommand(
            App::new("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .about("print debug info verbosely"),
                ),
        )
}
