use std::fs;
use std::io::{self, Write};
use anyhow::{Result, anyhow};
use pulldown_cmark::{Parser, html};

fn main() -> Result<()> {
    loop {
        println!("Options:");
        println!("1. Create a new note");
        println!("2. Read a note");
        println!("3. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => create_note()?,
            "2" => read_note()?,
            "3" => break,
            _ => println!("Invalid option"),
        }
    }
    Ok(())
}

fn create_note() -> Result<()> {
    println!("Enter the note content (press Ctrl+D when finished):");

    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;

    let filename = format!("note_{}.md", chrono::Utc::now().timestamp());

    fs::write(&filename, content)?;

    println!("Note saved as {}", filename);
    Ok(())
}

fn read_note() -> Result<()> {
    println!("Enter the note filename:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let filename = input.trim();

    let content = fs::read_to_string(filename)?;

    let parser = Parser::new(&content);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    println!("Note content (rendered as HTML):\n{}", html_output);
    Ok(())
}
