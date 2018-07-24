use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use gles::es20;
use gles::es31;
use gles::es30;
use gles::es32;

use outEv::env::{GLES_VERSION};
use hal::{buffer, image as i, Primitive};
use hal::format::Format;
use native::VertexAttribFunction;

/*
pub fn _image_kind_to_gl(kind: i::Kind) -> t::GLenum {
    match kind {
        i::Kind::D1(_) => gl::TEXTURE_1D,
        i::Kind::D1Array(_, _) => gl::TEXTURE_1D_ARRAY,
        i::Kind::D2(_, _, i::AaMode::Single) => gl::TEXTURE_2D,
        i::Kind::D2(_, _, _) => gl::TEXTURE_2D_MULTISAMPLE,
        i::Kind::D2Array(_, _, _, i::AaMode::Single) => gl::TEXTURE_2D_ARRAY,
        i::Kind::D2Array(_, _, _, _) => gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
        i::Kind::D3(_, _, _) => gl::TEXTURE_3D,
        i::Kind::Cube(_) => gl::TEXTURE_CUBE_MAP,
        i::Kind::CubeArray(_, _) => gl::TEXTURE_CUBE_MAP_ARRAY,
    }
}*/

pub fn filter_to_gl(version:&GLES_VERSION, mag: i::Filter, min: i::Filter, mip: i::Filter) -> (es20d::GLenum, es20d::GLenum) {
    use hal::image::Filter::*;

    let mag_filter = match mag {
        Nearest => es20d::GL_NEAREST,
        Linear => es20d::GL_LINEAR,
    };

    let min_filter = match (min, mip) {
        (Nearest, Nearest) => es20d::GL_NEAREST_MIPMAP_NEAREST,
        (Nearest, Linear) => es20d::GL_NEAREST_MIPMAP_LINEAR,
        (Linear, Nearest) => es20d::GL_LINEAR_MIPMAP_NEAREST,
        (Linear, Linear) => es20d::GL_LINEAR_MIPMAP_LINEAR,
    };

    (min_filter, mag_filter)
}

pub fn wrap_to_gl(version:&GLES_VERSION,w: i::WrapMode) -> es20d::GLenum {
    match w {
        i::WrapMode::Tile   => es20d::GL_REPEAT,
        i::WrapMode::Mirror => es20d::GL_MIRRORED_REPEAT,
        i::WrapMode::Clamp  => es20d::GL_CLAMP_TO_EDGE,
        i::WrapMode::Border => {
            match version {
                GLES_VERSION::ES32 => es32d::GL_CLAMP_TO_BORDER,
                GLES_VERSION::ES30 | GLES_VERSION::ES31 | GLES_VERSION::ES20
                =>{
                    info!("conv:wrap_to_gl: Border is not supported by es20,\
                    and feed GL_CLAMP_TO_EDGE instead");
                    es20d::GL_CLAMP_TO_EDGE
                }
            }
        },
    }
}

pub fn buffer_usage_to_gl_target(version: &GLES_VERSION, usage: buffer::Usage) -> Option<es20d::GLenum> {
    use self::buffer::Usage;
    match usage & (Usage::UNIFORM | Usage::INDEX | Usage::VERTEX | Usage::INDIRECT) {
        Usage::UNIFORM => {
            match version {
                GLES_VERSION::ES20 => {
                    info!("conv:buffer_usage_to_gl_target: Uniform Buffer is not support by gles20");
                    unimplemented!()
                },
                _ => {
                    Some(es30d::GL_UNIFORM_BUFFER)
                }
            }
        },
        Usage::INDEX => Some(es20d::GL_ELEMENT_ARRAY_BUFFER),
        Usage::VERTEX => Some(es20d::GL_ARRAY_BUFFER),
        Usage::INDIRECT => {
            info!("conv:buffer_usage_to_gl_target: INDIRECT is not support by gl");
            unimplemented!()
        },
        _ => None
    }
}

pub fn primitive_to_gl_primitive(version: &GLES_VERSION, primitive: Primitive) -> es20d::GLenum {
    match version {
        GLES_VERSION::ES20 |  GLES_VERSION::ES31 |  GLES_VERSION::ES30
        => {
            match primitive {
                Primitive::PointList => es20d::GL_POINTS,
                Primitive::LineList => es20d::GL_LINES,
                Primitive::LineStrip => es20d::GL_LINE_STRIP,
                Primitive::TriangleList => es20d::GL_TRIANGLES,
                Primitive::TriangleStrip => es20d::GL_TRIANGLE_STRIP,
                //30
                Primitive::LineListAdjacency |
                Primitive::LineStripAdjacency |
                Primitive::TriangleListAdjacency |
                Primitive::TriangleStripAdjacency |
                Primitive::PatchList(_)  => {
                    info!("conv: primitive_to_gl_primitive: LineListAdjacency | \
                    LineStripAdjacency | TriangleListAdjacency | TriangleStripAdjacency| \
                    PatchList are not supported by es20/es30/es31");
                    unimplemented!()
                }
            }
        },
        GLES_VERSION::ES32 => {
            match primitive {
                Primitive::PointList => es20d::GL_POINTS,
                Primitive::LineList => es20d::GL_LINES,
                Primitive::LineStrip => es20d::GL_LINE_STRIP,
                Primitive::TriangleList => es20d::GL_TRIANGLES,
                Primitive::TriangleStrip => es20d::GL_TRIANGLE_STRIP,
                //32 attribute
                Primitive::LineListAdjacency => es32d::GL_LINES_ADJACENCY,
                Primitive::LineStripAdjacency => es32d::GL_LINE_STRIP_ADJACENCY,
                Primitive::TriangleListAdjacency => es32d::GL_TRIANGLES_ADJACENCY,
                Primitive::TriangleStripAdjacency => es32d::GL_TRIANGLE_STRIP_ADJACENCY,
                Primitive::PatchList(_) => es32d::GL_PATCHES,
            }
        }
    }
}

pub fn format_to_gl_format(version: &GLES_VERSION, format: Format) -> Option<(es20d::GLint, es20d::GLenum, VertexAttribFunction)> {
    use hal::format::Format::*;
    use native::VertexAttribFunction::*;
    let _ = Double; //mark as used
    // TODO: Add more formats and error handling for `None`
    let format = match format {
        R8Uint => (1, es20d::GL_UNSIGNED_BYTE, Integer),
        R8Int => (1, es20d::GL_BYTE, Integer),
        Rg8Uint => (2, es20d::GL_UNSIGNED_BYTE, Integer),
        Rg8Int => (2, es20d::GL_BYTE, Integer),
        Rgba8Uint => (4, es20d::GL_UNSIGNED_BYTE, Integer),
        Rgba8Int => (4, es20d::GL_BYTE, Integer),
        R16Uint => (1, es20d::GL_UNSIGNED_SHORT, Integer),
        R16Int => (1, es20d::GL_SHORT, Integer),
        //
        R16Float => {
            match version {
                GLES_VERSION::ES20=>{

                },
                _ => {
                    (1, es30d::GL_HALF_FLOAT, Float)
                }
            }
        },
        Rg16Uint => (2, es20d::GL_UNSIGNED_SHORT, Integer),
        Rg16Int => (2, es20d::GL_SHORT, Integer),
        //
        Rg16Float => {
            match version {
                GLES_VERSION::ES20 => {
                    info!("conv: format_to_gl_format: half float is not supported by es20");
                    unimplemented!()
                },
                _ => {
                    (2, es30d::GL_HALF_FLOAT, Float)
                }
            }
        },
        Rgba16Uint => (4, es20d::GL_UNSIGNED_SHORT, Integer),
        Rgba16Int => (4, es20d::GL_SHORT, Integer),

        Rgba16Float => {
            match version {
                GLES_VERSION::ES20 => {
                    info!("conv: format_to_gl_format: half float is not supported by es20");
                    unimplemented!()
                },
                _ => {
                    (4, es30d::GL_HALF_FLOAT, Float)
                }
            }
        },

        R32Uint => (1, es20d::GL_UNSIGNED_INT, Integer),
        R32Int => (1, es20d::GL_INT, Integer),
        R32Float => (1, es20d::GL_FLOAT, Float),
        Rg32Uint => (2, es20d::GL_UNSIGNED_INT, Integer),
        Rg32Int => (2, es20d::GL_INT, Integer),
        Rg32Float => (2, es20d::GL_FLOAT, Float),
        Rgb32Uint => (3, es20d::GL_UNSIGNED_INT, Integer),
        Rgb32Int => (3, es20d::GL_INT, Integer),
        Rgb32Float => (3, es20d::GL_FLOAT, Float),
        Rgba32Uint => (4, es20d::GL_UNSIGNED_INT, Integer),
        Rgba32Int => (4, es20d::GL_INT, Integer),
        Rgba32Float => (4, es20d::GL_FLOAT, Float),

        _ => return None,
    };

    Some(format)
}
