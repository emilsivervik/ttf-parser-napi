pub mod head;
pub mod hhea;
pub mod maxp;
pub mod name;
pub mod os2;
use napi::bindgen_prelude::*;

use napi_derive::napi;
use ttf_parser::{Face, LineMetrics as PLineMetrics, Rect as PRect, Weight as PWeight};

#[napi(object)]
#[derive(Clone)]
pub struct Tables {
  /// A [Font Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/head).
  pub head: crate::tables::head::Table,
  // / A [Horizontal Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea).
  pub hhea: crate::tables::hhea::Table,

  // / A [Maximum Profile Table](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp).
  pub maxp: crate::tables::maxp::Table,
  // / A [OS/2 and Windows Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/os2).
  pub os2: Option<crate::tables::os2::Table>,

  // / https://docs.microsoft.com/en-us/typography/opentype/spec/name).
  pub name: Option<crate::tables::name::Table>,
}

impl Tables {
  pub fn new(face: &Face) -> Self {
    let face_tables = face.tables();

    let head = crate::tables::head::Table::new(face_tables.head);
    let hhea = crate::tables::hhea::Table::new(face_tables.hhea);
    let maxp = crate::tables::maxp::Table::new(face_tables.maxp);

    let os2 = crate::tables::os2::Table::new(face_tables.os2);
    let name = crate::tables::name::Table::new(face_tables.name);

    Self {
      head,
      hhea,
      maxp,
      os2,
      name,
    }
  }
}

pub enum TablesEnum {
  OS2,
}

#[napi(object)]
#[derive(Copy, Clone)]
pub struct LineMetrics {
  /// Line position.
  pub position: i16,

  /// Line thickness.
  pub thickness: i16,
}

impl From<PLineMetrics> for LineMetrics {
  fn from(strikeout_metrics: PLineMetrics) -> LineMetrics {
    LineMetrics {
      position: strikeout_metrics.position,
      thickness: strikeout_metrics.thickness,
    }
  }
}

/// A face [weight](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass).
#[napi]
pub enum Weight {
  Thin,
  ExtraLight,
  Light,
  Normal,
  Medium,
  SemiBold,
  Bold,
  ExtraBold,
  Black,
  Other, // TODO: Other(u16),
}

impl From<PWeight> for Weight {
  fn from(weight: PWeight) -> Weight {
    match weight {
      PWeight::Black => Weight::Black,
      PWeight::Bold => Weight::Bold,
      PWeight::ExtraBold => Weight::ExtraBold,
      PWeight::ExtraLight => Weight::ExtraLight,
      PWeight::Light => Weight::Light,
      PWeight::Medium => Weight::Medium,
      PWeight::Normal => Weight::Normal,
      PWeight::SemiBold => Weight::SemiBold,
      PWeight::Thin => Weight::Thin,
      PWeight::Other(_) => Weight::Other,
    }
  }
}

/// A rectangle.
///
/// Doesn't guarantee that `x_min` <= `x_max` and/or `y_min` <= `y_max`.
#[napi(object)]
#[derive(Copy, Clone)]
pub struct Rect {
  pub x_min: i16,
  pub y_min: i16,
  pub x_max: i16,
  pub y_max: i16,
}

#[napi]
impl From<PRect> for Rect {
  fn from(rect: PRect) -> Rect {
    Self {
      x_max: rect.x_max,
      x_min: rect.x_min,
      y_max: rect.y_max,
      y_min: rect.y_min,
    }
  }
}
