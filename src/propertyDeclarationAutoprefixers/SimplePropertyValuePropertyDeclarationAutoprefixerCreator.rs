// This file is part of css-autoprefix. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-autoprefix. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT.


enum SimplePropertyValuePropertyDeclarationAutoprefixerCreator
{
	Simple
	{
		featureName: &'static str,
		propertyName: &'static str,
		propertyValue: &'static str,
		removeUnprefixedProperty: bool,
	},
}

impl SimplePropertyValuePropertyDeclarationAutoprefixerCreator
{
	#[inline(always)]
	pub(crate) fn new(can_i_use: &CanIUse, agents: &AgentNameAndVersionSet) -> Vec<SimplePropertyValuePropertyDeclarationAutoprefixer>
	{
		let generators = vec!
		[
			Self::simple("css-sticky", "position", "sticky", false),
		];
		
		let mut result = Vec::new();
		
		for generator in generators.iter()
		{
			generator.generate(can_i_use, agents, &mut result);
		}
		
		result
	}
	
	#[inline(always)]
	fn simple(featureName: &'static str, propertyName: &'static str, propertyValue: &'static str, removeUnprefixedProperty: bool) -> Self
	{
		SimplePropertyValuePropertyDeclarationAutoprefixerCreator::Simple
		{
			featureName,
			propertyName,
			propertyValue,
			removeUnprefixedProperty,
		}
	}
	
	#[inline(always)]
	fn generate(&self, can_i_use: &CanIUse, agents: &AgentNameAndVersionSet, autoprefixers: &mut Vec<SimplePropertyValuePropertyDeclarationAutoprefixer>)
	{
		use self::SimplePropertyValuePropertyDeclarationAutoprefixerCreator::*;
		
		match *self
		{
			Simple { ref featureName, ref propertyName, ref propertyValue, removeUnprefixedProperty } =>
			{
				let featureName = toFeatureName(*featureName);
				
				let mut vendorPrefixes = BTreeSet::new();
				agents.support_for_a_feature(can_i_use, &featureName, |agent, version, support|
				{
					if support.requires_prefix()
					{
						let vendorPrefix = mapPrefixToVendorPrefix(agent.prefix(version));
						vendorPrefixes.insert(vendorPrefix);
					}
				});
				
				autoprefixers.push
				(
					SimplePropertyValuePropertyDeclarationAutoprefixer
					{
						propertyName,
						propertyValue,
						removeUnprefixedProperty,
						vendorPrefixes,
					}
				);
			},
		}
	}
}
