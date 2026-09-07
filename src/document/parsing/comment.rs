//! Comment text extraction

/// Extract plain text from a docx_rs Comment by walking its paragraph children.
///
/// Multiple paragraphs within one comment are joined with a space.
pub(crate) fn extract_comment_text(comment: &docx_rs::Comment) -> String {
    let mut parts = Vec::new();
    for child in &comment.children {
        if let docx_rs::CommentChild::Paragraph(para) = child {
            let mut para_text = String::new();
            for child in &para.children {
                if let docx_rs::ParagraphChild::Run(run) = child {
                    for run_child in &run.children {
                        if let docx_rs::RunChild::Text(t) = run_child {
                            para_text.push_str(&t.text);
                        }
                    }
                }
            }
            if !para_text.is_empty() {
                parts.push(para_text);
            }
        }
    }
    parts.join(" ").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_comment(id: usize, text: &str) -> docx_rs::Comment {
        docx_rs::Comment::new(id)
            .add_paragraph(docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text(text)))
    }

    #[test]
    fn test_extract_single_paragraph() {
        let comment = make_comment(1, "This needs revision.");
        assert_eq!(extract_comment_text(&comment), "This needs revision.");
    }

    #[test]
    fn test_extract_empty_comment() {
        let comment = docx_rs::Comment::new(1);
        assert_eq!(extract_comment_text(&comment), "");
    }

    #[test]
    fn test_extract_multiple_paragraphs() {
        let comment = docx_rs::Comment::new(1)
            .add_paragraph(
                docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text("First paragraph.")),
            )
            .add_paragraph(
                docx_rs::Paragraph::new()
                    .add_run(docx_rs::Run::new().add_text("Second paragraph.")),
            );
        assert_eq!(
            extract_comment_text(&comment),
            "First paragraph. Second paragraph."
        );
    }

    #[test]
    fn test_extract_trims_whitespace() {
        let comment = make_comment(1, "  padded text  ");
        assert_eq!(extract_comment_text(&comment), "padded text");
    }
}
