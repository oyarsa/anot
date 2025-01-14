use crate::annotation::{Annotation, Location};
use crate::input::FileType;
use anyhow::Result;
use std::path::Path;

pub const TAG: &str = "@";

pub fn extract_annotations(
    content: &str,
    file_type: &FileType,
    file: &Path,
) -> Result<Vec<Annotation>> {
    let comment_prefix = file_type.comment_prefix();
    let anot_prefix = format!("{} {}", comment_prefix, TAG);
    Ok(content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.contains(&anot_prefix) {
                parse_annotation(line, i + 1, file, false)
            } else if line.contains(&format!("{}{}", comment_prefix, TAG)) {
                // Handle inline comments
                let parts: Vec<&str> = line.split(comment_prefix).collect();
                if let Some(comment) = parts.last() {
                    if comment.trim().starts_with(TAG) {
                        return parse_annotation(comment.trim(), i + 1, file, true);
                    }
                }
                None
            } else {
                None
            }
        })
        .collect())
}

fn parse_annotation(line: &str, line_number: usize, file: &Path, is_inline: bool) -> Option<Annotation> {
    let at_pos = line.find('@')?;
    let colon_pos = line[at_pos..].find(':')?;

    let kind = line[at_pos + 1..at_pos + colon_pos].trim().to_string();
    let content = line[at_pos + colon_pos + 1..].trim().to_string();

    Some(Annotation {
        kind,
        content,
        location: Location {
            file: file.to_path_buf(),
            line: line_number,
            inline: is_inline,
        },
    })
}
