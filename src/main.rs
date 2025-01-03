// rcli csv -i input.csv -o output.json --header -d ','

use std::fs;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rlic::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_http_serve, process_text_key_generate, process_text_sign, process_text_verify,
    Base64SubCommand, CmdExecutor, HttpSubCommand, Opts, SubCommand, TextSubCommand,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.cmd.execute().await
    // match opts.cmd {
    //     SubCommand::Csv(opts) => opts.execute().await?,
    //     SubCommand::GenPass(opts) => opts.execute().await?,
    //     SubCommand::Base64(cmd) => cmd.execute().await?,
    //     SubCommand::Text(cmd) => cmd.execute().await?,
    //     SubCommand::HTTP(cmd) => cmd.execute().await?,
    // }
    // Ok(())
}

// 可以看到main中对每一个subcommand都做了分支处理，因此可以定一个CmdExecutor trait 来执行
