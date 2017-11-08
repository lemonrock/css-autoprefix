// This file is part of css-autoprefix. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-autoprefix. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-autoprefix/master/COPYRIGHT.



type VendorPrefixedVariants = HashMap<VendorPrefix, (HashMap<VendorPrefixablePseudoClassName, VendorPrefix>, HashMap<VendorPrefixablePseudoElementName, VendorPrefix>)>;

/// Combines selector autoprefixers
#[derive(Debug, Clone)]
pub struct CompositeSelectorAutoprefixer
{
	vendorPrefixedVariants: VendorPrefixedVariants,
}

impl SelectorAutoprefixer for CompositeSelectorAutoprefixer
{
	fn autoprefix(&self, selectors: &mut DeduplicatedSelectors, _parent_vendor_prefix: Option<&VendorPrefix>)
	{
		let selectors = &mut selectors.0;
		
		let mut index = 0;
		while index < selectors.len()
		{
			for &(ref applyVendorPrefixToPseudoClasses, ref applyVendorPrefixToPseudoElements) in self.vendorPrefixedVariants.values()
			{
				// Whilst the css-selectors crate allows a selector to be cloned it provides no way to mutate it.
				// Hence we actually prefix by 'reparsing' and serializing to CSS. Ghastly and inefficient.
				
				if let Some(withVendorPrefix) = OurSelectorImpl::reparse_with_vendor_prefix(unsafe { selectors.get_unchecked(index) }, applyVendorPrefixToPseudoClasses, applyVendorPrefixToPseudoElements)
				{
					// Add this to the selectors before this selector
					selectors.insert(index, withVendorPrefix);
					index += 2;
				}
				else
				{
					index += 1;
				}
			}
		}
	}
}

impl CompositeSelectorAutoprefixer
{
	#[inline(always)]
	fn newVendorPrefixedVariantsEntry<'a>(vendorPrefixedVariants: &'a mut VendorPrefixedVariants, vendorPrefix: &VendorPrefix) -> &'a mut (HashMap<VendorPrefixablePseudoClassName, VendorPrefix>, HashMap<VendorPrefixablePseudoElementName, VendorPrefix>)
	{
		vendorPrefixedVariants.entry(vendorPrefix.clone()).or_insert_with(|| (HashMap::new(), HashMap::new()))
	}
	
	#[inline(always)]
	fn addPseudoClasses(vendorPrefixedVariants: &mut VendorPrefixedVariants, can_i_use: &CanIUse, agents: &AgentNameAndVersionSet)
	{
		#[inline(always)]
		fn addPseudoClass(pseudoClassName: VendorPrefixablePseudoClassName, vendorPrefixedVariants: &mut VendorPrefixedVariants, vendorPrefix: &VendorPrefix)
		{
			let value = CompositeSelectorAutoprefixer::newVendorPrefixedVariantsEntry(vendorPrefixedVariants, vendorPrefix);
			value.0.insert(pseudoClassName, vendorPrefix.clone());
		}
		
		fn addPseudoClassForFeature(featureName: &str, pseudoClassName: VendorPrefixablePseudoClassName, vendorPrefixedVariants: &mut VendorPrefixedVariants, can_i_use: &CanIUse, agents: &AgentNameAndVersionSet)
		{
			agents.support_for_a_feature(can_i_use, &toFeatureName(featureName), |agent, version, support|
			{
				if support.requires_prefix()
				{
					let vendorPrefix = agent.prefix(version);
					addPseudoClass(pseudoClassName, vendorPrefixedVariants, &mapPrefixToVendorPrefix(vendorPrefix));
				}
			});
		}
		
		#[inline(always)]
		fn addPseudoClassAlways(vendorPrefixes: &[VendorPrefix], pseudoClassName: VendorPrefixablePseudoClassName, vendorPrefixedVariants: &mut VendorPrefixedVariants)
		{
			for vendorPrefix in vendorPrefixes.iter()
			{
				addPseudoClass(pseudoClassName, vendorPrefixedVariants, vendorPrefix);
			}
		}
		
		use self::VendorPrefixablePseudoClassName::*;
		
		addPseudoClassForFeature("fullscreen", fullscreen, vendorPrefixedVariants, can_i_use, agents);
		addPseudoClassForFeature("css-read-only-write", read_only, vendorPrefixedVariants, can_i_use, agents);
		addPseudoClassForFeature("css-read-only-write", read_write, vendorPrefixedVariants, can_i_use, agents);
		addPseudoClassForFeature("css-any-link", any_link, vendorPrefixedVariants, can_i_use, agents);
		addPseudoClassForFeature("css-dir-pseudo", dir, vendorPrefixedVariants, can_i_use, agents);
		addPseudoClassForFeature("css-placeholder-shown", placeholder_shown, vendorPrefixedVariants, can_i_use, agents);
		addPseudoClassAlways(&[moz, webkit], any, vendorPrefixedVariants);
	}
	
	#[inline(always)]
	fn addPseudoElements(vendorPrefixedVariants: &mut VendorPrefixedVariants, can_i_use: &CanIUse, agents: &AgentNameAndVersionSet)
	{
		#[inline(always)]
		fn addPseudoElement(pseudoElementName: VendorPrefixablePseudoElementName, vendorPrefixedVariants: &mut VendorPrefixedVariants, vendorPrefix: &VendorPrefix)
		{
			let value = CompositeSelectorAutoprefixer::newVendorPrefixedVariantsEntry(vendorPrefixedVariants, vendorPrefix);
			value.1.insert(pseudoElementName, vendorPrefix.clone());
		}
		
		fn addPseudoElementForFeature(featureName: &str, pseudoElementName: VendorPrefixablePseudoElementName, vendorPrefixedVariants: &mut VendorPrefixedVariants, can_i_use: &CanIUse, agents: &AgentNameAndVersionSet)
		{
			agents.support_for_a_feature(can_i_use, &toFeatureName(featureName), |agent, version, support|
			{
				if support.requires_prefix()
				{
					let vendorPrefix = agent.prefix(version);
					addPseudoElement(pseudoElementName, vendorPrefixedVariants, &mapPrefixToVendorPrefix(vendorPrefix));
				}
			});
		}
		
		#[inline(always)]
		fn addPseudoElementAlways(vendorPrefixes: &[VendorPrefix], pseudoElementName: VendorPrefixablePseudoElementName, vendorPrefixedVariants: &mut VendorPrefixedVariants)
		{
			for vendorPrefix in vendorPrefixes.iter()
			{
				addPseudoElement(pseudoElementName, vendorPrefixedVariants, vendorPrefix);
			}
		}
		
		use self::VendorPrefixablePseudoElementName::*;
		
		addPseudoElementForFeature("css-placeholder", placeholder, vendorPrefixedVariants, can_i_use, agents);
		addPseudoElementForFeature("css-selection", selection, vendorPrefixedVariants, can_i_use, agents);
		addPseudoElementForFeature("fullscreen", backdrop, vendorPrefixedVariants, can_i_use, agents);
		addPseudoElementAlways(&[moz, webkit, ms], progress_bar, vendorPrefixedVariants);
		addPseudoElementAlways(&[moz, ms], range_progress, vendorPrefixedVariants);
		addPseudoElementAlways(&[moz, webkit, ms], range_thumb, vendorPrefixedVariants);
		addPseudoElementAlways(&[moz, webkit, ms], range_track, vendorPrefixedVariants);
	}
	
	#[inline(always)]
	pub(crate) fn new(can_i_use: &CanIUse, agents: &AgentNameAndVersionSet) -> Self
	{
		let mut vendorPrefixedVariants = HashMap::new();
		Self::addPseudoClasses(&mut vendorPrefixedVariants, can_i_use, agents);
		Self::addPseudoElements(&mut vendorPrefixedVariants, can_i_use, agents);
		
		Self
		{
			vendorPrefixedVariants,
		}
	}
}
