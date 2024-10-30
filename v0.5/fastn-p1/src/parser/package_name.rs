pub fn package_name(scanner: &mut fastn_p1::parser::Scanner) -> Option<fastn_p1::PackageName> {
    let first = scanner.peek()?;
    if !first.is_alphabetic() {
        return None;
    }

    let span = scanner.take_while(|c| c.is_alphanumeric() || c == '.')?;

    let o_name = scanner.source(&span);
    println!("o_name: {:?}", o_name);
    let name = o_name.split_once('.').unwrap_or((o_name, "")).0;

    Some(fastn_p1::PackageName {
        alias: fastn_p1::Span {
            start: span.start,
            end: span.start + name.len(),
        },
        name: span,
    })
}

#[cfg(test)]
mod test {
    fastn_p1::tt!(super::package_name);

    #[test]
    fn package_name() {
        t!(" foo.com", null, " foo.com");
        t!("foo.com", "foo.com");
        t!("foo.com ", "foo.com", " ");
    }
}
