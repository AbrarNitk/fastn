#[derive(Default)]
pub struct SectionProvider {
    cache: std::collections::HashMap<String, Result<fastn_section::Document, fastn_section::Error>>,
}

#[async_trait::async_trait]
impl fastn_continuation::AsyncMutProvider for &mut SectionProvider {
    type Needed = Vec<String>;
    type Found = Vec<(
        String,
        Result<fastn_section::Document, fastn_section::Error>,
    )>;

    async fn provide(&mut self, needed: Vec<String>) -> Self::Found {
        let mut r = vec![];
        for f in needed {
            if let Some(doc) = self.cache.get(&f) {
                r.push((f, doc.clone()));
                continue;
            }

            match tokio::fs::read_to_string(&f).await {
                Ok(v) => {
                    let d = fastn_section::Document::parse(&arcstr::ArcStr::from(v));
                    self.cache.insert(f.clone(), Ok(d.clone()));
                    r.push((f, Ok(d)));
                }
                Err(e) => {
                    todo!("error handler not ready for: {e:?}")
                }
            }
        }
        r
    }
}
