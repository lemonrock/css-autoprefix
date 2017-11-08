// This file is part of css-autoprefix. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-autoprefix. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT.


#![allow(non_snake_case)]

//! # css-autoprefix
//!
//! A Rust library crate which autoprefixes CSS dependending on the agent versions, ages, regional usage patterns, etc selected.
//!
//! Currently needs nightly rust as a dependency (phf) needs nightly.
//!
//! ## Getting Going
//!
//!
//! ### To just get autoprefixed, sensible CSS
//!
//! ```
//! extern crate css_autoprefix;
//! use ::css_autoprefix::*;
//! use ::css_autoprefix::css::*;
//!
//! let some_css_utf_8_encoded = "margin-top: 10px;"
//! let stylesheet = Stylesheet::parse(some_css_utf_8_encoded).expect("valid CSS");
//!
//! let (can_i_use, agents) = sensible_rules_to_prefixes_default();
//! autoprefix_stylesheet(&mut stylesheet, &can_i_use, &agents);
//!
//! let mut destination = String::new();
//! // Don't write source-map and source-url comments if any are present in the stylesheet
//! let include_source_urls = false;
//! stylesheet.to_css(&mut destination, include_source_urls).expect("Failed to write to destination");
//! ```


pub extern crate caniuse_serde;
pub extern crate chrono;
pub extern crate css;
#[macro_use] extern crate maplit;


use self::cssRulesAutoprefixers::CompositeCssRulesAutoprefixer;
use ::caniuse_serde::*;
use ::caniuse_serde::regional_usage::*;
use ::css::Stylesheet;
use ::css::domain::*;
use ::css::domain::properties::*;
use ::css::domain::selectors::*;
use ::css::domain::VendorPrefix::*;
use ::std::collections::HashMap;
use ::std::ops::Deref;


pub(crate) mod cssRulesAutoprefixers;
pub(crate) mod propertyDeclarationAutoprefixers;
pub(crate) mod selectorAutoprefixers;


include!("autoprefix_stylesheet.rs");
include!("toFeatureName.rs");
include!("mapPrefixToVendorPrefix.rs");
include!("sensible_rules_to_prefixes.rs");
include!("sensible_rules_to_prefixes_default.rs");
