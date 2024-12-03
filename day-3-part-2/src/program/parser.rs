mod expressions;
mod token;

use nom::{
    branch::alt,
    character::complete::anychar,
    multi::{many1, many_till},
    IResult, Parser,
};

use super::Expression;
use expressions::*;

pub(super) fn parse(input: &str) -> IResult<&str, Vec<Expression>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

fn instruction(input: &str) -> IResult<&str, Expression> {
    alt((enable, disable, mul))(input)
}
