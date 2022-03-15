// This file is part of senpy-ffi <https://github.com/senpy-club/senpy-ffi>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

//! FFI bindings for [`senpy-rs`](https://github.com/senpy-club/senpy-rs)

#![feature(trivial_bounds)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)]
#![recursion_limit = "128"]
#![doc(
  html_logo_url = "https://senpy.club/favicon.png",
  html_favicon_url = "https://senpy.club/favicon.png"
)]

use std::ffi::{CStr, CString};

use libc::c_char;

/// The base URL to The Senpy Club API
///
/// FFI binding to `senpy::SENPY_CLUB_API_BASE_URL`
#[no_mangle]
pub static SENPY_CLUB_API_BASE_URL: &str = senpy::SENPY_CLUB_API_BASE_URL;
/// The current API version of The Senpy Club API
///
/// FFI binding to `senpy::SENPY_CLUB_API_CURRENT_VERSION`
#[no_mangle]
pub static SENPY_CLUB_API_CURRENT_VERSION: u32 =
  senpy::SENPY_CLUB_API_CURRENT_VERSION;
/// The API URL to The Senpy Club API
///
/// FFI binding to `senpy::SENPY_CLUB_API_URL`
#[no_mangle]
pub static SENPY_CLUB_API_URL: &str = senpy::SENPY_CLUB_API_URL;

/// The response of the <https://api.senpy.club/v2/random> route
///
/// Part of the FFI binding to `senpy::random`
#[repr(C)]
#[derive(Default)]
pub struct Random {
  language: String,
  image:    String,
}
impl Random {
  /// Initializes a new `Random`
  ///
  /// Part of the FFI binding to `senpy::random`
  #[must_use]
  pub fn new() -> Self { Self::default() }

  /// Populates a `Random` from a `senpy::random` call
  ///
  /// Part of the FFI binding to `senpy::random`
  pub fn populate(&mut self) {
    if let Ok(image) = senpy::random() {
      self.language = image.language;
      self.image = image.image;
    } else {
      self.language = "".to_string();
      self.image = "".to_string();
    }
  }

  /// Frees a `Random`
  ///
  /// Part of the FFI binding to `senpy::random`
  #[must_use]
  pub fn get(&self, key: &str) -> String {
    match key {
      "language" => self.language.clone(),
      "image" => self.image.clone(),
      _ => "".to_string(),
    }
  }
}

/// Returns an array where the first element is the size of the array and the
/// remaining elements are the images.
///
/// If the first element (size) is `-1`; the
/// request failed for any reason.
///
/// # Safety
/// This is an *unsafe* FFI binding to `senpy::language`.
///
/// # Panics
/// if a `String` cannot be converted into a `CString`
#[no_mangle]
pub unsafe extern "C" fn language(language: *const c_char) -> *mut *mut c_char {
  match senpy::language(CStr::from_ptr(language).to_str().unwrap()) {
    Ok(images) => {
      let mut images_c =
        vec![CString::new(images.len().to_string()).unwrap().into_raw()];

      for image in images {
        images_c.push(CString::new(image).unwrap().into_raw());
      }

      images_c.as_mut_ptr()
    }
    Err(_) => vec![CString::new("-1").unwrap().into_raw()].as_mut_ptr(),
  }
}

/// Returns an array where the first element is the size of the array and the
/// remaining elements are the languages.
///
/// If the first element (size) is `-1`; the request failed for
/// any reason.
///
/// FFI binding to `senpy::languages`
///
/// # Panics
/// if a `String` cannot be converted into a `CString`
#[no_mangle]
pub extern "C" fn languages() -> *mut *mut c_char {
  match senpy::languages() {
    Ok(languages) => {
      let mut languages_c = vec![CString::new(languages.len().to_string())
        .unwrap()
        .into_raw()];

      for language in languages {
        languages_c.push(CString::new(language).unwrap().into_raw());
      }

      languages_c.as_mut_ptr()
    }
    Err(_) => vec![CString::new("-1").unwrap().into_raw()].as_mut_ptr(),
  }
}

/// Initializes a new `Random`
///
/// Part of the FFI binding to `senpy::random`
#[no_mangle]
pub extern "C" fn random_new() -> *mut Random {
  Box::into_raw(Box::new(Random::new()))
}

/// Populates a `Random` from a `senpy::random` call
///
/// # Safety
/// This is part of an *unsafe* FFI binding to `senpy::random`.
#[no_mangle]
pub unsafe extern "C" fn random_populate(random: *mut Random) {
  (&mut *random).populate();
}

/// Gets a member from a `Random`, valid members are `language` and `image`.
///
/// # Safety
/// This is part of an *unsafe* FFI binding to `senpy::random`.
///
/// # Panics
/// if the `key` cannot be wrapped as a safe `CStr`
#[no_mangle]
pub unsafe extern "C" fn random_get(
  random: *const Random,
  key: *const c_char,
) -> *mut c_char {
  CString::new((&*random).get(CStr::from_ptr(key).to_str().unwrap()))
    .unwrap()
    .into_raw()
}

/// Frees a `Random`
///
/// # Safety
/// This is part of an *unsafe* FFI binding to `senpy::random`.
#[no_mangle]
pub unsafe extern "C" fn random_free(random: *mut Random) {
  if random.is_null() {
    return;
  }

  Box::from_raw(random);
}

/// Returns `1` if up, returns `0` if down, and returns `-1` if the request
/// failed for any reason.
///
/// FFI binding to `senpy::status`
#[no_mangle]
pub extern "C" fn status() -> i32 {
  match senpy::status() {
    Ok(status) =>
      if status {
        1
      } else {
        0
      },
    Err(_) => -1,
  }
}
