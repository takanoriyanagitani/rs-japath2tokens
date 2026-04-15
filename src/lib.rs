use std::io;

use io::BufWriter;
use io::Write;

use io::BufRead;

use serde::ser::{Serialize, SerializeSeq, Serializer};

use lindera::token::Token;

use lindera::dictionary::Dictionary;
use lindera::dictionary::load_dictionary;

use lindera::mode::Mode;
use lindera::segmenter::Segmenter;

pub struct Tokens<'a>(pub &'a [Token<'a>]);

impl<'a> Serialize for Tokens<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq: S::SerializeSeq = serializer.serialize_seq(Some(self.0.len()))?;

        for tok in self.0 {
            let sur: &str = &tok.surface;
            seq.serialize_element(sur)?;
        }

        seq.end()
    }
}

impl<'a> Tokens<'a> {
    pub fn to_writer<W>(&self, wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        serde_json::to_writer(wtr.by_ref(), self)?;
        writeln!(wtr)?;
        Ok(())
    }
}

pub struct Tokenizer(pub lindera::tokenizer::Tokenizer);

impl Tokenizer {
    pub fn path2tokens2json2writer<W>(&self, filepath: &str, mut wtr: W) -> Result<(), io::Error>
    where
        W: Write,
    {
        let toks: Vec<Token> = self.0.tokenize(filepath).map_err(io::Error::other)?;
        Tokens(&toks).to_writer(&mut wtr)
    }
}

impl Tokenizer {
    pub fn paths2tokens2writer<I, W>(&self, paths: I, mut wtr: W) -> Result<(), io::Error>
    where
        I: Iterator<Item = Result<String, io::Error>>,
        W: Write,
    {
        for rline in paths {
            let filepath: String = rline?;
            self.path2tokens2json2writer(&filepath, &mut wtr)?;
        }
        wtr.flush()
    }
}

impl Tokenizer {
    pub fn stdin2paths2tokens2stdout(&self) -> Result<(), io::Error> {
        let o = io::stdout();
        let mut ol = o.lock();
        self.paths2tokens2writer(io::stdin().lock().lines(), BufWriter::new(&mut ol))?;
        ol.flush()
    }
}

impl Tokenizer {
    pub fn new_default() -> Result<Self, io::Error> {
        let dict: Dictionary = load_dictionary("embedded://ipadic").map_err(io::Error::other)?;
        let segm: Segmenter = Segmenter::new(Mode::Normal, dict, None);
        Ok(Self(lindera::tokenizer::Tokenizer::new(segm)))
    }
}

pub fn stdin2paths2tokens2stdout() -> Result<(), io::Error> {
    let tok: Tokenizer = Tokenizer::new_default()?;
    tok.stdin2paths2tokens2stdout()
}
