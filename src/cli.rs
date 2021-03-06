use clap::{AppSettings, ArgEnum, ArgGroup, Clap};
use std::ops::Not;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(about = "Create a new Minecraft server")]
    Init,

    #[clap(about = "Run the server")]
    Run(Run),

    #[clap(about = "Print the server information")]
    Info,

    #[clap(about = "Update something to specific/latest version")]
    Update(Update),

    #[clap(about = "add something", group = ArgGroup::new("adding_search").required(true))]
    Add(Add),

    #[clap(
        about = "Check wether the server has been configured properly"
    )]
    Check(Check),

    #[clap(
        about = "Apply your modifications of configuration file. Update/install/uninstall all binary files to the version of configuration"
    )]
    Refresh,
}

#[derive(Debug, Clap)]
struct Add {
    #[clap(
        name = "KEYWORDS",
        about = "Search a mod with some key words",
        group = "adding_search"
    )]
    search: Option<String>,

    #[clap(
        short,
        name = "ID",
        about = "Add a mod directly with the specific project id",
        group = "adding_search"
    )]
    id: Option<i32>,

    #[clap(
        arg_enum,
        short,
        name = "SOURCE",
        about = "Set the source which you would like to download mods from",
        default_value = "cf"
    )]
    source: ModSource,
}

#[derive(ArgEnum, Debug)]
enum ModSource {
    CF,
}

#[derive(Debug, Clap)]
struct Run {
    #[clap(long = "no-check", parse(from_flag = Not::not))]
    check: bool,
}

#[derive(Debug, Clap)]
struct Update {
    #[clap(short, long, about = "Update server core to latest version")]
    core: bool,

    #[clap(
        short,
        long,
        about = "Update specific mod to latest version",
        value_name = "MOD_NAME",
        conflicts_with = "all_mod"
    )]
    _mod: Vec<String>,

    #[clap(long, about = "Update all mods to latest version")]
    all_mod: bool,

    #[clap(short, long, about = "Update peano to latest version")]
    peano: bool,
}

#[derive(Debug, Clap)]
struct Check {
    #[clap(long = "no-mod", about = "Skip the check for ALL mods", parse(from_flag = Not::not))]
    _mod: bool,
}
