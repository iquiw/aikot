use anyhow::Error;
use argh::FromArgs;

use aikot::cmd;
use aikot::env::AikotEnv;
use aikot::password::PwGen;

#[derive(FromArgs, Debug)]
#[argh(description = "Aikot password manager")]
struct AikotCommand {
    #[argh(subcommand)]
    subcmd: AikotSubcommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum AikotSubcommand {
    Add(AddCommand),
    Browse(BrowseCommand),
    Clip(ClipCommand),
    Edit(EditCommand),
    Init(InitCommand),
    List(ListCommand),
    Pwgen(PwgenCommand),
    Show(ShowCommand),
    Version(VersionCommand),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "add", description = "Add new secret")]
struct AddCommand {
    #[argh(positional)]
    name: String,

    #[argh(positional, default = "24")]
    length: usize,

    #[argh(switch, description = "not to generate password")]
    no_generate: bool,

    #[argh(switch, description = "include symbol characters in password")]
    symbol: bool,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "browse", description = "Browse url of secret")]
struct BrowseCommand {
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "clip", description = "Copy password to clipboard")]
struct ClipCommand {
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "edit", description = "Edit secret by EDITOR")]
struct EditCommand {
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, Debug)]
#[argh(
    subcommand,
    name = "init",
    description = "Initialize new password store"
)]
struct InitCommand {
    #[argh(positional)]
    gpg_ids: Vec<String>,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "list", description = "List secrets")]
struct ListCommand {
    #[argh(positional)]
    pattern: Option<String>,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "pwgen", description = "Generate passwords")]
struct PwgenCommand {
    #[argh(positional, default = "12")]
    length: usize,
    #[argh(
        option,
        description = "number of passwords to be generated",
        default = "8"
    )]
    count: u16,

    #[argh(switch, description = "include symbol characters in password")]
    symbol: bool,
}

#[derive(FromArgs, Debug)]
#[argh(
    subcommand,
    name = "show",
    description = "Display secret contents without password"
)]
struct ShowCommand {
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "version", description = "Print the version")]
struct VersionCommand {}

fn main() {
    if let Err(err) = aikot_main() {
        eprintln!("{}", err);
    }
}

fn aikot_main() -> Result<(), Error> {
    let cmd: AikotCommand = argh::from_env();
    let aikot_env = AikotEnv::from_env()?;
    match cmd.subcmd {
        AikotSubcommand::Add(AddCommand {
            name,
            length,
            no_generate,
            symbol,
        }) => {
            let opwgen = if no_generate {
                None
            } else {
                Some(PwGen::new(length, symbol)?)
            };
            cmd::cmd_add(&aikot_env, &name, opwgen.as_ref())
        }
        AikotSubcommand::Browse(BrowseCommand { name }) => cmd::cmd_browse(&aikot_env, &name),
        AikotSubcommand::Clip(ClipCommand { name }) => cmd::cmd_clip(&aikot_env, &name),
        AikotSubcommand::Edit(EditCommand { name }) => cmd::cmd_edit(&aikot_env, &name),
        AikotSubcommand::Init(InitCommand { gpg_ids }) => cmd::cmd_init(&aikot_env, &gpg_ids),
        AikotSubcommand::List(ListCommand { pattern }) => {
            cmd::cmd_list(&aikot_env, pattern.as_deref())
        }
        AikotSubcommand::Pwgen(PwgenCommand {
            length,
            count,
            symbol,
        }) => {
            let pwgen = PwGen::new(length, symbol)?;
            cmd::cmd_pwgen(&aikot_env, &pwgen, count)
        }
        AikotSubcommand::Show(ShowCommand { name }) => cmd::cmd_show(&aikot_env, &name),
        AikotSubcommand::Version(_) => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}
