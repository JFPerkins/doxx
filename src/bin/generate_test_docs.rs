use docx_rs::*;
use std::fs::File;

type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    println!("Generating test documents...");

    std::fs::create_dir_all("tests/fixtures")?;

    generate_minimal_doc()?;
    generate_colors_doc()?;
    generate_comments_doc()?;

    println!("All test documents generated successfully!");
    Ok(())
}

fn generate_minimal_doc() -> Result<(), DynError> {
    let doc = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Minimal Test").bold()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
            "This is the smallest possible test document with just a title and one paragraph.",
        )))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
            "This single paragraph tests the most basic document parsing functionality.",
        )));

    let path = "tests/fixtures/minimal.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}

fn generate_colors_doc() -> Result<(), DynError> {
    let doc = Docx::new()
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("Color Formatting Test").bold().size(28)),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Red text ").color("FF0000"))
                .add_run(Run::new().add_text("Green text ").color("008000"))
                .add_run(Run::new().add_text("Blue text ").color("0000FF"))
                .add_run(Run::new().add_text("Orange text ").color("FF8C00"))
                .add_run(Run::new().add_text("Purple text").color("800080")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Bold red").bold().color("FF0000"))
                .add_run(Run::new().add_text(" and "))
                .add_run(Run::new().add_text("italic blue").italic().color("0000FF")),
        )
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new().add_text("This paragraph has no color formatting for contrast."),
            ),
        );

    let path = "tests/fixtures/colors.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}

fn generate_comments_doc() -> Result<(), DynError> {
    let comment1 = Comment::new(1)
        .author("Alice")
        .date("2024-01-15T10:00:00Z")
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("This section needs more detail.")),
        );

    let comment2 = Comment::new(2)
        .author("Bob")
        .date("2024-01-15T11:00:00Z")
        .parent_comment_id(1)
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("Agreed, I will expand this.")),
        );

    let comment3 = Comment::new(3)
        .author("Alice")
        .date("2024-01-15T12:00:00Z")
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("Consider adding a table here.")),
        );

    let para_with_comment1 = Paragraph::new()
        .add_comment_start(comment1)
        .add_run(Run::new().add_text("The introduction covers the main topics briefly."))
        .add_comment_end(1);

    // Reply is anchored to the same paragraph as parent
    let para_with_comment2 = Paragraph::new()
        .add_comment_start(comment2)
        .add_run(Run::new().add_text("More context will be provided in the next revision."))
        .add_comment_end(2);

    let para_with_comment3 = Paragraph::new()
        .add_comment_start(comment3)
        .add_run(Run::new().add_text("The data section summarizes findings from the study."))
        .add_comment_end(3);

    let doc = Docx::new()
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("Comments Test Document").bold()),
        )
        .add_paragraph(para_with_comment1)
        .add_paragraph(para_with_comment2)
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("This paragraph has no comment attached.")),
        )
        .add_paragraph(para_with_comment3);

    let path = "tests/fixtures/comments.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}
