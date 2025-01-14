use anot::{extract_annotations, FileType};
use std::path::PathBuf;

#[test]
fn test_basic_annotation_extraction() {
    let content = std::fs::read_to_string("tests/data/test.py").unwrap();
    let file = PathBuf::from("tests/data/test.py");
    let annotations = extract_annotations(&content, &FileType::Python, &file).unwrap();

    assert_eq!(annotations.len(), 2);

    assert_eq!(annotations[0].kind, "note");
    assert_eq!(
        annotations[0].content,
        "this experiment will be re-written later"
    );
    assert_eq!(annotations[0].location.line, 2);
    assert!(!annotations[0].location.inline);

    assert_eq!(annotations[1].kind, "hypothesis");
    assert_eq!(annotations[1].content, "5 is better than 4");
    assert_eq!(annotations[1].location.line, 5);
    assert!(!annotations[1].location.inline);
}

#[test]
fn test_rust_annotation_extraction() {
    let content = std::fs::read_to_string("tests/data/test.rs").unwrap();
    let file = PathBuf::from("tests/data/test.rs");
    let annotations = extract_annotations(&content, &FileType::Rust, &file).unwrap();

    assert_eq!(annotations.len(), 2);

    assert_eq!(annotations[0].kind, "todo");
    assert_eq!(annotations[0].content, "Add more fields");
    assert_eq!(annotations[0].location.line, 2);
    assert!(!annotations[0].location.inline);

    assert_eq!(annotations[1].kind, "fixme");
    assert_eq!(annotations[1].content, "This needs better error handling");
    assert_eq!(annotations[1].location.line, 9);
    assert!(!annotations[1].location.inline);
}

#[test]
fn test_javascript_annotation_extraction() {
    let content = std::fs::read_to_string("tests/data/test.js").unwrap();
    let file = PathBuf::from("tests/data/test.js");
    let annotations = extract_annotations(&content, &FileType::JavaScript, &file).unwrap();

    assert_eq!(annotations.len(), 2);

    assert_eq!(annotations[0].kind, "todo");
    assert_eq!(annotations[0].content, "Add initialization");
    assert_eq!(annotations[0].location.line, 3);
    assert!(!annotations[0].location.inline);

    assert_eq!(annotations[1].kind, "bug");
    assert_eq!(annotations[1].content, "Sometimes fails on Safari");
    assert_eq!(annotations[1].location.line, 6);
    assert!(!annotations[1].location.inline);
}
