// This file is part of css-autoprefix. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-autoprefix. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT.


/// Autoprefix a stylesheet
#[inline(always)]
pub fn autoprefix_stylesheet(stylesheet: &mut Stylesheet, can_i_use: &CanIUse, agents: &AgentNameAndVersionSet)
{
	CompositeCssRulesAutoprefixer::new(can_i_use, agents).autoprefix_stylesheet(stylesheet)
}
