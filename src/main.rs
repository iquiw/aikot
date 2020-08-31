use argh::FromArgs;
use failure::Error;

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
    Clip(ClipCommand),
    Edit(EditCommand),
    List(ListCommand),
    Pwgen(PwgenCommand),
    Show(ShowCommand),
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
        AikotSubcommand::Clip(ClipCommand { name }) => cmd::cmd_clip(&aikot_env, &name),
        AikotSubcommand::Edit(EditCommand { name }) => cmd::cmd_edit(&aikot_env, &name),
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
    }
}
