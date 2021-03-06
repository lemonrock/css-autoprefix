// This file is part of css-autoprefix. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-autoprefix. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT.


use super::*;
use ::css::domain::HasPropertyDeclarations;
use ::css::domain::HasVendorPrefix;
use ::css::domain::VendorPrefix;
use ::std::collections::BTreeSet;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::rc::Rc;


include!("CompositePropertyDeclarationAutoprefixer.rs");
include!("PropertyNamePropertyDeclarationAutoprefixer.rs");
include!("PropertyNamePropertyDeclarationAutoprefixerCreator.rs");
include!("SimplePropertyValuePropertyDeclarationAutoprefixer.rs");
include!("SimplePropertyValuePropertyDeclarationAutoprefixerCreator.rs");
include!("PropertyDeclarationAutoprefixer.rs");
