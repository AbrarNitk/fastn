pub struct Scanner {
    // source: String,
    tokens: Vec<(fastn_p1::Token, fastn_p1::Span)>,
    index: usize,
    pub output: fastn_p1::ParseOutput,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        use logos::Logos;
        Scanner {
            tokens: fastn_p1::Token::lexer(&source)
                .spanned()
                .map(|(r, span)| (r.unwrap(), span))
                .collect(),
            index: 0,
            output: fastn_p1::ParseOutput::default(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.index >= self.tokens.len()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn reset(&mut self, index: usize) {
        self.index = index;
    }

    pub fn next_is(&self, token: fastn_p1::Token) -> bool {
        self.peek().map(|(t, _)| t == token).unwrap_or(false)
    }

    pub fn peek(&self) -> Option<(fastn_p1::Token, fastn_p1::Span)> {
        self.tokens.get(self.index).map(|v| v.to_owned())
    }

    pub fn pop(&mut self) -> Option<(fastn_p1::Token, fastn_p1::Span)> {
        match self.tokens.get(self.index) {
            Some(t) => {
                self.index += 1;
                Some(t.to_owned())
            }
            None => None,
        }
    }

    pub fn take(&mut self, token: fastn_p1::Token) -> Option<fastn_p1::Span> {
        if let Some((t, s)) = self.peek() {
            if t == token {
                self.pop();
                return Some(s);
            }
        }
        None
    }

    // this uses fn extend_range(a: &mut fastn_p1::Span, b: fastn_p1::Span) to extend the spans
    pub fn take_consecutive(&mut self, token: fastn_p1::Token) -> Option<fastn_p1::Span> {
        let mut span = self.take(token)?;
        while let Some(s) = self.take(token) {
            fastn_p1::parser_v3::utils::extend_range(&mut span, s);
        }
        Some(span)
    }

    pub fn one_of(
        &mut self,
        tokens: &[fastn_p1::Token],
    ) -> Option<(fastn_p1::Token, fastn_p1::Span)> {
        if let Some((t, s)) = self.peek() {
            if tokens.contains(&t) {
                return self.pop();
            }
        }
        None
    }

    // eats up all comment lines and empty lines
    pub fn gobble(&mut self) -> bool {
        let mut found = false;
        while let Some(_) = self.one_of(&[fastn_p1::Token::CommentLine, fastn_p1::Token::EmptyLine])
        {
            // TODO: we have to store comments here
            found = true;
        }
        found
    }
}
