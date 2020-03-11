use fehler::throws;
use htmlescape::encode_minimal;
use lazy_static::lazy_static;
use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;
use regex::Regex;
use std::io::prelude::*;

// Make sure it's rebuild when grammar changed.
const _: &[u8] = include_bytes!("grammar.pest");

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Grammar;

fn escape(string: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }
    encode_minimal(&RE.replace(string.trim(), " ").to_string())
}

#[derive(Default)]
pub struct Document {
    output: Vec<u8>,
}

impl Document {
    /// Parse a document from string.
    #[throws(_)]
    pub fn parse(input: &str) -> Self {
        let mut doc = Self::default();
        let pairs = Grammar::parse(Rule::grammar, &input)?;
        for pair in pairs {
            match pair.as_rule() {
                Rule::header => {
                    doc.add_header(pair.into_inner())?;
                }
                Rule::paragraph => {
                    doc.add_paragraph(pair.into_inner())?;
                }
                Rule::tag => {
                    doc.add_tag(pair.as_str())?;
                }
                _ => panic!("unexpceted token {:#?}", pair),
            }
        }
        doc
    }

    /// Write the resulting html into the IO stream.
    #[throws(_)]
    pub fn to_writer<W: Write>(&self, writer: &mut W) {
        writer.write_all(&self.output)?;
    }

    #[throws(_)]
    fn add_header<'i>(&mut self, pairs: Pairs<'i, Rule>) {
        let mut tag = "h1".to_owned();
        let mut text = String::new();
        for pair in pairs {
            let token = pair.as_str();
            match pair.as_rule() {
                Rule::atx_header_kind => {
                    tag = format!("h{}", token.len());
                }
                Rule::settext_header_kind => {
                    if token.chars().next().unwrap() == '-' {
                        tag = "h2".to_owned();
                    }
                }
                Rule::line => {
                    text = escape(token);
                }
                _ => panic!("unexpceted token {:#?}", pair),
            }
        }
        self.output
            .write_all(format!("<{}>{}</{}>\n", tag, text, tag).as_bytes())?;
    }

    #[throws(_)]
    fn add_paragraph<'i>(&mut self, lines: Pairs<'i, Rule>) {
        let lines: Vec<String> = lines.map(|line| escape(line.as_str())).collect();
        let paragraph = lines.join(" ");
        self.output
            .write_all(format!("<p>{}</p>\n", paragraph).as_bytes())?;
    }

    #[throws(_)]
    fn add_tag<'i>(&mut self, tag: &str) {
        self.output.write_all(tag.as_bytes())?;
        self.output.write_all(b"\n")?;
    }
}

#[cfg(test)]
#[throws(_)]
pub fn test(name: &'static str) {
    use dialoguer::Confirmation;
    use difference::Changeset;
    use std::fs;
    use std::path::PathBuf;
    let mut f = fs::File::open(format!("tests/{}.md", name))?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    let doc = Document::parse(&input)?;
    let mut actual_bytes = Vec::new();
    doc.to_writer(&mut actual_bytes)?;
    let mut expected = String::new();
    let path = PathBuf::from(format!("tests/{}.html", name));
    if path.exists() {
        let mut f = fs::File::open(&path)?;
        f.read_to_string(&mut expected)?;
    }
    let actual = std::str::from_utf8(&actual_bytes).unwrap();
    if expected != actual {
        println!("\n{}", Changeset::new(&expected, actual, "\n"));
        if std::env::var("INTERACTIVE") == Ok("1".to_owned()) {
            if Confirmation::new()
                .with_text("Accept changes?")
                .default(false)
                .interact()?
            {
                let mut f = fs::File::create(format!("tests/{}.html", name))?;
                f.write_all(&actual_bytes)?;
            } else {
                panic!("tests/{}.md", name);
            }
        } else {
            println!("To review changes run:\n");
            println!("  INTERACTIVE=1 cargo test -- --test-threads=1 --nocapture\n");
            panic!("tests/{}.md", name);
        }
    }
}
