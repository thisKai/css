// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Represents an entire CSS stylesheet.
/// The values of property declarations are currently stored as a string. Parsing property declarations is a monster job. If you feel like helping...
#[derive(Debug, Clone)]
pub struct Stylesheet
{
	/// The stylesheet's rules.
	pub rules: CssRules,
	
	/// An optional source map for this stylesheet.
	pub source_map_url: Option<String>,
	
	/// An optional source URL for this stylesheet.
	pub source_url: Option<String>,
}

impl HasCssRules for Stylesheet
{
	#[inline(always)]
	fn css_rules(&self) -> &CssRules
	{
		&self.rules
	}
	
	#[inline(always)]
	fn css_rules_mut(&mut self) -> &mut CssRules
	{
		&mut self.rules
	}
	
	#[inline(always)]
	fn css_rules_slice(&self) -> &[CssRule]
	{
		&self.rules.0[..]
	}
	
	#[inline(always)]
	fn css_rules_vec(&self) -> &Vec<CssRule>
	{
		&self.rules.0
	}
	
	#[inline(always)]
	fn css_rules_vec_mut(&mut self) -> &mut Vec<CssRule>
	{
		&mut self.rules.0
	}
}

impl Stylesheet
{
	// An solution is to use the HTTP header SourceMap: <url>
	pub fn to_css<W: fmt::Write>(&self, destination: &mut W, include_source_urls: bool) -> fmt::Result
	{
		if include_source_urls
		{
			// An older convention was to use '@' instead of '#'
			
			if let Some(ref source_map_url) = self.source_map_url
			{
				write!(destination, "//# sourceMappingURL=<{}>\n", source_map_url)?;
			}
			
			if let Some(ref source_url) = self.source_url
			{
				write!(destination, "//# sourceURL=<{}>\n", source_url)?;
			}
		}
		
		self.rules.to_css(destination)?;
		
		Ok(())
	}
	
	/// Parses a string of CSS to produce a stylesheet.
	/// Can be used with the contents of a CSS file.
	/// Assumes the string is UTF-8 encoded.
	/// Does not use a stream of bytes as parsing CSS involves going backwards and forwards a lot... CSS parsing is somewhat evil and is not particularly efficient.
	/// The parser does apply a few small modifications to the incoming CSS, normalizing some pseudo-class, psuedo-element and media query names.
	/// The parser does not parse properties as such, simply keeping them as a CSS string. Hopefully it will one day - there are only 200 odd specialist rules to implement.
	pub fn parse<'i>(css: &'i str) -> Result<Self, PreciseParseError<CustomParseError<'i>>>
	{
		const LineNumberingIsZeroBased: u32 = 0;
		
		let mut parserInput = ParserInput::new_with_line_number_offset(css, LineNumberingIsZeroBased);
		let mut input = Parser::new(&mut parserInput);
		
		let mut rules = Vec::new();
		
		let topLevelRuleParser = TopLevelRuleParser
		{
			context: ParserContext
			{
				rule_type: None,
				parsing_mode: ParsingMode::Default,
			},
			state: State::Start,
			namespaces: Namespaces::empty(),
		};
		
		{
			let mut iter = RuleListParser::new_for_stylesheet(&mut input, topLevelRuleParser);
			
			while let Some(result) = iter.next()
			{
				match result
				{
					Ok(rule) => rules.push(rule),
					Err(preciseParseError) => return Err(preciseParseError),
				}
			}
		}
		
		Ok
		(
			Self
			{
				rules: CssRules(rules),
				source_map_url: input.current_source_map_url().map(String::from),
				source_url: input.current_source_url().map(String::from),
			}
		)
	}
}
