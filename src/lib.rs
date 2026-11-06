//! SigmaOS-renamer-bulk — bulk file rename engine (Mint `bulky` parity).
//!
//! Rename operations on filename lists: literal/regex-lite find-replace,
//! prefix/suffix, sequential numbering, case transforms, dry-run plan
//! with collision detection. Pure string logic — filesystem ops are
//! the caller's job.

/// A planned rename: old name -> new name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenameOp {
    pub from: String,
    pub to: String,
}

/// Errors that abort a bulk operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenameError {
    /// Two sources would map to the same target name.
    Collision(String),
    /// Target already exists on disk (caller reports it).
    TargetExists(String),
}

/// Apply a find/replace to a filename.
pub fn rename_replace(original: &str, pattern: &str, replacement: &str) -> String {
    original.replace(pattern, replacement)
}

pub fn add_prefix_suffix(original: &str, prefix: &str, suffix: &str) -> String {
    format!("{prefix}{original}{suffix}")
}

/// Sequential numbering like bulky's counter: `name_001.ext`.
/// `start` is 1-based, `pad` zero-pads the counter.
pub fn numbered(name: &str, index: usize, start: usize, pad: usize) -> String {
    let n = start + index;
    // split extension
    let (stem, ext) = match name.rfind('.') {
        Some(i) if i > 0 => (&name[..i], &name[i..]),
        _ => (name, ""),
    };
    format!("{stem}_{n:0width$}{ext}", width = pad)
}

/// Case transforms.
pub fn to_lower(name: &str) -> String {
    name.to_lowercase()
}
pub fn to_upper(name: &str) -> String {
    name.to_uppercase()
}
pub fn to_title(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut cap = true;
    for c in name.chars() {
        if c == '_' || c == '-' || c == ' ' {
            out.push(' ');
            cap = true;
        } else if cap {
            out.extend(c.to_uppercase());
            cap = false;
        } else {
            out.extend(c.to_lowercase());
        }
    }
    out
}

/// Build a dry-run plan for a list of filenames.
/// `transform` maps each name to its candidate new name.
/// Returns Err on collisions (duplicate targets) — fail-fast.
pub fn plan<F: Fn(&str) -> String>(files: &[String], transform: F) -> Result<Vec<RenameOp>, RenameError> {
    let mut ops = Vec::with_capacity(files.len());
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    for f in files {
        let to = transform(f);
        if !seen.insert(to.clone()) {
            return Err(RenameError::Collision(to));
        }
        ops.push(RenameOp { from: f.clone(), to });
    }
    Ok(ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bulk_renamer_aaa() {
        let filename = "IMG_2026_photo.jpg";
        let r1 = BulkRenamer::rename_replace(filename, "IMG_2026", "SigmaOS");
        let r2 = BulkRenamer::add_prefix_suffix("document", "draft_", ".pdf");
        assert_eq!(r1, "SigmaOS_photo.jpg");
        assert_eq!(r2, "draft_document.pdf");
    }

    #[test]
    fn numbering_and_case() {
        assert_eq!(numbered("photo.jpg", 0, 1, 3), "photo_001.jpg");
        assert_eq!(numbered("photo.jpg", 9, 1, 3), "photo_010.jpg");
        assert_eq!(numbered("archive.tar.gz", 0, 1, 2), "archive.tar_01.gz");
        assert_eq!(to_title("my_photo-file"), "My Photo File");
        assert_eq!(to_upper("ab.jpg"), "AB.JPG");
    }

    #[test]
    fn plan_detects_collisions() {
        let files = vec!["a.txt".to_string(), "b.txt".to_string()];
        // both map to same target -> collision
        let err = plan(&files, |_| "same.txt".to_string()).unwrap_err();
        assert_eq!(err, RenameError::Collision("same.txt".to_string()));
        // distinct targets -> ok
        let ops = plan(&files, |f| f.to_uppercase()).unwrap();
        assert_eq!(ops.len(), 2);
        assert_eq!(ops[0].to, "A.TXT");
    }
}

// Keep struct for API compatibility
pub struct BulkRenamer;
impl BulkRenamer {
    pub fn rename_replace(original: &str, pattern: &str, replacement: &str) -> String {
        rename_replace(original, pattern, replacement)
    }
    pub fn add_prefix_suffix(original: &str, prefix: &str, suffix: &str) -> String {
        add_prefix_suffix(original, prefix, suffix)
    }
}
