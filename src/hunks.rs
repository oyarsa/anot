use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::process::Command;

/// Get modified or added lines from git diff.
///
/// # Arguments
/// * `file_path`: Path to file to find modified lines.
///
/// # Returns
/// Set of line numbers that were added or modified. If there's an error getting the diff from git,
/// returns an empty set.
pub fn get_modified_line_numbers(file_path: impl AsRef<Path>) -> HashSet<usize> {
    let file_path = file_path.as_ref();
    // Fall back to "." if there's no parent
    let parent_dir = file_path.parent().unwrap_or_else(|| Path::new("."));

    // Try running the git command in the parent directory
    let output = match Command::new("git")
        .current_dir(parent_dir)
        .args(["diff", "--unified=0"])
        .arg(file_path)
        .output()
    {
        Ok(out) if out.status.success() => out,
        _ => return HashSet::new(),
    };

    let diff_output = match String::from_utf8(output.stdout) {
        Ok(diff) => diff,
        Err(_) => return HashSet::new(),
    };

    // Regex capturing: @@ -<old> +<start>(,<count>)? @@
    // Group 1 = start, Group 2 = count (optional).
    let re = Regex::new(r"@@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? @@").unwrap();

    re.captures_iter(&diff_output)
        .filter_map(|caps| {
            let start: usize = caps.get(1)?.as_str().parse().ok()?;
            let count: usize = caps
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(1);
            Some((start, count))
        })
        .flat_map(|(start, count)| start..(start + count))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_modified_lines() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;

        // Initialize git repo
        Command::new("git")
            .args(["init"])
            .current_dir(&dir)
            .output()?;

        // Create and commit initial file
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path)?;
        writeln!(file, "line 1\nline 2\nline 3")?;

        Command::new("git")
            .args(["add", "test.txt"])
            .current_dir(&dir)
            .output()?;

        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(&dir)
            .output()?;

        // Modify the file:
        // - modify line 2
        // - add line 4
        // - add line 5
        let mut file = File::create(&file_path)?;
        writeln!(
            file,
            "line 1\nmodified line 2\nline 3\nnew line 4\nnew line 5"
        )?;

        let modified_lines = get_modified_line_numbers(&file_path);

        // Line 1: unchanged
        assert!(!modified_lines.contains(&1));
        // Line 2: modified
        assert!(modified_lines.contains(&2));
        // Line 3: unchanged
        assert!(!modified_lines.contains(&3));
        // Line 4 and 5: new
        assert!(modified_lines.contains(&4));
        assert!(modified_lines.contains(&5));

        Ok(())
    }
}
