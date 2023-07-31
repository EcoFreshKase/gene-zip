/*
Contains code to represent a FastaHeader
*/

use std::fmt;
use std::fmt::{Display, Formatter};

// Represent a FastaHeader
pub struct FastaHeader {
    header_options: Vec<FastaCustomizerOption>
}

impl FastaHeader {
    fn new() -> FastaHeader {
        FastaHeader { header_options: Vec::new() }
    }

    /// Push a FastaCustomizerOption to the FastaHeader
    fn push_option(&mut self, opt: FastaCustomizerOption) {
        self.header_options.push(opt)
    }
}

impl Display for FastaHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut output = ">".to_string();
        for option in &self.header_options {
            output.push_str(format!("{option}|").as_str());
        }
        write!(f, "{output}")
    }
}

/// A single Option to customize the FASTA-Header
///    FileName(String), The name of the file
///    FileExt(String), The file extension
///    FileSize(u64), The original size of the file
///    SeqLen(String), The amount of Nucleotides in the sequence
///    CustomMsg(String), Custom messages created by the user
#[derive(Clone)]
pub enum FastaCustomizerOption {
    FileName(String), // The name of the file
    FileExt(String), // The file extension
    FileSize(u64), // The original size of the file
    SeqLen(u64), // The amount of Nucleotides in the sequence
    CustomMsg(String), // Custom messages created by the user
}

impl Display for FastaCustomizerOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let output = match self {
            FastaCustomizerOption::FileName(str) => str.to_owned(),
            FastaCustomizerOption::FileExt(str) => str.to_owned(),
            FastaCustomizerOption::FileSize(size) => size.to_string(),
            FastaCustomizerOption::SeqLen(size) => size.to_string(),
            FastaCustomizerOption::CustomMsg(str) => str.to_owned(),
        };
        
        write!(f, "{output}")
    }
}

#[cfg(test)]
mod test {
    use super::FastaHeader;
    use super::FastaCustomizerOption;

    #[test]
    fn fasta_header_test() {
        let mut header = FastaHeader::new();

        header.push_option(FastaCustomizerOption::FileName("test_file".to_string()));
        header.push_option(FastaCustomizerOption::FileExt(".txt".to_string()));
        header.push_option(FastaCustomizerOption::FileSize(100));
        header.push_option(FastaCustomizerOption::SeqLen(50));
        header.push_option(FastaCustomizerOption::CustomMsg("This is a custom message".to_string()));
        header.push_option(FastaCustomizerOption::CustomMsg("Hello World".to_string()));

        assert_eq!(header.to_string(), ">test_file|.txt|100|50|This is a custom message|Hello World|");
    }
}