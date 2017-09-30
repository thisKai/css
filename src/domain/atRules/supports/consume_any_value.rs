// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// https://drafts.csswg.org/css-syntax-3/#typedef-any-value
fn consume_any_value<'i, 't>(input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, CustomParseError<'i>>>
{
	input.expect_no_error_token().map_err(|err| err.into())
}
