use super::data_struct::*;
use super::es20::data_struct::*;

extern "C" {
    pub fn glDispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);

    pub fn glDispatchComputeIndirect(indirect: GLintptr);

    pub fn glDrawArraysIndirect(mode: GLenum, indirect: *const ::std::os::raw::c_void);

    pub fn glDrawElementsIndirect(
        mode: GLenum,
        type_: GLenum,
        indirect: *const ::std::os::raw::c_void,
    );

    pub fn glFramebufferParameteri(target: GLenum, pname: GLenum, param: GLint);

    pub fn glGetFramebufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);

    pub fn glGetProgramInterfaceiv(
        program: GLuint,
        programInterface: GLenum,
        pname: GLenum,
        params: *mut GLint,
    );

    pub fn glGetProgramResourceIndex(
        program: GLuint,
        programInterface: GLenum,
        name: *const GLchar,
    ) -> GLuint;

    pub fn glGetProgramResourceName(
        program: GLuint,
        programInterface: GLenum,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar,
    );

    pub fn glGetProgramResourceiv(
        program: GLuint,
        programInterface: GLenum,
        index: GLuint,
        propCount: GLsizei,
        props: *const GLenum,
        bufSize: GLsizei,
        length: *mut GLsizei,
        params: *mut GLint,
    );

    pub fn glGetProgramResourceLocation(
        program: GLuint,
        programInterface: GLenum,
        name: *const GLchar,
    ) -> GLint;

    pub fn glUseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint);

    pub fn glActiveShaderProgram(pipeline: GLuint, program: GLuint);

    pub fn glCreateShaderProgramv(
        type_: GLenum,
        count: GLsizei,
        strings: *const *const GLchar,
    ) -> GLuint;

    pub fn glBindProgramPipeline(pipeline: GLuint);

    pub fn glDeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint);

    pub fn glGenProgramPipelines(n: GLsizei, pipelines: *mut GLuint);

    pub fn glIsProgramPipeline(pipeline: GLuint) -> GLboolean;

    pub fn glGetProgramPipelineiv(pipeline: GLuint, pname: GLenum, params: *mut GLint);

    pub fn glProgramUniform1i(program: GLuint, location: GLint, v0: GLint);

    pub fn glProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint);

    pub fn glProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);

    pub fn glProgramUniform4i(
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint,
    );

    pub fn glProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint);

    pub fn glProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);

    pub fn glProgramUniform3ui(
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    );

    pub fn glProgramUniform4ui(
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    );

    pub fn glProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat);

    pub fn glProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);

    pub fn glProgramUniform3f(
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
    );

    pub fn glProgramUniform4f(
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    );

    pub fn glProgramUniform1iv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    );

    pub fn glProgramUniform2iv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    );

    pub fn glProgramUniform3iv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    );

    pub fn glProgramUniform4iv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    );

    pub fn glProgramUniform1uiv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    );

    pub fn glProgramUniform2uiv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    );

    pub fn glProgramUniform3uiv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    );

    pub fn glProgramUniform4uiv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    );

    pub fn glProgramUniform1fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    );

    pub fn glProgramUniform2fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    );

    pub fn glProgramUniform3fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    );

    pub fn glProgramUniform4fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    );

    pub fn glProgramUniformMatrix2fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glProgramUniformMatrix3fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glProgramUniformMatrix4fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glProgramUniformMatrix2x3fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glProgramUniformMatrix3x2fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glProgramUniformMatrix2x4fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glProgramUniformMatrix4x2fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glProgramUniformMatrix3x4fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glProgramUniformMatrix4x3fv(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );

    pub fn glValidateProgramPipeline(pipeline: GLuint);

    pub fn glGetProgramPipelineInfoLog(
        pipeline: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );

    pub fn glBindImageTexture(
        unit: GLuint,
        texture: GLuint,
        level: GLint,
        layered: GLboolean,
        layer: GLint,
        access: GLenum,
        format: GLenum,
    );

    pub fn glGetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean);

    pub fn glMemoryBarrier(barriers: GLbitfield);

    pub fn glMemoryBarrierByRegion(barriers: GLbitfield);

    pub fn glTexStorage2DMultisample(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    );

    pub fn glGetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat);

    pub fn glSampleMaski(maskNumber: GLuint, mask: GLbitfield);

    pub fn glGetTexLevelParameteriv(
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLint,
    );

    pub fn glGetTexLevelParameterfv(
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat,
    );

    pub fn glBindVertexBuffer(
        bindingindex: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    );

    pub fn glVertexAttribFormat(
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        relativeoffset: GLuint,
    );

    pub fn glVertexAttribIFormat(
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        relativeoffset: GLuint,
    );

    pub fn glVertexAttribBinding(attribindex: GLuint, bindingindex: GLuint);

    pub fn glVertexBindingDivisor(bindingindex: GLuint, divisor: GLuint);

}
