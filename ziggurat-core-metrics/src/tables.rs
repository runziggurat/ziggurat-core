//! Tables to display metrics.
use std::time::Duration;

use tabled::{object::Segment, Alignment, Modify, Style, Table};

/// Formats `f64` with 2 decimal points.
pub fn table_float_display(x: &f64) -> String {
    format!("{0:.2}", x)
}

/// Returns the duration converted to milliseconds.
pub fn duration_as_ms(duration: Duration) -> f64 {
    duration.as_millis() as f64
}

/// Formats a table with our custom style.
///
/// Modifications:
///  - [pseudo style](https://docs.rs/tabled/0.2.1/tabled/style/struct.Style.html#method.pseudo)
///  - centered headers
///  - right aligned data
pub fn fmt_table(mut table: Table) -> String {
    // table with pseudo style, right aligned data and center aligned headers
    table
        .with(Style::modern())
        .with(Modify::new(Segment::all()).with(Alignment::right()))
        .to_string()
}
