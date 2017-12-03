//! Example parser 1
use example::prelude::*;

/// The first example parser
#[derive(Debug)]
pub struct Parser1;

#[derive(Debug, PartialEq)]
/// The result of the first example parser
pub struct Parser1Result;

impl Parsable<()> for Parser1 {
    /// The actual parsing entry point
    fn parse<'a>(&mut self,
                 input: &'a [u8],
                 _: Option<&ParserResultVec>,
                 _: Option<&mut ()>)
                 -> IResult<&'a [u8], ParserResult> {

        do_parse!(input, tag!("1") >> (Box::new(Parser1Result)))
    }
}
