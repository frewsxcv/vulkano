//! Stage when triangles are turned into pixels.
//! 
//! The rasterization is the stage when collections of triangles are turned into collections
//! of pixels or samples.
//!
use vk;

/// State of the rasterizer.
#[derive(Clone, Debug)]
pub struct Rasterization {
    /// If true, then the depth value of the vertices will be clamped to [0.0 ; 1.0]. If false,
    /// fragments whose depth is outside of this range will be discarded.
    pub depth_clamp: bool,

    /// If true, all the fragments will be discarded. This is usually used when your vertex shader
    /// has some side effects and you don't need to run the fragment shader.
    pub rasterizer_discard: bool,

    /// This setting can ask the rasterizer to downgrade triangles into lines or points, or lines
    /// into points.
    pub polygon_mode: PolygonMode,

    /// Specifies whether front faces or back faces should be discarded, or none, or both.
    pub cull_mode: CullMode,

    /// Specifies which triangle orientation corresponds to the front or the triangle.
    pub front_face: FrontFace,

    /// Width, in pixels, of lines when drawing lines.
    ///
    /// If you pass `None`, then this state will be considered as dynamic and the line width will
    /// need to be set when you build the command buffer.
    pub line_width: Option<f32>,

    // TODO: clean this
    pub depthBiasEnable: bool,
    pub depthBiasConstantFactor: f32,
    pub depthBiasClamp: f32,
    pub depthBiasSlopeFactor: f32,
}

impl Default for Rasterization {
    #[inline]
    fn default() -> Rasterization {
        Rasterization {
            depth_clamp: false,
            rasterizer_discard: false,
            polygon_mode: Default::default(),
            cull_mode: Default::default(),
            front_face: Default::default(),
            line_width: Some(1.0),

            depthBiasEnable: false,
            depthBiasConstantFactor: 0.0,
            depthBiasClamp: 0.0,
            depthBiasSlopeFactor: 0.0,
        }
    }
}

/// Specifies the culling mode.
///
/// This setting works in pair with `front_face`. The `front_face` setting tells the GPU whether
/// clockwise or counter-clockwise correspond to the front and the back of each triangle. Then
/// `cull_mode` lets you specify whether front faces should be discarded, back faces should be
/// discarded, or none, or both.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum CullMode {
    None = vk::CULL_MODE_NONE,
    Front = vk::CULL_MODE_FRONT_BIT,
    Back = vk::CULL_MODE_BACK_BIT,
    FrontAndBack = vk::CULL_MODE_FRONT_AND_BACK,
}

impl Default for CullMode {
    #[inline]
    fn default() -> CullMode {
        CullMode::None
    }
}

/// Specifies which triangle orientation corresponds to the front or the triangle.
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum FrontFace {
    /// Triangles whose vertices are oriented counter-clockwise on the screen will be considered
    /// as facing their front. Otherwise they will be considered as facing their back.
    CounterClockwise = vk::FRONT_FACE_COUNTER_CLOCKWISE,

    /// Triangles whose vertices are oriented clockwise on the screen will be considered
    /// as facing their front. Otherwise they will be considered as facing their back.
    Clockwise = vk::FRONT_FACE_CLOCKWISE,
}

impl Default for FrontFace {
    #[inline]
    fn default() -> FrontFace {
        FrontFace::CounterClockwise
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum PolygonMode {
    Fill = vk::POLYGON_MODE_FILL,
    Line = vk::POLYGON_MODE_LINE,
    Point = vk::POLYGON_MODE_POINT,
}

impl Default for PolygonMode {
    #[inline]
    fn default() -> PolygonMode {
        PolygonMode::Fill
    }
}
