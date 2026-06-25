use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

pub(super) struct ReviewRow {
    pub previous: String,
    pub current: String,
    pub count: usize,
    pub doc_count: usize,
    pub previous_qstring: String,
    pub current_qstring: String,
    pub previous_rank: usize,
    pub current_rank: usize,
    pub probability: f64,
    pub examples: Vec<String>,
}

pub(super) fn write_review(path: &Path, rows: &[ReviewRow]) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
        }
    }

    let output = File::create(path).with_context(|| format!("create {}", path.display()))?;
    let mut output = BufWriter::new(output);

    writeln!(
        output,
        "review_status\tprevious\tcurrent\tcount\tdoc_count\tprevious_rank\tcurrent_rank\tprevious_qstring\tcurrent_qstring\tqstring\tprobability\texamples"
    )?;

    for row in rows {
        writeln!(
            output,
            "pending\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            cell(&row.previous),
            cell(&row.current),
            row.count,
            row.doc_count,
            row.previous_rank,
            row.current_rank,
            cell(&row.previous_qstring),
            cell(&row.current_qstring),
            cell(&format!("{} {}", row.previous_qstring, row.current_qstring)),
            row.probability,
            cell(&row.examples.join(" | "))
        )?;
    }

    Ok(())
}

fn cell(value: &str) -> String {
    value
        .replace('\t', " ")
        .replace('\r', " ")
        .replace('\n', " ")
}
