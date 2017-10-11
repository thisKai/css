// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PropertyDeclaration
{
	pub vendor_prefix: Option<VendorPrefix>,
	pub name: Atom,
	pub value: UnparsedPropertyValue,
	pub importance: Importance,
}

impl ToCss for PropertyDeclaration
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		if let Some(ref vendorPrefix) = self.vendor_prefix
		{
			vendorPrefix.to_css(dest)?;
		}
		self.name.to_css(dest)?;
		dest.write_char(':')?;
		self.value.to_css(dest)?;
		self.importance.to_css(dest)?;
		dest.write_char(';')
	}
}

impl PropertyDeclaration
{
	/// https://drafts.csswg.org/css-variables/#typedef-custom-property-name
	#[inline(always)]
	pub fn hasACustomPropertyName(&self) -> bool
	{
		self.name.starts_with("--")
	}
	
	#[inline(always)]
	pub fn hasAVendorPrefix(&self) -> bool
	{
		if self.hasACustomPropertyName()
		{
			false
		}
		else
		{
			self.name.starts_with("-")
		}
	}
	
	#[inline(always)]
	pub fn hasAsciiNameIgnoringCase(&self, name: &str) -> bool
	{
		self.name.eq_ignore_ascii_case(name)
	}
}
