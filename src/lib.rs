pub struct BulkRenamer;

impl BulkRenamer {
    pub fn rename_replace(original: &str, pattern: &str, replacement: &str) -> String {
        original.replace(pattern, replacement)
    }
    pub fn add_prefix_suffix(original: &str, prefix: &str, suffix: &str) -> String {
        format!("{}{}{}", prefix, original, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bulk_renamer_aaa() {
        // Arrange
        let filename = "IMG_2026_photo.jpg";
        // Act
        let r1 = BulkRenamer::rename_replace(filename, "IMG_2026", "SigmaOS");
        let r2 = BulkRenamer::add_prefix_suffix("document", "draft_", ".pdf");
        // Assert
        assert_eq!(r1, "SigmaOS_photo.jpg");
        assert_eq!(r2, "draft_document.pdf");
    }
}
