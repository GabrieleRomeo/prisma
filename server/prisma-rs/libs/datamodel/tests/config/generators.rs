use crate::common::ErrorAsserts;
use datamodel::errors::ValidationError;

const DATAMODEL: &str = r#"
generator js1 {
    provider = "javascript"
    output = "../../js"
}

generator go {
    provider = "go"
    platforms = ["a", "b"]
    pinnedPlatform = "b"
}"#;

#[test]
fn serialize_generators_to_cmf() {
    let config = datamodel::load_configuration(DATAMODEL).unwrap();
    let rendered = datamodel::generators_to_json(&config.generators);

    let expected = r#"[
  {
    "name": "js1",
    "provider": "javascript",
    "output": "../../js",
    "platforms": [],
    "pinnedPlatform": null,
    "config": {}
  },
  {
    "name": "go",
    "provider": "go",
    "output": null,
    "platforms": ["a","b"],
    "pinnedPlatform": "b",
    "config": {}
  }
]"#;

    print!("{}", &rendered);

    assert_eq_json(&rendered, expected);
}

fn assert_eq_json(a: &str, b: &str) {
    let json_a: serde_json::Value = serde_json::from_str(a).expect("The String a was not valid JSON.");
    let json_b: serde_json::Value = serde_json::from_str(b).expect("The String b was not valid JSON.");

    assert_eq!(json_a, json_b);
}

const INVALID_DATAMODEL: &str = r#"
generator js1 {
    no_provider = "javascript"
    output = "../../js"
}
"#;

#[test]
fn fail_to_load_generator_with_options_missing() {
    let res = datamodel::load_configuration(INVALID_DATAMODEL);

    if let Err(error) = res {
        error.assert_is(ValidationError::GeneratorArgumentNotFound {
            argument_name: String::from("provider"),
            generator_name: String::from("js1"),
            span: datamodel::ast::Span::new(1, 73),
        });
    } else {
        panic!("Expected error.")
    }
}