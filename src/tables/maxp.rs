use napi_derive::napi;
use ttf_parser::maxp;

/// A [Maximum Profile Table](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp).
#[napi(js_name = "MAXPTable", object)]
#[derive(Copy, Clone)]
pub struct Table {
  /// The total number of glyphs in the face.
  pub number_of_glyphs: u16,
}

impl Table {
  pub fn new(table: maxp::Table) -> Self {
    Self {
      number_of_glyphs: table.number_of_glyphs.get(),
    }
  }
}
