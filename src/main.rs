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
    List(ListCommand),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "list", description = "List passwords")]
struct ListCommand {}

fn main() {
    let cmd: AikotCommand = argh::from_env();
    let result = match cmd.subcmd {
        AikotSubcommand::List(_) => cmd::cmd_list(),
    };
    if let Err(err) = result {
        println!("{}", err);
    }
}
