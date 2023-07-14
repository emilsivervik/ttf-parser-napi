use napi::bindgen_prelude::*;
use napi_derive::napi;

use tables::os2::{Permissions, ScriptMetrics};
use ttf_parser::Face;

mod tables;

pub use tables::{Tables, TablesEnum};

// #[napi]
// pub struct TTFParser {
//   // pub test: Option<u32>,
//   pub tables: Tables,
// }

pub enum CustomError {
  NapiError(Error),
  Panic,
}

impl AsRef<str> for CustomError {
  fn as_ref(&self) -> &str {
    match self {
      CustomError::Panic => "Panic",
      CustomError::NapiError(e) => e.status.as_ref(),
    }
  }
}

// #[napi]
// impl TTFParser {
//   #[napi(constructor)]
//   pub fn new(buffer: Buffer) -> TTFParser {
//     // let buf = &mut ctx.get::<JsBuffer>(0)?.into_value()?; // &mut [u8]
//     // let bla = Vec::<u8>::from(buffer);

//     // let bla = Vec::<u8>::from(buffer);
//     let face = Face::parse(buffer.as_ref(), 0).unwrap();

//     let tables = Tables::new(&face);

//     TTFParser { tables }
//     // TTFParser { test: None }
//   }
// }

#[napi(js_name = "TTFParser")]
#[derive(Clone)]
pub struct TTFParser {
  tables: Tables,

  /// Checks that face is marked as *Bold*.
  ///
  /// Returns `false` when OS/2 table is not present.
  #[napi(writable = false)]
  pub is_bold: bool,

  /// Checks that face is marked as *Italic*.
  ///
  /// Returns `false` when OS/2 table is not present.
  pub is_italic: bool,

  /// Checks that face is marked as *Monospaced*.
  ///
  /// Returns `false` when `post` table is not present.
  pub is_monospaced: bool,

  /// Checks that face is marked as *Oblique*.
  ///
  /// Returns `false` when OS/2 table is not present or when its version is < 4.
  pub is_oblique: bool,

  /// Checks that face is marked as *Regular*.
  ///
  /// Returns `false` when OS/2 table is not present.
  pub is_regular: bool,

  /// Checks that face is variable.
  ///
  /// Simply checks the presence of a `fvar` table.
  pub is_variable: bool,

  // Read https://github.com/freetype/freetype/blob/49270c17011491227ec7bd3fb73ede4f674aa065/src/sfnt/sfobjs.c#L1279
  // to learn more about the logic behind the following property.
  /// Returns a horizontal face ascender.
  ///
  /// This property is affected by variation axes.
  pub ascender: i16,

  /// Returns a horizontal face descender.
  ///
  /// This property is affected by variation axes.
  pub descender: i16,

  /// Returns face's height.
  ///
  /// This property is affected by variation axes.
  pub height: i16,

  /// Returns a horizontal face line gap.
  ///
  /// This property is affected by variation axes.
  pub line_gap: i16,

  /// Returns a horizontal typographic face ascender.
  ///
  /// Prefer `ascender` unless you explicitly want this. This is a more
  /// low-level alternative.
  ///
  /// This property is affected by variation axes.
  ///
  /// Returns `None` when OS/2 table is not present.
  pub typographic_ascender: Option<i16>,

  /// Returns a horizontal typographic face ascender.
  ///
  /// Prefer `ascender` unless you explicitly want this. This is a more
  /// low-level alternative.
  ///
  /// This property is affected by variation axes.
  ///
  /// Returns `None` when OS/2 table is not present.
  pub typographic_descender: Option<i16>,

  /// Returns a horizontal typographic face line gap.
  ///
  /// Prefer `line_gap` unless you explicitly want this. This is a more
  /// low-level alternative.
  ///
  /// This property is affected by variation axes.
  ///
  /// Returns `None` when OS/2 table is not present.
  pub typographic_line_gap: Option<i16>,

  /// Returns a vertical face ascender.
  ///
  /// This property is affected by variation axes.
  pub vertical_ascender: Option<i16>,

  /// Returns a vertical face descender.
  ///
  /// This property is affected by variation axes.
  pub vertical_descender: Option<i16>,

  /// Returns a vertical face height.
  ///
  /// This method is affected by variation axes.
  pub vertical_height: Option<i16>,

  /// Returns a vertical face line gap.
  ///
  /// This property is affected by variation axes.
  pub vertical_line_gap: Option<i16>,

  /// Returns face's units per EM.
  ///
  /// Guarantee to be in a 16..=16384 range.
  pub units_per_em: u16,

  /// Returns face's x height.
  ///
  /// This property is affected by variation axes.
  ///
  /// Returns `undefined` when OS/2 table is not present or when its version is < 2.
  pub x_height: Option<i16>,

  /// Returns face's capital height.
  ///
  /// This property is affected by variation axes.
  ///
  /// Returns `undefined` when OS/2 table is not present or when its version is < 2.
  pub capital_height: Option<i16>,

  /// Returns face's italic angle.
  ///
  /// Returns `undefined` when `post` table is not present.
  // pub italic_angle: Option<f32>,

  /// Returns face permissions.
  pub permissions: Option<Permissions>,

  /// Checks if the face subsetting is allowed.
  pub is_subsetting_allowed: bool,

  /// Checks if the face bitmaps embedding is allowed.
  pub is_bitmap_embedding_allowed: bool,

  /// Returns a total number of glyphs in the face.
  ///
  /// Never zero.
  pub number_of_glyphs: u16,

  /// Returns face's superscript metrics.
  ///
  /// This property is affected by variation axes.
  ///
  /// Returns `undefined` when OS/2 table is not present.
  pub superscript_metrics: Option<ScriptMetrics>,
}

#[napi]
impl TTFParser {
  /// Creates a new `TTFParser` from a raw data.
  ///
  /// `index` indicates the specific font face in a font collection.
  /// Use [`fonts_in_collection`] to get the total number of font faces.
  /// Defaults to 0 if not set.
  ///
  /// Required tables: `head`, `hhea` and `maxp`.
  ///
  /// If an optional table has invalid data it will be skipped.
  #[napi(constructor)]
  pub fn new(env: Env, buffer: Buffer, index: Option<i16>) -> napi::Result<TTFParser> {
    let face = match Face::parse(buffer.as_ref(), index.unwrap_or_default() as u32) {
      Ok(font) => font,
      Err(err) => {
        return Err(Error::new(Status::GenericFailure, "asd"));
        // return Err(Status::from_reason(env, "Division by zero is not allowed"))
      }
    };

    let tables = Tables::new(&face);
    // log(format!("{:?}", data).as_str());

    let permissions = tables.os2.as_ref().map(|v| v.permissions.to_owned());

    let parser = Self {
      tables,

      is_bold: face.is_bold(),
      is_italic: face.is_italic(),
      is_monospaced: face.is_monospaced(),
      is_oblique: face.is_oblique(),
      is_regular: face.is_regular(),
      is_subsetting_allowed: face.is_subsetting_allowed(),
      is_variable: face.is_variable(),
      // has_non_default_variation_coordinates: face.has_non_default_variation_coordinates(),
      units_per_em: face.units_per_em(),
      // italic_angle: face.italic_angle(),
      ascender: face.ascender(),
      descender: face.descender(),
      height: face.height(),
      line_gap: face.line_gap(),
      capital_height: face.capital_height(),
      typographic_ascender: face.typographic_ascender(),
      typographic_descender: face.typographic_descender(),
      typographic_line_gap: face.typographic_line_gap(),
      vertical_ascender: face.vertical_ascender(),
      vertical_descender: face.vertical_descender(),
      vertical_height: face.vertical_height(),
      vertical_line_gap: face.vertical_line_gap(),
      x_height: face.x_height(),
      permissions,
      is_bitmap_embedding_allowed: face.is_bitmap_embedding_allowed(),
      number_of_glyphs: face.number_of_glyphs(),
      superscript_metrics: face.superscript_metrics().map(ScriptMetrics::from),
      // glyph_ver_advance: face.glyph_ver_advance(),
      // glyph_hor_advance: face.glyph_hor_advance(),
    };

    Ok(parser)
  }

  // #[wasm_bindgen(js_name = style)]
  // pub fn style(&self) -> tables::os2::Style {
  //     if let Some(os2_value) = self.tables.os2 {
  //         return os2_value.style;
  //     }
  //     tables::os2::Style::Normal
  // }

  /// Returns a bounding box that large enough to enclose any glyph from the face.
  // #[wasm_bindgen(js_name = globalBoundingBox, method)]
  #[napi(getter)]
  pub fn global_bounding_box(&self) -> tables::Rect {
    self.tables.head.global_bbox
  }

  // #[wasm_bindgen(getter)]
  #[napi(getter)]
  pub fn tables(&mut self) -> Tables {
    self.tables.to_owned()
  }

  // #[wasm_bindgen(getter)]
  #[napi(getter, js_name = "toJSON")]
  pub fn to_json(&mut self) -> &str {
    "{\"name\":\"Macbook\",\"cpus\":1,\"hasCpus\":true,\"cores\":4,\"hasCores\":true}"
  }
}
