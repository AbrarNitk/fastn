/// example: `list<string> foo` | `foo bar` | `bar`
pub fn section_init(scanner: &mut fastn_p1::parser::Scanner) -> Option<fastn_p1::SectionInit> {
    scanner.skip_spaces();
    let dashdash = scanner.token("--")?;
    scanner.skip_spaces();
    let name = fastn_p1::parser::kinded_name(scanner)?;
    scanner.skip_spaces();
    let colon = scanner.token(":")?;
    Some(fastn_p1::SectionInit {
        dashdash,
        name,
        colon,
    })
}

#[cfg(test)]
mod test {
    fastn_p1::tt!(super::section_init);

    #[test]
    fn section_init() {
        t!("-- foo:", {"name": {"name": "foo"}}, "");
    }
}
