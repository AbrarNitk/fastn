pub struct ResolutionOutput {
    pub stuck_on: std::collections::HashSet<fastn_unresolved::SymbolName>,
    pub errors: Vec<fastn_section::Spanned<fastn_section::Error>>,
    pub warnings: Vec<fastn_section::Spanned<fastn_section::Warning>>,
    pub comments: Vec<fastn_section::Span>,
}

pub struct ResolutionInput<'a> {
    pub bag: &'a std::collections::HashMap<
        string_interner::DefaultSymbol,
        fastn_unresolved::LookupResult,
    >,
    pub auto_imports: &'a [fastn_section::AutoImport],
    // TODO: use interned string instead of String below
    pub builtins: &'a indexmap::IndexMap<String, fastn_resolved::Definition>,
}

impl fastn_unresolved::ComponentInvocation {
    pub fn resolve(&mut self, _input: ResolutionInput<'_>) -> ResolutionOutput {
        todo!()
    }
}

impl fastn_unresolved::Definition {
    pub fn resolve(&mut self, _input: ResolutionInput<'_>) -> ResolutionOutput {
        todo!()
    }
}
