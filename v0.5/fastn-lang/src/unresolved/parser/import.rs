pub(super) fn import(
    _source: &str,
    section: fastn_section::Section,
    _document: &mut fastn_lang::unresolved::Document,
) {
    if let Some(_kind) = section.init.name.kind {
        // document.errors.push(fastn_section::Error::ImportCantHaveType);
        todo!()
    }
    // section.name must be exactly import.
    // section.caption must be single text block, parsable as a module-name.
    //       module-name must be internally able to handle aliasing.
    // only two headers allowed: exports and exposing, unresolved them.
    // ensure there are no children or body.
    todo!()
}
