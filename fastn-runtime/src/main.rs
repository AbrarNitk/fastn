fn main() {
    let c = fastn_resolved::ComponentInvocation {
        id: None,
        name: "ftd#text".to_string(),
        properties: vec![fastn_resolved::Property {
            value: fastn_resolved::Value::new_string("Hello World!").into_property_value(false, 0),
            source: Default::default(),
            condition: None,
            line_number: 0,
        }], // add hello-world caption etc.
        iteration: Box::new(None),
        condition: Box::new(None),
        events: vec![],
        children: vec![],
        source: Default::default(),
        line_number: 0,
    };

    std::fs::write(std::path::PathBuf::from("../output.html"), html_str).unwrap();

    // this main should create a HTML file, and store it in current folder as index.html etc.
}
