use nom::{
    character::complete::{self},
    combinator::value,
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};

use super::{token, Expression};

pub(super) fn enable(input: &str) -> IResult<&str, Expression> {
    value(
        Expression::Enable,
        terminated(token::r#do, pair(token::param_start, token::param_end)),
    )(input)
}

pub(super) fn disable(input: &str) -> IResult<&str, Expression> {
    value(
        Expression::Disable,
        terminated(token::dont, pair(token::param_start, token::param_end)),
    )(input)
}

pub(super) fn mul(input: &str) -> IResult<&str, Expression> {
    let (input, _) = token::mul(input)?;
    let (input, pair) = delimited(
        token::param_start,
        separated_pair(complete::i32, token::param_separator, complete::i32),
        token::param_end,
    )(input)?;
    Ok((input, Expression::Mul(pair.0, pair.1)))
}
