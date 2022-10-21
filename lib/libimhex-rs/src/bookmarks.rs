use crate::Color;
use std::ops::Range;
use std::usize;

use autocxx::*;

/// Add a bookmark to a region of the current imhex view with an optionally provided color
///
/// ```rust,ignore
/// use hex::bookmarks;
///
/// bookmarks::add(0..0x10, "header", "this is the header of the file", None);
///
/// let red = Color::new(0xFF, 0, 0, 0xFF);
/// bookmarks::add(0x10..0x30, "table", "this is the table of the file", red);
/// ```
pub fn add(region: Range<u64>, name: &str, comment: &str, color: impl Into<Option<Color>>) {
    cxx::let_cxx_string!(cpp_name = name);
    cxx::let_cxx_string!(cpp_comment = comment);

    crate::ffi::hex::ImHexApi::Bookmarks::add(
        c_ulonglong::from(region.start),
        region.end.saturating_sub(region.start) as usize,
        &cpp_name,
        &cpp_comment,
        color.into().unwrap_or(crate::Color::new(0, 0, 0, 0)).rgba().into(),
    );
}
