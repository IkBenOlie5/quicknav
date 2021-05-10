use anyhow::anyhow;
use anyhow::Result;
use colored::*;

pub fn to_bool(string: String) -> Result<bool> {
    match string.to_lowercase() {
        "true" => return Ok(true),
        "false" => return Ok(false),
        _ => {
            return Err(anyhow!(format!(
                "{} The argument {} is not of type bool. The value must be true or false.",
                "error:".red(),
                string
            )))
        }
    }
}
