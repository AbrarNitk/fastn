impl fastn::commands::Render {
    pub async fn run(self, config: &mut fastn_core::Config) {
        let route = config.resolve(self.path.as_str()).await;
        match route {
            fastn_core::Route::Document(path, data) => {
                let html = fastn::commands::render::render_document(
                    config.auto_imports.clone(),
                    path.as_str(),
                    data,
                    self.strict,
                )
                .await;
                std::fs::write(path.replace(".ftd", ".html"), html).unwrap();
            }
            _ => todo!(),
        };
    }
}

pub async fn render_document(
    _global_aliases: fastn_unresolved::AliasesSimple,
    _path: &str,
    _data: serde_json::Value,
    _strict: bool,
) -> String {
    todo!()
    // let source = std::fs::File::open(path)
    //     .and_then(std::io::read_to_string)
    //     .unwrap();
    // let o = fastn_compiler::compile(&source, "main", None, global_aliases)
    //     .await
    //     .unwrap();
    //
    // let h = fastn_runtime::HtmlData::from_cd(o);
    // h.to_test_html()
}
