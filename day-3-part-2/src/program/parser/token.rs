macro_rules! define_token {
    ($name:ident, $tag:literal) => {
        pub(super) fn $name<'i, 'o>(input: &'i str) -> nom::IResult<&'i str, &'o str>
        where
            'i: 'o,
            'o: 'i,
        {
            nom::bytes::complete::tag($tag)(input)
        }
    };
    () => {};
}
define_token!(mul, "mul");
define_token!(r#do, "do");
define_token!(dont, "don't");
define_token!(param_start, "(");
define_token!(param_end, ")");
define_token!(param_separator, ",");
