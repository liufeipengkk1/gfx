//#![feature(target_os = "macos", target_os = "ios")]

use super::data_struct::*;
use super::es20::data_struct::*;

extern "C" {
    pub fn glReadBuffer(mode: GLenum);

    pub fn glDrawRangeElements(
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
    );

    pub fn glTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid,
    );

    pub fn glTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid,
    );

    pub fn glCopyTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );

    pub fn glCompressedTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const GLvoid,
    );

    pub fn glCompressedTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const GLvoid,
    );

    pub fn glGenQueries(n: GLsizei, ids: *mut GLuint);

    pub fn glDeleteQueries(n: GLsizei, ids: *const GLuint);

    pub fn glIsQuery(id: GLuint) -> GLboolean;

    pub fn glBeginQuery(target: GLenum, id: GLuint);

    pub fn glEndQuery(target: GLenum);

    pub fn glGetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint);

    pub fn glGetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint);

    pub fn glUnmapBuffer(target: GLenum) -> GLboolean;

    pub fn glGetBufferPointerv(target: GLenum, pname: GLenum, params: *mut *mut GLvoid);

    pub fn glDrawBuffers(n: GLsizei, bufs: *const GLenum);

    pub fn glUniformMatrix2x3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glUniformMatrix3x2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glUniformMatrix2x4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glUniformMatrix4x2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glUniformMatrix3x4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glUniformMatrix4x3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glBlitFramebuffer(
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    );

    pub fn glRenderbufferStorageMultisample(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );

    pub fn glFramebufferTextureLayer(
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    );

    pub fn glMapBufferRange(
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) -> *mut GLvoid;

    pub fn glFlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr);

    pub fn glBindVertexArray(array: GLuint);

    pub fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint);

    pub fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint);

    pub fn glIsVertexArray(array: GLuint) -> GLboolean;

    pub fn glGetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint);

    pub fn glEndTransformFeedback();

    pub fn glBindBufferRange(
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    );

    pub fn glBindBufferBase(target: GLenum, index: GLuint, buffer: GLuint);

    pub fn glTransformFeedbackVaryings(
        program: GLuint,
        count: GLsizei,
        varyings: *const *const GLchar,
        bufferMode: GLenum,
    );

    pub fn glGetTransformFeedbackVarying(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLsizei,
        type_: *mut GLenum,
        name: *mut GLchar,
    );

    pub fn glVertexAttribIPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const GLvoid,
    );

    pub fn glGetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint);

    pub fn glGetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint);

    pub fn glVertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

    pub fn glVertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

    pub fn glVertexAttribI4iv(index: GLuint, v: *const GLint);

    pub fn glVertexAttribI4uiv(index: GLuint, v: *const GLuint);

    pub fn glGetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint);

    pub fn glGetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint;

    pub fn glUniform1ui(location: GLint, v0: GLuint);

    pub fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint);

    pub fn glUniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

    pub fn glUniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

    pub fn glUniform1uiv(location: GLint, count: GLsizei, value: *const GLuint);

    pub fn glUniform2uiv(location: GLint, count: GLsizei, value: *const GLuint);

    pub fn glUniform3uiv(location: GLint, count: GLsizei, value: *const GLuint);

    pub fn glUniform4uiv(location: GLint, count: GLsizei, value: *const GLuint);

    pub fn glClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint);

    pub fn glClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);

    pub fn glClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);

    pub fn glClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);

    pub fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte;

    pub fn glCopyBufferSubData(
        readTarget: GLenum,
        writeTarget: GLenum,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr,
    );

    pub fn glGetUniformIndices(
        program: GLuint,
        uniformCount: GLsizei,
        uniformNames: *const *const GLchar,
        uniformIndices: *mut GLuint,
    );

    pub fn glGetActiveUniformsiv(
        program: GLuint,
        uniformCount: GLsizei,
        uniformIndices: *const GLuint,
        pname: GLenum,
        params: *mut GLint,
    );

    pub fn glGetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;

    pub fn glGetActiveUniformBlockiv(
        program: GLuint,
        uniformBlockIndex: GLuint,
        pname: GLenum,
        params: *mut GLint,
    );

    pub fn glUniformBlockBinding(
        program: GLuint,
        uniformBlockIndex: GLuint,
        uniformBlockBinding: GLuint,
    );

    pub fn glDrawArraysInstanced(
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
    );

    pub fn glDrawElementsInstanced(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
        instancecount: GLsizei,
    );

    pub fn glFenceSync(condition: GLenum, flags: GLbitfield) -> GLsync;

    pub fn glIsSync(sync: GLsync) -> GLboolean;

    pub fn glDeleteSync(sync: GLsync);

    pub fn glClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;

    pub fn glWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64);

    pub fn glGetInteger64v(pname: GLenum, params: *mut GLint64);

    pub fn glGetSynciv(
        sync: GLsync,
        pname: GLenum,
        bufSize: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint,
    );

    pub fn glGetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64);

    pub fn glGetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64);

    pub fn glGenSamplers(count: GLsizei, samplers: *mut GLuint);

    pub fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint);

    pub fn glIsSampler(sampler: GLuint) -> GLboolean;

    pub fn glBindSampler(unit: GLuint, sampler: GLuint);

    pub fn glSamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint);

    pub fn glSamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint);

    pub fn glSamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat);

    pub fn glSamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat);

    pub fn glGetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint);

    pub fn glGetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat);

    pub fn glVertexAttribDivisor(index: GLuint, divisor: GLuint);

    pub fn glBindTransformFeedback(target: GLenum, id: GLuint);

    pub fn glDeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint);

    pub fn glGenTransformFeedbacks(n: GLsizei, ids: *mut GLuint);

    pub fn glIsTransformFeedback(id: GLuint) -> GLboolean;

    pub fn glPauseTransformFeedback();

    pub fn glResumeTransformFeedback();

    pub fn glGetProgramBinary(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        binaryFormat: *mut GLenum,
        binary: *mut GLvoid,
    );

    pub fn glProgramBinary(
        program: GLuint,
        binaryFormat: GLenum,
        binary: *const GLvoid,
        length: GLsizei,
    );

    pub fn glProgramParameteri(program: GLuint, pname: GLenum, value: GLint);

    pub fn glInvalidateFramebuffer(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
    );

    pub fn glInvalidateSubFramebuffer(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );

    pub fn glTexStorage2D(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );

    pub fn glTexStorage3D(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    );

    pub fn glGetInternalformativ(
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
        bufSize: GLsizei,
        params: *mut GLint,
    );
}
