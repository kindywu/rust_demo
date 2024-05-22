use winnow::combinator::{dispatch, empty, fail, preceded};
use winnow::prelude::*;
use winnow::token::any;

fn escaped(input: &mut &str) -> PResult<char> {
    preceded('\\', escape_seq_char).parse_next(input)
}

fn escape_seq_char(input: &mut &str) -> PResult<char> {
    dispatch! {any;
        'b' => empty.value('\u{8}'),
        'f' => empty.value('\u{c}'),
        'n' => empty.value('\n'),
        'r' => empty.value('\r'),
        't' => empty.value('\t'),
        '\\' => empty.value('\\'),
        '"' => empty.value('"'),
        _ => fail::<_, char, _>,
    }
    .parse_next(input)
}

fn main() {
    let result = escaped.parse_peek("\\\"Hello");
    println!("{result:?}")
}
