use std::{fmt, str::FromStr};

use clap::Parser;

use crate::CmdExecutor;

use super::verify_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 to string")]
    Decode(Base64DecodeOpts),
}
impl CmdExecutor for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long,value_parser=verify_file, default_value="-")]
    pub input: String,
    #[arg(long,value_parser= paser_base64_format,default_value="standard")]
    pub format: Base64Format,
}
impl CmdExecutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let ret = crate::process_encode(&mut reader, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,value_parser=verify_file, default_value="-")]
    pub input: String,
    #[arg(long,value_parser= paser_base64_format,default_value="standard")]
    pub format: Base64Format,
}
impl CmdExecutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let ret = crate::process_decode(&mut reader, self.format)?;
        println!("{:?}", ret);
        Ok(())
    }
}
#[derive(Debug, Clone, Parser, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn paser_base64_format(format: &str) -> anyhow::Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
