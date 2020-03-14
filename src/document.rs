use fehler::throws;
use kuchiki::traits::TendrilSink;
use kuchiki::{parse_html, NodeRef};
use lazy_static::lazy_static;
use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;
use regex::Regex;
use std::io;
use std::io::prelude::*;

// Make sure it's rebuild when grammar changed.
const _: &[u8] = include_bytes!("grammar.pest");

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Grammar;

fn clean_whitespace(string: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }
    RE.replace(string.trim(), " ").to_string()
}

fn new_element<T: AsRef<str>, C: Into<String>>(tag: T, content: C) -> NodeRef {
    use html5ever::{LocalName, QualName};
    use markup5ever::{namespace_url, ns};
    let name = QualName::new(None, ns!(html), LocalName::from(tag.as_ref()));
    let element = NodeRef::new_element(name, std::iter::empty());
    element.append(NodeRef::new_text(content));
    element
}

pub struct Document {
    dom: NodeRef,
}

impl Document {
    /// Parse and process a document from string.
    #[throws(_)]
    pub fn parse(input: &str) -> Self {
        let mut doc = Self {
            dom: NodeRef::new_document(),
        };
        doc.process(input)?;
        doc.post_process()?;
        doc
    }

    #[doc(hidden)]
    #[throws(_)]
    pub fn parse_without_post_process(input: &str) -> Self {
        let mut doc = Self {
            dom: NodeRef::new_document(),
        };
        doc.process(input)?;
        doc
    }

    #[throws(_)]
    fn process(&mut self, input: &str) {
        let pairs = Grammar::parse(Rule::grammar, &input)?;
        for pair in pairs {
            match pair.as_rule() {
                Rule::header => {
                    self.add_header(pair.into_inner())?;
                }
                Rule::paragraph => {
                    self.add_paragraph(pair.into_inner())?;
                }
                Rule::tag => {
                    self.add_html(pair.as_str())?;
                }
                _ => panic!("unexpceted token {:#?}", pair),
            }
        }
    }

    #[throws(_)]
    fn post_process(&mut self) {
        for node_data in self.dom.select("h2").unwrap() {
            let element = node_data.as_node().as_element().unwrap();
            element
                .attributes
                .borrow_mut()
                .insert("class", "subtitle".to_owned());
        }
    }

    /// Write the resulting html into the IO stream.
    #[throws(_)]
    pub fn to_writer<W: Write>(&self, writer: &mut W) {
        for node in self.dom.children() {
            node.serialize(writer)?;
            writer.write_all(&[b'\n'])?;
        }
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
                    if token.starts_with('-') {
                        tag = "h2".to_owned();
                    }
                }
                Rule::line => {
                    text = clean_whitespace(token);
                }
                _ => panic!("unexpceted token {:#?}", pair),
            }
        }
        self.dom.append(new_element(tag, text));
    }

    #[throws(_)]
    fn add_paragraph<'i>(&mut self, lines: Pairs<'i, Rule>) {
        let lines: Vec<String> = lines.map(|line| clean_whitespace(line.as_str())).collect();
        let paragraph = lines.join(" ");
        self.dom.append(new_element("p", paragraph));
    }

    #[throws(_)]
    fn add_html(&mut self, mut html: &str) {
        let doc = parse_html()
            .from_utf8()
            .read_from(&mut io::Cursor::new(&mut html))?;
        for head_body in doc.first_child().unwrap().children() {
            for element in head_body.children() {
                self.dom.append(element);
            }
        }
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
