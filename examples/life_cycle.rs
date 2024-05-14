use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Url<'a> {
    protocol: &'a str,
    host: &'a str,
    path: &'a str,
    query: &'a str,
    fragment: &'a str,
    // fragment: &'static str,
}

impl<'a> Display for Url<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}://{}{}{}{}",
            self.protocol,
            self.host,
            if !self.path.is_empty() {
                format!("{}", self.path)
            } else {
                "".to_string()
            },
            if !self.query.is_empty() {
                format!("?{}", self.query)
            } else {
                "".to_string()
            },
            if !self.fragment.is_empty() {
                format!("#{}", self.fragment)
            } else {
                "".to_string()
            },
        )
    }
}

fn main() {
    let s = "https://rustcc.cn/article?id=019f9937#title";
    println!("{s}");

    let a_url = Url {
        protocol: &s[..5],
        host: &s[8..17],
        path: &s[17..25],
        query: &s[26..37],
        fragment: &s[38..43],
    };
    println!("{a_url}");

    let a_url = Url {
        protocol: "https",
        host: "rustcc.cn",
        path: "/article",
        query: "id=019f9937",
        fragment: "title",
    };
    println!("{a_url}");
}
