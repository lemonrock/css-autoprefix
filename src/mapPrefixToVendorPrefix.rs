// This file is part of css-autoprefix. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-autoprefix. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[inline(always)]
pub(crate) fn mapPrefixToVendorPrefix(prefix: &Prefix) -> VendorPrefix
{
	match *prefix
	{
		Prefix::o => o,
		
		Prefix::moz => moz,
		
		Prefix::webkit => webkit,
		
		Prefix::ms => ms,
		
		Prefix::Unknown(ref value) => match value.as_str()
		{
			"epub" => epub,
			"servo" => servo,
			_ => Unrecognised(value.to_owned()),
		},
	}
}
