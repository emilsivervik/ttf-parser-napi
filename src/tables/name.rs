use napi::{bindgen_prelude::*, sys::Status};
use napi_derive::napi;
use ttf_parser::{name, LazyArray16};

/// A [Name ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids).
#[napi]
pub enum NameId {
  CopyrightNotice,
  Family,
  Subfamily,
  UniqueId,
  FullName,
  Version,
  PostScriptName,
  Trademark,
  Manufacturer,
  Designer,
  Description,
  VendorUrl,
  DesignerUrl,
  License,
  LicenseUrl,
  // Reserved = "Reserved",
  TypographicFamily,
  TypographicSubFamily,
  CompatibleFull,
  SampleText,
  PostScriptCID,
  WWSFamily,
  WWSSubFamily,
  LightBackgroundPalette,
  DarkBackgroundPalette,
  VariationsPostScriptNamePrefix,
  Unknown,
}

// #[napi]
impl From<u16> for NameId {
  fn from(name_id: u16) -> NameId {
    match name_id {
      name::name_id::COPYRIGHT_NOTICE => NameId::CopyrightNotice,
      name::name_id::FAMILY => NameId::Family,
      name::name_id::SUBFAMILY => NameId::Subfamily,
      name::name_id::UNIQUE_ID => NameId::UniqueId,
      name::name_id::FULL_NAME => NameId::FullName,
      name::name_id::VERSION => NameId::Version,
      name::name_id::POST_SCRIPT_NAME => NameId::PostScriptName,
      name::name_id::TRADEMARK => NameId::Trademark,
      name::name_id::MANUFACTURER => NameId::Manufacturer,
      name::name_id::DESIGNER => NameId::Designer,
      name::name_id::DESCRIPTION => NameId::Description,
      name::name_id::VENDOR_URL => NameId::VendorUrl,
      name::name_id::DESIGNER_URL => NameId::DesignerUrl,
      name::name_id::LICENSE => NameId::License,
      name::name_id::LICENSE_URL => NameId::LicenseUrl,
      name::name_id::TYPOGRAPHIC_FAMILY => NameId::TypographicFamily,
      name::name_id::TYPOGRAPHIC_SUBFAMILY => NameId::TypographicSubFamily,
      name::name_id::COMPATIBLE_FULL => NameId::CompatibleFull,
      name::name_id::SAMPLE_TEXT => NameId::SampleText,
      name::name_id::POST_SCRIPT_CID => NameId::PostScriptCID,
      name::name_id::WWS_FAMILY => NameId::WWSFamily,
      name::name_id::WWS_SUBFAMILY => NameId::WWSSubFamily,
      name::name_id::LIGHT_BACKGROUND_PALETTE => NameId::LightBackgroundPalette,
      name::name_id::DARK_BACKGROUND_PALETTE => NameId::DarkBackgroundPalette,
      name::name_id::VARIATIONS_POST_SCRIPT_NAME_PREFIX => NameId::VariationsPostScriptNamePrefix,
      _ => NameId::Unknown,
    }
  }
}

/// A [platform ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#platform-ids).
#[napi]
// #[derive(Clone, Copy)]
pub enum PlatformId {
  Unicode,
  Macintosh,
  Iso,
  Windows,
  Custom,
}

impl From<name::PlatformId> for PlatformId {
  fn from(platform_id: name::PlatformId) -> PlatformId {
    match platform_id {
      name::PlatformId::Custom => PlatformId::Custom,
      name::PlatformId::Iso => PlatformId::Iso,
      name::PlatformId::Macintosh => PlatformId::Macintosh,
      name::PlatformId::Unicode => PlatformId::Unicode,
      name::PlatformId::Windows => PlatformId::Windows,
    }
  }
}

/// A [Name Record](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-records).
#[napi(object)]
#[derive(Clone)]
pub struct NameRecord {
  /// A platform ID.
  pub platform_id: PlatformId,

  /// A language ID.
  pub language_id: String,

  /// A [Name ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids).
  pub name_id: NameId,

  pub name: String,
}

/// A [Naming Table](https://docs.microsoft.com/en-us/typography/opentype/spec/name).
#[napi(js_name = "NAMETable", object)]
#[derive(Clone)]
pub struct Table {
  pub names: Vec<NameRecord>,
}

impl Table {
  pub fn new(table: Option<name::Table>) -> Option<Self> {
    let Some(table_names) = table else {
            return None;
        };
    let names = table_names
      .names
      .into_iter()
      .map(|v| {
        let name: String = {
          if v.is_unicode() {
            let mut name: Vec<u16> = Vec::new();
            for c in LazyArray16::<u16>::new(v.name) {
              name.push(c);
            }

            String::from_utf16(&name).unwrap()
          } else {
            std::str::from_utf8(v.name).unwrap_or_default().to_string()
          }
        };

        NameRecord {
          platform_id: v.platform_id.into(),
          language_id: v.language().to_string(),
          name_id: v.name_id.into(),
          name,
        }
      })
      .collect::<Vec<NameRecord>>();

    Some(Self { names })
  }
}
