use crate::cli::OutputFormat;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() 使用 headers 迭代器
        // record.iter() 使用 record的迭代器
        // zip() 将两个迭代器合并为一个元祖的迭代器[(header, record), ..]
        // collect::<Value>() 将元祖的迭代器转换为JSON Value
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => toml::to_string(&ret)?, // TODO: 暂不支持toml
    };
    fs::write(output, content)?; // => ()
    Ok(())
}
