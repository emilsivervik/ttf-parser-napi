use napi_derive::napi;
use ttf_parser::hhea;

/// A [Horizontal Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea).
#[napi(js_name = "HHEATable", object)]
#[derive(Clone, Copy)]
pub struct Table {
  /// Face ascender.
  pub ascender: i16,
  /// Face descender.
  pub descender: i16,
  /// Face line gap.
  pub line_gap: i16,
  /// Number of metrics in the `hmtx` table.
  pub number_of_metrics: u16,
}

impl Table {
  pub fn new(table: hhea::Table) -> Self {
    Self {
      ascender: table.ascender,
      descender: table.descender,
      line_gap: table.line_gap,
      number_of_metrics: table.number_of_metrics,
    }
  }
}
