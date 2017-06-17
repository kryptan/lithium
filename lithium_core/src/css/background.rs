use cssparser::{Parser, ParseError};
use theme::ElementStyle;
use theme::element_style::{BackgroundPicture, BackgroundImage, LinearGradient, RadialGradient};
use super::parse_color;

pub fn background<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    if let Ok(layers) = parser.parse_comma_separated(parse_background_layer) {
        element_style.background_images = layers;
    }

    Ok(())
}

pub fn parse_background_layer<'i, 'tt>(_parser: &mut Parser<'i, 'tt>) -> Result<BackgroundImage, ParseError<'i, ()>> {
    unimplemented!()
}

pub fn background_attachment<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn background_blend_mode<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn background_color<'i, 'tt>(parser: &mut Parser<'i, 'tt>, element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    let parsed_color = parse_color(parser)?;
    if let Some(image) = element_style.background_images.last_mut() {
        if let BackgroundPicture::Color(ref mut color) = image.image {
            *color = parsed_color;
            return Ok(());
        }
    }

    element_style.background_images.push(BackgroundImage {
        image: BackgroundPicture::Color(parsed_color),
        .. BackgroundImage::default()
    });

    Ok(())
}

fn parse_background_image<'i, 'tt>(parser: &mut Parser<'i, 'tt>) -> Result<BackgroundPicture, ParseError<'i, ()>> {
    let function = parser.expect_function()?;

    Ok(match_ignore_ascii_case! { function.as_ref(),
        "linear-gradient" => BackgroundPicture::LinearGradient(parser.parse_nested_block(|parser| parse_linear_gradient(parser, false))?),
        "radial-gradient" => BackgroundPicture::RadialGradient(parser.parse_nested_block(|parser| parse_radial_gradient(parser, false))?),
        "repeating-linear-gradient" => BackgroundPicture::LinearGradient(parser.parse_nested_block(|parser| parse_linear_gradient(parser, true))?),
        "repeating-radial-gradient" => BackgroundPicture::RadialGradient(parser.parse_nested_block(|parser| parse_radial_gradient(parser, true))?),
        _ => return Err(ParseError::Custom(())),
    })
}

fn parse_linear_gradient<'i, 'tt>(parser: &mut Parser<'i, 'tt>, repeating: bool) -> Result<LinearGradient, ParseError<'i, ()>> {
    unimplemented!()
}

fn parse_radial_gradient<'i, 'tt>(parser: &mut Parser<'i, 'tt>, repeating: bool) -> Result<RadialGradient, ParseError<'i, ()>> {
    unimplemented!()
}

pub fn background_image<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn background_position<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn background_repeat<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn background_clip<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn background_origin<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}

pub fn background_size<'i, 'tt>(_parser: &mut Parser<'i, 'tt>, _element_style: &mut ElementStyle) -> Result<(), ParseError<'i, ()>> {
    unimplemented!()
}
