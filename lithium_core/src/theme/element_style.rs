use std::sync::Arc;
use {Vec2, Color, Font};
use font::ErrorFont;

pub mod border {
    pub const TOP: usize = 0;
    pub const RIGHT: usize = 1;
    pub const BOTTOM: usize = 2;
    pub const LEFT: usize = 3;
}

pub mod corner {
    pub const TOP_LEFT: usize = 0;
    pub const TOP_RIGHT: usize = 1;
    pub const BOTTOM_RIGHT: usize = 2;
    pub const BOTTOM_LEFT: usize = 3;
}

#[derive(Clone, Debug)]
pub struct ElementStyle {
    pub background_layers: Vec<BackgroundLayer>,
    pub background_color: Color,
    pub font_color: Color,
    pub font: Arc<Font>,
    pub box_shadows: Vec<Shadow>,
    pub filters: Vec<Filter>,
    pub border: [Border; 4],
    pub border_radius: [Vec2<LengthOrPercentage>; 4],
    pub visible: bool,
    pub outline: Outline,
    pub overflow: Overflow,
    pub opacity: f32,
    pub isolate: bool,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PositionCoordinate {
    Length(f32),
    LengthOpposite(f32),
    Percentage(f32),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Image {
}

#[derive(Clone, PartialEq, Debug)]
pub struct BackgroundLayer {
    pub image: BackgroundImage,
    pub attachment: BackgroundAttachment,
    pub blend_mode: BackgroundBlendMode,
    pub clip: BackgroundBox,
    pub repeat_x: BackgroundRepeat,
    pub repeat_y: BackgroundRepeat,
    pub position: Vec2<PositionCoordinate>,
    pub size: BackgroundSize,
    pub origin: BackgroundBox,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BackgroundImage {
    None,
    Image(Image),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BackgroundAttachment {
    /// The background is fixed with regard to the element itself and does not scroll with
    /// its contents. (It is effectively attached to the element's border.) 
    Scroll,

    /// The background is fixed with regard to the viewport. Even if an element has a
    /// scrolling mechanism (see the ‘overflow’ property), a ‘fixed’ background doesn't move
    /// with the element.
    Fixed,

    /// The background is fixed with regard to the element's contents: if the element has a
    /// scrolling mechanism, the background scrolls with the element's contents, and the
    /// background painting area and background positioning area are relative to the
    /// scrollable area of the element rather than to the border framing them.
    Local,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum BackgroundBlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BackgroundRepeat {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BackgroundBox {
    BorderBox,
    PaddingBox,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BackgroundSize {
    Cover,
    Contain,
    Size(Vec2<Option<LengthOrPercentage>>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct LinearGradient {
    pub direction: AngleOrCorner,
    pub stops: Vec<ColorStop>,
    pub repeating: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RadialGradient {
    pub shape: RadialGradientShape,
    pub position: Vec2<PositionCoordinate>,
    pub stops: Vec<ColorStop>,
    pub repeating: bool,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RadialGradientShape {
    Circle(RadialGradientExtent),
    Ellipse(RadialGradientExtent),
    Ellipse2(Vec2<LengthOrPercentage>),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RadialGradientExtent {
    ClosestSide,
    FarthestSide,
    ClosestCorner,
    FarthestCorner,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AngleOrCorner {
    Angle(f32),
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorStop {
    pub color: Color,
    pub position: Option<LengthOrPercentage>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LengthOrPercentage {
    Length(f32),
    Percentage(f32),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LengthOrPercentageOrAuto {
    Auto,
    Length(f32),
    Percentage(f32),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Border {
    pub width: f32,
    pub style: BorderStyle,
    pub color: Color,
    pub image_outset: LengthOrPercentage,
    pub image_width: LengthOrPercentage,
    pub image_slice: LengthOrPercentage,
}

#[derive(Clone, Debug)]
pub struct BorderImage {
    pub source: Image,
    pub repeat: BorderImageRepeat,
    pub fill: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BorderImageRepeat {
    Stretch,
    Repeat,
    Round,
    Space,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BorderStyle {
    None,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Outline {
    pub color: Color,
    pub style: BorderStyle,
    pub width: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Shadow {
    pub position: Vec2<f32>, // h-shadow, v-shadow
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
    pub inset: bool,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Filter {
    Blur(f32),
    Brightness(f32),
    Contrast(f32),
    DropShadow(Shadow),
    Grayscale(f32),
    HueRotate(f32),
    Invert(f32),
    Opacity(f32),
    Saturate(f32),
    Sepia(f32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    // Scroll,
}

impl Default for ElementStyle {
    fn default() -> Self {
        ElementStyle {
            background_layers: Vec::new(),
            background_color: Color::transparent(),
            font_color: Color::transparent(),
            font: Arc::new(ErrorFont),
            box_shadows: Vec::new(),
            filters: Vec::new(),
            border: [Border::default(); 4],
            border_radius: [Vec2::new(LengthOrPercentage::Length(0.0), LengthOrPercentage::Length(0.0)); 4],
            visible: true,
            outline: Outline::default(),
            overflow: Overflow::default(),
            opacity: 1.0,
            isolate: false,
        }
    }
}

impl Default for Border {
    fn default() -> Self {
        Border {
            width: 0.0,
            style: BorderStyle::None,
            color: Color::transparent(),
            image_outset: LengthOrPercentage::Length(0.0),
            image_width: LengthOrPercentage::Length(1.0),
            image_slice: LengthOrPercentage::Percentage(1.0/3.0),
        }
    }
}

impl Default for Outline {
    fn default() -> Self {
        Outline {
            color: Color::transparent(),
            style: BorderStyle::None,
            width: 0.0,
        }
    }
}

impl Default for BackgroundLayer {
    fn default() -> Self {
        BackgroundLayer {
            image: BackgroundImage::None,
            attachment: BackgroundAttachment::default(),
            blend_mode: BackgroundBlendMode::default(),
            clip: BackgroundBox::default(),
            repeat_x: BackgroundRepeat::default(),
            repeat_y: BackgroundRepeat::default(),
            position: Vec2::new(PositionCoordinate::Percentage(0.0), PositionCoordinate::Percentage(0.0)),
            size: BackgroundSize::default(),
            origin: BackgroundBox::default(),
        }
    }
}

impl PartialEq<ElementStyle> for ElementStyle {
    fn eq(&self, other: &ElementStyle) -> bool {
        self.background_layers == other.background_layers &&
        self.font_color == other.font_color &&
        self.box_shadows == other.box_shadows &&
        self.filters == other.filters &&
        self.border == other.border &&
        self.visible == other.visible &&
        self.outline == other.outline &&
        self.overflow == other.overflow &&
        Arc::ptr_eq(&self.font, &other.font)
    }
}

impl Default for BackgroundSize {
    fn default() -> Self {
        BackgroundSize::Size(Vec2::new(None, None))
    }
}

macro_rules! derive_default {
    ($t:ident :: $i:ident) => {
        impl Default for $t {
            fn default() -> Self {
                $t::$i
            }
        }
    }
}

derive_default!(Overflow::Visible);
derive_default!(BackgroundAttachment::Scroll);
derive_default!(BackgroundBlendMode::Normal);
derive_default!(BackgroundBox::PaddingBox);
derive_default!(BorderImageRepeat::Stretch);
derive_default!(BackgroundRepeat::Repeat);
