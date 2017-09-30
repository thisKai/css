// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind
{
	/// https://www.w3.org/TR/mediaqueries-4/#width
	Width(Range<Length>),
	
	/// https://www.w3.org/TR/mediaqueries-4/#height
	Height(Range<Length>),
	
	/// https://www.w3.org/TR/mediaqueries-4/#aspect-ratio
	AspectRatio(Range<Ratio>),
	
	/// https://www.w3.org/TR/mediaqueries-4/#orientation
	Orientation(MediaOrientation),
	
	/// https://www.w3.org/TR/mediaqueries-4/#resolution
	Resolution(Range<MediaResolution>),
	
	/// https://www.w3.org/TR/mediaqueries-4/#scan
	Scan(MediaScan),
	
	/// https://www.w3.org/TR/mediaqueries-4/#grid
	Grid(MediaGrid),
	
	/// https://www.w3.org/TR/mediaqueries-4/#update
	Update(MediaUpdate),
	
	/// https://www.w3.org/TR/mediaqueries-4/#mf-overflow-block
	OverflowBlock(MediaOverflowBlock),
	
	/// https://www.w3.org/TR/mediaqueries-4/#mf-overflow-inline
	OverflowInline(MediaOverflowInline),
	
	/// https://www.w3.org/TR/mediaqueries-4/#color
	Color(Range<ColorBitDepth>),
	
	/// https://www.w3.org/TR/mediaqueries-4/#color-index
	ColorIndex(Range<MediaColorIndex>),
	
	/// https://www.w3.org/TR/mediaqueries-4/#monochrome
	Monochrome(Range<MonochromeBitDepth>),
	
	/// https://www.w3.org/TR/mediaqueries-4/#color-gamut
	ColorGamut(MediaColorGamut),
	
	/// https://www.w3.org/TR/mediaqueries-4/#pointer
	Pointer(MediaPointer),
	
	/// https://www.w3.org/TR/mediaqueries-4/#hover
	Hover(MediaHover),
	
	/// https://www.w3.org/TR/mediaqueries-4/#any-input
	AnyPointer(MediaPointer),
	
	/// https://www.w3.org/TR/mediaqueries-4/#any-input
	AnyHover(MediaHover),
	
	/// https://compat.spec.whatwg.org/#css-media-queries-webkit-transform-3d
	Transform3D(MediaTransform3D),
}