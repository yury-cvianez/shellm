use crate::cli::repl::line::decoder::InputDecoder;
use crate::cli::repl::line::editor::LineEditor;

pub fn process_entry_byte(b: u8) -> std::io::Result<Option<String>> {

    let mut decoder = InputDecoder::new();
    let mut editor = LineEditor::new();

    decoder.feed_byte(b);

    while let Some(event) = decoder.next_event() {
        if let Some(line) = editor.process_input(event) {
            println!("Line entered: {}", line);
            return Ok(Some(line))
        }
    }

    Ok(None)

}