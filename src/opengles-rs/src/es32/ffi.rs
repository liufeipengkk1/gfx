use super::data_struct::*;
use super::es20::data_struct::*;

pub type GLDEBUGPROC = ::std::option::Option<
    unsafe extern "C" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *const ::std::os::raw::c_void,
    ),
>;

extern "C" {

    pub fn glBlendBarrier();

    pub fn glCopyImageSubData(
        srcName: GLuint,
        srcTarget: GLenum,
        srcLevel: GLint,
        srcX: GLint,
        srcY: GLint,
        srcZ: GLint,
        dstName: GLuint,
        dstTarget: GLenum,
        dstLevel: GLint,
        dstX: GLint,
        dstY: GLint,
        dstZ: GLint,
        srcWidth: GLsizei,
        srcHeight: GLsizei,
        srcDepth: GLsizei,
    );

    pub fn glDebugMessageControl(
        source: GLenum,
        type_: GLenum,
        severity: GLenum,
        count: GLsizei,
        ids: *const GLuint,
        enabled: GLboolean,
    );

    pub fn glDebugMessageInsert(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        buf: *const GLchar,
    );

    pub fn glDebugMessageCallback(callback: GLDEBUGPROC, userParam: *const ::std::os::raw::c_void);

    pub fn glGetDebugMessageLog(
        count: GLuint,
        bufSize: GLsizei,
        sources: *mut GLenum,
        types: *mut GLenum,
        ids: *mut GLuint,
        severities: *mut GLenum,
        lengths: *mut GLsizei,
        messageLog: *mut GLchar,
    ) -> GLuint;

    pub fn glPushDebugGroup(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);

    pub fn glPopDebugGroup();

    pub fn glObjectLabel(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar);

    pub fn glGetObjectLabel(
        identifier: GLenum,
        name: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    );

    pub fn glObjectPtrLabel(
        ptr: *const ::std::os::raw::c_void,
        length: GLsizei,
        label: *const GLchar,
    );

    pub fn glGetObjectPtrLabel(
        ptr: *const ::std::os::raw::c_void,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    );

    pub fn glGetPointerv(pname: GLenum, params: *mut *mut ::std::os::raw::c_void);

    pub fn glEnablei(target: GLenum, index: GLuint);

    pub fn glDisablei(target: GLenum, index: GLuint);

    pub fn glBlendEquationi(buf: GLuint, mode: GLenum);

    pub fn glBlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);

    pub fn glBlendFunci(buf: GLuint, src: GLenum, dst: GLenum);

    pub fn glBlendFuncSeparatei(
        buf: GLuint,
        srcRGB: GLenum,
        dstRGB: GLenum,
        srcAlpha: GLenum,
        dstAlpha: GLenum,
    );

    pub fn glColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

    pub fn glIsEnabledi(target: GLenum, index: GLuint) -> GLboolean;

    pub fn glDrawElementsBaseVertex(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::std::os::raw::c_void,
        basevertex: GLint,
    );

    pub fn glDrawRangeElementsBaseVertex(
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::std::os::raw::c_void,
        basevertex: GLint,
    );

    pub fn glDrawElementsInstancedBaseVertex(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::std::os::raw::c_void,
        instancecount: GLsizei,
        basevertex: GLint,
    );

    pub fn glFramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);

    pub fn glPrimitiveBoundingBox(
        minX: GLfloat,
        minY: GLfloat,
        minZ: GLfloat,
        minW: GLfloat,
        maxX: GLfloat,
        maxY: GLfloat,
        maxZ: GLfloat,
        maxW: GLfloat,
    );

    pub fn glGetGraphicsResetStatus() -> GLenum;

    pub fn glReadnPixels(
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        bufSize: GLsizei,
        data: *mut ::std::os::raw::c_void,
    );

    pub fn glGetnUniformfv(
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLfloat,
    );

    pub fn glGetnUniformiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

    pub fn glGetnUniformuiv(
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLuint,
    );

    pub fn glMinSampleShading(value: GLfloat);

    pub fn glPatchParameteri(pname: GLenum, value: GLint);

    pub fn glTexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint);

    pub fn glTexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint);

    pub fn glGetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint);

    pub fn glGetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint);

    pub fn glSamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *const GLint);

    pub fn glSamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *const GLuint);

    pub fn glGetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint);

    pub fn glGetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint);

    pub fn glTexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint);

    pub fn glTexBufferRange(
        target: GLenum,
        internalformat: GLenum,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    );

    pub fn glTexStorage3DMultisample(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean,
    );
}
