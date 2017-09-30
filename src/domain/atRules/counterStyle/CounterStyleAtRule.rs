// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// An @counter-style rule
#[derive(Clone, Debug)]
pub struct CounterStyleAtRule
{
	pub name: CounterStyleIdent,

	/// https://drafts.csswg.org/css-counter-styles/#counter-style-system
	pub system: Option<System>,
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-negative
	pub negative: Option<Negative>,
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-prefix
	pub prefix: Option<Symbol>,
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-suffix
	pub suffix: Option<Symbol>,
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-range
	pub range: Option<Ranges>,
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-pad
	pub pad: Option<Pad>,
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-fallback
	pub fallback: Option<Fallback>,
	
	/// https://drafts.csswg.org/css-counter-styles/#descdef-counter-style-symbols
	pub symbols: Option<Symbols>,
	
	/// https://drafts.csswg.org/css-counter-styles/#descdef-counter-style-additive-symbols
	pub additive_symbols: Option<AdditiveSymbols>,
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-speak-as
	pub speak_as: Option<SpeakAs>,
}

impl ToCss for CounterStyleAtRule
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		#[inline(always)]
		fn write<W: fmt::Write, T: ToCss>(dest: &mut W, name: &str, value: &Option<T>) -> fmt::Result
		{
			if let Some(ref value) = value
			{
				dest.write_str(name)?;
				dest.write_char(':')?;
				value.to_css(dest)?;
				dest.write_char(';')?;
			}
		}
		
		dest.write_str("@counter-style ")?;
		self.name.to_css(dest)?;
		dest.write_char('{')?;
		write(dest, "system", &self.system)?;
		write(dest, "negative", &self.negative)?;
		write(dest, "prefix", &self.prefix)?;
		write(dest, "suffix", &self.suffix)?;
		write(dest, "range", &self.range)?;
		write(dest, "pad", &self.pad)?;
		write(dest, "fallback", &self.fallback)?;
		write(dest, "symbols", &self.symbols)?;
		write(dest, "additive-symbols", &self.additive_symbols)?;
		write(dest, "speak-as", &self.speak_as)?;
		dest.write_char('}')
	}
}

impl CounterStyleAtRule
{
	#[inline(always)]
	fn empty(name: CounterStyleIdent) -> Self
	{
		Self
		{
			name,
			system: None,
			negative: None,
			prefix: None,
			suffix: None,
			range: None,
			pad: None,
			fallback: None,
			synbols: None,
			additive_symbols: None,
			speak_as: None,
		}
	}
	
	/// Get the name of the counter style rule.
	pub fn name(&self) -> &CounterStyleIdent
	{
		&self.name
	}
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-system
	pub fn system(&self) -> Cow<System>
	{
		if let Some(ref value) = self.system
		{
			Cow::Borrowed(value)
		}
		else
		{
			Cow::Owned(System::Symbolic)
		}
	}
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-negative
	pub fn negative(&self) -> Cow<Negative>
	{
		if let Some(ref value) = self.negative
		{
			Cow::Borrowed(value)
		}
		else
		{
			Cow::Owned(Negative(Symbol::String("-".to_owned()), None))
		}
	}
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-prefix
	pub fn prefix(&self) -> Cow<Symbol>
	{
		if let Some(ref value) = self.prefix
		{
			Cow::Borrowed(value)
		}
		else
		{
			Cow::Owned(Symbol::String("".to_owned()))
		}
	}
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-suffix
	pub fn suffix(&self) -> Cow<Symbol>
	{
		if let Some(ref value) = self.suffix
		{
			Cow::Borrowed(value)
		}
		else
		{
			Cow::Owned(Symbol::String(". ".to_owned()))
		}
	}
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-range
	pub fn range(&self) -> Cow<Ranges>
	{
		if let Some(ref value) = self.range
		{
			Cow::Borrowed(value)
		}
		else
		{
			Cow::Owned(Ranges::empty())
		}
	}
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-pad
	pub fn pad(&self) -> Cow<Pad>
	{
		if let Some(ref value) = self.pad
		{
			Cow::Borrowed(value)
		}
		else
		{
			Cow::Owned(Pad(0, Symbol::String("".to_owned())))
		}
	}
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-fallback
	pub fn fallback(&self) -> Cow<Fallback>
	{
		if let Some(ref value) = self.fallback
		{
			Cow::Borrowed(value)
		}
		else
		{
			Cow::Owned(Fallback(CounterStyleIdent::decimal))
		}
	}
	
	/// https://drafts.csswg.org/css-counter-styles/#descdef-counter-style-symbols
	pub fn symbols(&self) -> Option<&Symbols>
	{
		self.symbols.as_ref()
	}
	
	/// https://drafts.csswg.org/css-counter-styles/#descdef-counter-style-additive-symbols
	pub fn additive_symbols(&self) -> Option<&AdditiveSymbols>
	{
		self.symbols.as_ref()
	}
	
	/// https://drafts.csswg.org/css-counter-styles/#counter-style-speak-as
	pub fn speak_as(&self) -> Cow<SpeakAs>
	{
		if let Some(ref value) = self.speak_as
		{
			Cow::Borrowed(value)
		}
		else
		{
			Cow::Owned(SpeakAs::Auto)
		}
	}
	
	/// Parse the body (inside `{}`) of an @counter-style rule
	pub(crate) fn parse_body<'i, 't>(name: CustomIdent, context: &ParserContext,input: &mut Parser<'i, 't>) -> Result<CounterStyleAtRule, ParseError<'i, CustomParseError<'i>>>
	{
		let mut rule = CounterStyleAtRule::empty(name);
		
		{
			let parser = CounterStyleAtRuleParser
			{
				context,
				rule: &mut rule,
			};
			while let Some(declaration) = DeclarationListParser::new(input, parser).next()
			{
				if declaration.is_err()
				{
					return declaration;
				}
			}
		}
		
		use self::System::*;
		match *rule.system()
		{
			ref system @ Cyclic | ref system @ Fixed { .. } | ref system @ Symbolic | ref system @ Alphabetic | ref system @ Numeric if rule.symbols.is_none() =>
			{
				Err(ParseError::Custom(CustomParseError::InvalidCounterStyleWithoutSymbols(system)))
			}
			
			ref system @ Alphabetic | ref system @ Numeric if rule.symbols().unwrap().0.len() < 2 =>
			{
				Err(ParseError::Custom(CustomParseError::InvalidCounterStyleNotEnoughSymbols(system)))
			}
			
			Additive if rule.additive_symbols.is_none() =>
			{
				Err(ParseError::Custom(CustomParseError::InvalidCounterStyleWithoutAdditiveSymbols))
			}
			
			Extends(_) if rule.symbols.is_some() =>
			{
				Err(ParseError::Custom(CustomParseError::InvalidCounterStyleExtendsWithSymbols))
			}
			
			Extends(_) if rule.additive_symbols.is_some() =>
			{
				Err(ParseError::Custom(CustomParseError::InvalidCounterStyleExtendsWithAdditiveSymbols))
			}
			
			_ => Ok(rule)
		}
	}
}