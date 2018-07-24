#![allow(dead_code)] //TODO: remove

use hal::{ColorSlot};
use hal::pso;
use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use gles::es20;
use gles::es31;
use gles::es30;
use gles::es32;

use outEv::env::{GLES_VERSION};

use smallvec::SmallVec;

pub fn bind_polygon_mode(version:&GLES_VERSION, mode: pso::PolygonMode, bias: Option<pso::DepthBias>) {

    //ES support FILL only
    use hal::pso::PolygonMode::*;

    //let (gl_draw, gl_offset) = match mode {
    //    Point => (gl::POINT, gl::POLYGON_OFFSET_POINT),
    //    Line(width) => {
    //        unsafe { gl.LineWidth(width) };
    //        (gl::LINE, gl::POLYGON_OFFSET_LINE)
    //    },
    //    Fill => (gl::FILL, gl::POLYGON_OFFSET_FILL),
    //};


    //unsafe { gl.PolygonMode(gl::FRONT_AND_BACK, gl_draw) };
    let gl_offset = es20d::GL_POLYGON_OFFSET_FILL;
    match bias {
        Some(bias) =>  {
            es20::wrapper::enable(gl_offset);
            es20::wrapper::polygon_offset(bias.slope_factor as _, bias.const_factor as _);
            //gl.PolygonOffset(bias.slope_factor as _, bias.const_factor as _);
        },
        None => unsafe {
            es20::wrapper::disable(gl_offset)
        },
    }
}

pub fn bind_rasterizer(version:&GLES_VERSION, r: &pso::Rasterizer, is_embedded: bool) {
    use hal::pso::FrontFace::*;

    unsafe {
        es20::wrapper::front_face(match r.front_face {
            Clockwise => es20d::GL_CW,
            CounterClockwise => es20d::GL_CCW,
        })
    };

    if !r.cull_face.is_empty() {
        es20::wrapper::enable(es20d::GL_CULL_FACE);
        es20::wrapper::cull_face(match r.cull_face {
                pso::Face::FRONT => es20d::GL_FRONT,
                pso::Face::BACK => es20d::GL_BACK,
                _ => es20d::GL_FRONT_AND_BACK,
            });
    } else {
        es20::wrapper::disable(es20d::GL_CULL_FACE);
    }

    if !is_embedded {
        bind_polygon_mode(version, r.polygon_mode, r.depth_bias);
        //match false { //TODO
        //    true => unsafe { gl.Enable(gl::MULTISAMPLE) },
         //   false => unsafe { gl.Disable(gl::MULTISAMPLE) },
        //}
    }
}

pub fn bind_draw_color_buffers(version:&GLES_VERSION, num: usize) {

    let mut attachments : Vec<es20d::GLenum> = Vec::new();
    match version {
        GLES_VERSION::ES20 => {
            attachments.push(es20d::GL_COLOR_ATTACHMENT0);
        },
        _ => {
            loop {
                if i > num - 1 {
                    break;
                }
                attachments.push(es30d::GL_COLOR_ATTACHMENT1 + i);
                i = i + 1;
            }
        }
    }

    //let attachments: SmallVec<[gl::types::GLenum; 16]> =
    //    (0..num).map(|x| gl::COLOR_ATTACHMENT0 + x as u32).collect();
    match version {
        GLES_VERSION::ES20 => {
           //
        },
        _ => {
            unsafe {
                es30::ffi::glDrawBuffers(num as es20d::GLint, attachments[0..num].as_ptr());
            };
        }
    }
}

pub fn map_comparison(cmp: pso::Comparison) -> es20d::GLenum {
    use hal::pso::Comparison::*;
    match cmp {
        Never        => es20d::GL_NEVER,
        Less         => es20d::GL_LESS,
        LessEqual    => es20d::GL_LEQUAL,
        Equal        => es20d::GL_EQUAL,
        GreaterEqual => es20d::GL_GEQUAL,
        Greater      => es20d::GL_GREATER,
        NotEqual     => es20d::GL_NOTEQUAL,
        Always       => es20d::GL_ALWAYS,
    }
}

pub fn bind_depth(version:&GLES_VERSION, depth: &pso::DepthTest) {
    match *depth {
        pso::DepthTest::On { fun, write } =>  {
            es20::wrapper::enable(es20d::GL_DEPTH_TEST);
            es20::wrapper::depth_func(map_comparison(fun));
            es20::wrapper::depth_mask(write as _);
        },
        pso::DepthTest::Off =>  {
            es20::wrapper::disable(es20d::GL_DEPTH_TEST);
        },
    }
}

fn map_operation(op: pso::StencilOp) -> es20d::GLenum {
    use hal::pso::StencilOp::*;
    match op {
        Keep          => es20d::GL_KEEP,
        Zero          => es20d::GL_ZERO,
        Replace       => es20d::GL_REPLACE,
        IncrementClamp=> es20d::GL_INCR,
        IncrementWrap => es20d::GL_INCR_WRAP,
        DecrementClamp=> es20d::GL_DECR,
        DecrementWrap => es20d::GL_DECR_WRAP,
        Invert        => es20d::GL_INVERT,
    }
}

pub fn bind_stencil(
    version: &GLES_VERSION,
    stencil: &pso::StencilTest,
    (ref_front, ref_back): (pso::StencilValue, pso::StencilValue),
    cull: Option<pso::Face>,
) {
    fn bind_side(version:&GLES_VERSION, face: es20d::GLenum, side: &pso::StencilFace, ref_value: pso::StencilValue) {

        let mr = match side.mask_read {
            pso::State::Static(v) => v,
            pso::State::Dynamic => !0,
        };
        let mw = match side.mask_write {
            pso::State::Static(v) => v,
            pso::State::Dynamic => !0,
        };
        es20::wrapper::stencil_func_separate(face, map_comparison(side.fun), ref_value as _, mr);
        es20::wrapper::stencil_mask_separate(face, mw);
        es20::wrapper::stencil_op_separate(face, map_operation(side.op_fail), map_operation(side.op_depth_fail), map_operation(side.op_pass));
    }
    match *stencil {
        pso::StencilTest::On { ref front, ref back } => {
            es20::wrapper::enable(es20d::GL_STENCIL_TEST);
            if let Some(cf) = cull {
                if !cf.contains(pso::Face::FRONT) {
                    bind_side(version, es20d::GL_FRONT, front, ref_front);
                }
                if !cf.contains(pso::Face::BACK) {
                    bind_side(version, es20d::GL_BACK, back, ref_back);
                }
            }
        }
        pso::StencilTest::Off =>  {
            es20::wrapper::disable(es20d::GL_STENCIL_TEST);
        },
    }
}

fn map_factor(version:&GLES_VERSION, factor: pso::Factor) -> es20d::GLenum {
    use hal::pso::Factor::*;
    match factor {
        Zero => es20d::GL_ZERO,
        One => es20d::GL_ONE,
        SrcColor => es20d::GL_SRC_COLOR,
        OneMinusSrcColor => es20d::GL_ONE_MINUS_SRC_COLOR,
        DstColor => es20d::GL_DST_COLOR,
        OneMinusDstColor => es20d::GL_ONE_MINUS_DST_COLOR,
        SrcAlpha => es20d::GL_SRC_ALPHA,
        OneMinusSrcAlpha => es20d::GL_ONE_MINUS_SRC_ALPHA,
        DstAlpha => es20d::GL_DST_ALPHA,
        OneMinusDstAlpha => es20d::GL_ONE_MINUS_DST_ALPHA,
        ConstColor => es20d::GL_CONSTANT_COLOR,
        OneMinusConstColor => es20d::GL_ONE_MINUS_CONSTANT_COLOR,
        ConstAlpha => es20d::GL_CONSTANT_ALPHA,
        OneMinusConstAlpha => es20d::GL_ONE_MINUS_CONSTANT_ALPHA,
        SrcAlphaSaturate => es20d::GL_SRC_ALPHA_SATURATE,

        OneMinusSrc1Color | Src1Alpha |
        OneMinusSrc1Alpha| Src1Color => {
          match version {
              GLES_VERSION::ES30 | GLES_VERSION::ES31 | GLES_VERSION::ES32 | GLES_VERSION::ES20
              => {
                  panic!("state: map_factor :OPENGL ES don't support")
                  //unimplemented!();
              }
          }
        }
    }
}

fn map_blend_op(version:&GLES_VERSION, operation: pso::BlendOp) -> (es20d::GLenum, es20d::GLenum, es20d::GLenum) {
    match operation {
        pso::BlendOp::Add { src, dst }    => (es20d::GL_FUNC_ADD,              map_factor(version,src), map_factor(version,dst)),
        pso::BlendOp::Sub { src, dst }    => (es20d::GL_FUNC_SUBTRACT,         map_factor(version,src), map_factor(version,dst)),
        pso::BlendOp::RevSub { src, dst } => (es20d::GL_FUNC_REVERSE_SUBTRACT, map_factor(version,src), map_factor(version,dst)),
        pso::BlendOp::Min => (es20d::GL_MIN, es20d::GL_ZERO, es20d::GL_ZERO),
        pso::BlendOp::Max => (es20d::GL_MAX, es20d::GL_ZERO, es20d::GL_ZERO),
    }
}

pub fn bind_blend(version:&GLES_VERSION, desc: &pso::ColorBlendDesc) {
    use hal::pso::ColorMask as Cm;

    match desc.1 {
        pso::BlendState::On { color, alpha } => {
            let (color_eq, color_src, color_dst) = map_blend_op(version, color);
            let (alpha_eq, alpha_src, alpha_dst) = map_blend_op(version, alpha);
            es20::wrapper::enable(es20d::GL_BLEND);
            es20::wrapper::blend_equation_separate(color_eq, alpha_eq);
            es20::wrapper::blend_func_separate(color_src, color_dst, alpha_src, alpha_dst);
        },
        pso::BlendState::Off => {
            es20::wrapper::disable(es20d::GL_BLEND);
        },
    };

     { es20::wrapper::color_mask(
        desc.0.contains(Cm::RED) as _,
        desc.0.contains(Cm::GREEN) as _,
        desc.0.contains(Cm::BLUE) as _,
        desc.0.contains(Cm::ALPHA) as _,
    )};
}

pub fn bind_blend_slot(version:&GLES_VERSION, slot: ColorSlot, desc: &pso::ColorBlendDesc) {
    use hal::pso::ColorMask as Cm;

    match desc.1 {
        pso::BlendState::On { color, alpha } => unsafe {
            let (color_eq, color_src, color_dst) = map_blend_op(version,color);
            let (alpha_eq, alpha_src, alpha_dst) = map_blend_op(version,alpha);
            //Note: using ARB functions as they are more compatible

            match version {
                GLES_VERSION::ES20 | GLES_VERSION::ES30 | GLES_VERSION::ES31=>{
                    //do nothing
                    //es20::wrapper::enable(es20d::GL_BLEND);
                    //es20::wrapper::blend_equation_separate(color_eq, alpha_eq);
                    //es20::wrapper::blend_func_separate(color_src, color_dst, alpha_src, alpha_dst);
                },
                GLES_VERSION::ES32 => {
                    unsafe {
                        es32::ffi::glEnablei(es20d::GL_BLEND, slot as _);
                        es32::ffi::glBlendEquationSeparatei(slot as _, color_eq, alpha_eq);
                        es32::ffi::glBlendFuncSeparatei(slot as _, color_src, color_dst, alpha_src, alpha_dst);
                    }
                }
            }
        },
        pso::BlendState::Off => unsafe {
            match version {
                GLES_VERSION::ES20 | GLES_VERSION::ES30 | GLES_VERSION::ES31 => {
                   // es20::wrapper::disenable(es20d::GL_BLEND);
                },
                GLES_VERSION::ES32 => {
                    unsafe {
                        es32::ffi::glDisanablei(es20d::GL_BLEND, slot as _);
                    }
                }
            }
        },
    };

    match version {
        GLES_VERSION::ES20 | GLES_VERSION::ES30 | GLES_VERSION::ES31=>{
            //es20::wrapper::color_mask( desc.0.contains(Cm::RED) as _,
            //                           desc.0.contains(Cm::GREEN) as _,
            //                           desc.0.contains(Cm::BLUE) as _,
            //                           desc.0.contains(Cm::ALPHA) as _,);
        },
        GLES_VERSION::ES32 => {
            unsafe { es32::ffi::glColorMaski(slot as _,
                                   desc.0.contains(Cm::RED) as _,
                                   desc.0.contains(Cm::GREEN) as _,
                                   desc.0.contains(Cm::BLUE) as _,
                                   desc.0.contains(Cm::ALPHA) as _,
            )};
        }
    }
}

pub fn unlock_color_mask(version:&GLES_VERSION) {
     es20::wrapper::color_mask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
}

pub fn set_blend_color(version:&GLES_VERSION, color: pso::ColorValue) {
    es20::wrapper::blend_color(color[0], color[1], color[2], color[3])
}
