use doxx::document::{load_document, ImageOptions};
use std::path::Path;

fn load_comments_fixture() -> doxx::document::Document {
    let path = Path::new("tests/fixtures/comments.docx");
    assert!(path.exists(), "comments.docx fixture must exist");
    load_document(path, ImageOptions::default()).expect("Failed to load comments.docx")
}

#[test]
fn test_comments_are_parsed() {
    let doc = load_comments_fixture();
    assert_eq!(doc.comments.len(), 3, "Should parse 3 comments");
}

#[test]
fn test_comment_authors_and_text() {
    let doc = load_comments_fixture();

    let alice_comments: Vec<_> = doc
        .comments
        .iter()
        .filter(|c| c.author == "Alice")
        .collect();
    let bob_comments: Vec<_> = doc.comments.iter().filter(|c| c.author == "Bob").collect();

    assert_eq!(alice_comments.len(), 2, "Alice should have 2 comments");
    assert_eq!(bob_comments.len(), 1, "Bob should have 1 comment");

    assert!(
        alice_comments
            .iter()
            .any(|c| c.text.contains("needs more detail")),
        "Alice's first comment text should be present"
    );
    assert!(
        bob_comments[0].text.contains("expand"),
        "Bob's comment text should be present"
    );
}

#[test]
fn test_comment_reply_has_parent_id() {
    let doc = load_comments_fixture();

    // Comment 2 (Bob's) is a reply to comment 1 (Alice's)
    let reply = doc.comments.iter().find(|c| c.author == "Bob").unwrap();
    assert_eq!(
        reply.parent_id,
        Some(1),
        "Bob's comment should be a reply to comment id 1"
    );
}

#[test]
fn test_root_comments_have_no_parent() {
    let doc = load_comments_fixture();
    let roots: Vec<_> = doc
        .comments
        .iter()
        .filter(|c| c.parent_id.is_none())
        .collect();
    assert_eq!(roots.len(), 2, "Should have 2 root comments");
}

#[test]
fn test_comment_anchors_are_set() {
    let doc = load_comments_fixture();
    let anchored: Vec<_> = doc
        .comments
        .iter()
        .filter(|c| c.anchor_element_index.is_some())
        .collect();
    assert!(
        !anchored.is_empty(),
        "At least some comments should have anchor element indices"
    );
}

#[test]
fn test_document_without_comments_has_empty_list() {
    let path = Path::new("tests/fixtures/minimal.docx");
    let doc = load_document(path, ImageOptions::default()).expect("Failed to load minimal.docx");
    assert!(
        doc.comments.is_empty(),
        "minimal.docx should have no comments"
    );
}
