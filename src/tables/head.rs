use super::Rect;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ttf_parser::head;

#[napi]
pub enum IndexToLocationFormat {
  Short,
  Long,
}

#[napi]
impl From<head::IndexToLocationFormat> for IndexToLocationFormat {
  fn from(index_to_location_format: head::IndexToLocationFormat) -> IndexToLocationFormat {
    match index_to_location_format {
      head::IndexToLocationFormat::Long => IndexToLocationFormat::Long,
      head::IndexToLocationFormat::Short => IndexToLocationFormat::Short,
    }
  }
}

/// A [Font Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/head).
#[napi(js_name = "HEADTable", object)]
#[derive(Clone)]
pub struct Table {
  /// Units per EM.
  pub units_per_em: u16,

  /// A bounding box that large enough to enclose any glyph from the face.
  pub global_bbox: Rect,

  /// An index format used by the [Index to Location Table](
  /// https://docs.microsoft.com/en-us/typography/opentype/spec/loca).
  pub index_to_location_format: IndexToLocationFormat,
}

impl Table {
  pub fn new(table: head::Table) -> Self {
    Self {
      index_to_location_format: table.index_to_location_format.into(),
      units_per_em: table.units_per_em,
      global_bbox: table.global_bbox.into(),
    }
  }
}
