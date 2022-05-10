use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Features{}

pub fn features(_ : Features ) -> Result<()> {
    println!("Features");
    Ok(())
}