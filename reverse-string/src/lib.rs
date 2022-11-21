#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    input.rsplit("").collect()
}

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}
