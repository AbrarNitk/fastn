mod module_doc;
mod scanner;
mod section;
mod utils;

use module_doc::module_doc;
use section::section;

impl fastn_p1::ParseOutput {
    pub fn parse(name: &str, source: &str) -> fastn_p1::ParseOutput {
        let mut scanner = scanner::Scanner::new(name, source);
        let mut potential_errors: Vec<fastn_p1::Spanned<fastn_p1::SingleError>> = vec![];

        if module_doc(&mut scanner, &mut potential_errors) {
            return scanner.output;
        }
        potential_errors.clear();

        let mut count = 0;
        while section(&mut scanner, &mut potential_errors) && count < 1000 {
            potential_errors.clear();
            count += 1;
        }

        scanner.output
    }
}
