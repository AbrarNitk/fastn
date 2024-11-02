pub fn import(
    source: &str,
    section: fastn_lang::section::Section,
    document: &mut fastn_lang::unresolved::Document,
) -> Option<fastn_lang::unresolved::Import> {
    if let Some(kind) = section.init.name.kind {
        document.errors.push(fastn_lang::Error::ImportCantHaveType);
    }
    // section.name must be exactly import.
    // section.caption must be single text block, parsable as a module-name.
    //       module-name must be internally able to handle aliasing.
    // only two headers allowed: exports and exposing, parse them.
    // ensure there are no children or body.
    todo!()
}