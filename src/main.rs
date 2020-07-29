use argh::FromArgs;

use aikot::cmd;

#[derive(FromArgs, Debug)]
#[argh(description = "Aikot password manager")]
struct AikotCommand {
    #[argh(subcommand)]
    subcmd: AikotSubcommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum AikotSubcommand {
    Clip(ClipCommand),
    List(ListCommand),
    Show(ShowCommand),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "clip", description = "Copy password to clipboard")]
struct ClipCommand {
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "list", description = "List passwords")]
struct ListCommand {}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "show", description = "Show existing password")]
struct ShowCommand {
    #[argh(positional)]
    name: String,
}

fn main() {
    let cmd: AikotCommand = argh::from_env();
    let result = match cmd.subcmd {
        AikotSubcommand::Clip(ClipCommand { name }) => cmd::cmd_clip(&name),
        AikotSubcommand::List(_) => cmd::cmd_list(),
        AikotSubcommand::Show(ShowCommand { name }) => cmd::cmd_show(&name),
    };
    if let Err(err) = result {
        println!("{}", err);
    }
}
