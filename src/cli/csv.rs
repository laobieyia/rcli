use std::{fmt, str::FromStr};

use clap::Parser;
use tracing_subscriber::fmt::format;

use crate::CmdExecutor;

use super::verify_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}
#[derive(Parser, Debug)]
pub struct CsvOpts {
    // clap version 3 is about
    // clap version 4 is help
    // value_parser 检查
    #[arg(
        short,
        long,
        value_parser = verify_file,
        help = "Input file path"
    )]
    pub input: String,

    #[arg(short, long, help = "Output file path")]
    // "output.json".into() &str -> String
    // default_value 不适合了，因为output.xx 需要根据format的值来动态生成。
    pub output: Option<String>,
    // 为什么需要value_parse? 是因为OutputFormat是枚举？
    #[arg( long, value_parser=parse_format,default_value = "json",help="Output format")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',', help = "Delimiter")]
    pub delimiter: char,

    #[arg(long, default_value_t = true, help = "Header")]
    pub header: bool,
}
impl CmdExecutor for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", self.format)
        };
        crate::process_csv(&self.input, output, self.format)
    }
}
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
    //::<OutputFormat>
    // match format.to_lowercase().as_str() {
    //     "json" => Ok(OutputFormat::Json),
    //     "yaml" => Ok(OutputFormat::Yaml),
    //     "toml" => Ok(OutputFormat::Toml),
    //     _ => Err("Invalid format"),
    // }
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            v => Err(anyhow::bail!("Unsupported format: {}", v)),
        }
    }
}
// impl TryFrom<&str> for OutputFormat {
//     type Error = anyhow::Error;
//     fn try_from(format: &str) -> Result<Self, Self::Error> {
//         match format.to_lowercase().as_str() {
//             "json" => Ok(OutputFormat::Json),
//             "yaml" => Ok(OutputFormat::Yaml),
//             "toml" => Ok(OutputFormat::Toml),
//             v => anyhow::bail!("Unsupported format: {}", v),
//         }
//     }
// }
// impl From<&str> for OutputFormat {
//     fn from(format: &str) -> Self {
//         match format.to_lowercase().as_str() {
//             "json" => OutputFormat::Json,
//             "yaml" => OutputFormat::Yaml,
//             "toml" => OutputFormat::Toml,
//             _ => unreachable!(),
//         }
//     }
// }

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
