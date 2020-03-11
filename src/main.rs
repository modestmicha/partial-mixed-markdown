use anyhow::Error;
use fehler::throws;
use std::io;
use std::io::prelude::*;

mod document;
#[cfg(test)]
mod tests;

pub use crate::document::Document;

#[throws(_)]
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let doc = Document::parse(&input)?;
    doc.to_writer(&mut io::stdout())?;
}
