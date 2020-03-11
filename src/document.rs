use fehler::throws;
use std::io::prelude::*;

#[derive(Default)]
pub struct Document {
}

impl Document {
    /// Parse a document from string.
    #[throws(_)]
    pub fn parse(input: &str) -> Self {
        let doc = Self::default();
        doc
    }

    /// Write the resulting html into the IO stream.
    #[throws(_)]
    pub fn to_writer<W: Write>(&self, writer: &mut W) {
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
