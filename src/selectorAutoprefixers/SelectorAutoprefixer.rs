// This file is part of css-autoprefix. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-autoprefix. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT.


/// Prefixes selectors, such as pseudo-elements and pseudo-classes
pub trait SelectorAutoprefixer
{
	/// Prefixes selectors, such as pseudo-elements and pseudo-classes
	fn autoprefix(&self, selectors: &mut DeduplicatedSelectors, parent_vendor_prefix: Option<&VendorPrefix>);
}
