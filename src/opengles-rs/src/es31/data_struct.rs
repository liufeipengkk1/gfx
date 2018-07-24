use super::es20::data_struct::*;

pub const GL_COMPUTE_SHADER: GLuint = 37305;
pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: GLuint = 37307;
pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLuint = 37308;
pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: GLuint = 37309;
pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: GLuint = 33378;
pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: GLuint = 33379;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLuint = 33380;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: GLuint = 33381;
pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLuint = 33382;
pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLuint = 37099;
pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: GLuint = 37310;
pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: GLuint = 37311;
pub const GL_COMPUTE_WORK_GROUP_SIZE: GLuint = 33383;
pub const GL_DISPATCH_INDIRECT_BUFFER: GLuint = 37102;
pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: GLuint = 37103;
pub const GL_COMPUTE_SHADER_BIT: GLuint = 32;
pub const GL_DRAW_INDIRECT_BUFFER: GLuint = 36671;
pub const GL_DRAW_INDIRECT_BUFFER_BINDING: GLuint = 36675;
pub const GL_MAX_UNIFORM_LOCATIONS: GLuint = 33390;
pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: GLuint = 37648;
pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: GLuint = 37649;
pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: GLuint = 37651;
pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLuint = 37652;
pub const GL_MAX_FRAMEBUFFER_WIDTH: GLuint = 37653;
pub const GL_MAX_FRAMEBUFFER_HEIGHT: GLuint = 37654;
pub const GL_MAX_FRAMEBUFFER_SAMPLES: GLuint = 37656;
pub const GL_UNIFORM: GLuint = 37601;
pub const GL_UNIFORM_BLOCK: GLuint = 37602;
pub const GL_PROGRAM_INPUT: GLuint = 37603;
pub const GL_PROGRAM_OUTPUT: GLuint = 37604;
pub const GL_BUFFER_VARIABLE: GLuint = 37605;
pub const GL_SHADER_STORAGE_BLOCK: GLuint = 37606;
pub const GL_ATOMIC_COUNTER_BUFFER: GLuint = 37568;
pub const GL_TRANSFORM_FEEDBACK_VARYING: GLuint = 37620;
pub const GL_ACTIVE_RESOURCES: GLuint = 37621;
pub const GL_MAX_NAME_LENGTH: GLuint = 37622;
pub const GL_MAX_NUM_ACTIVE_VARIABLES: GLuint = 37623;
pub const GL_NAME_LENGTH: GLuint = 37625;
pub const GL_TYPE: GLuint = 37626;
pub const GL_ARRAY_SIZE: GLuint = 37627;
pub const GL_OFFSET: GLuint = 37628;
pub const GL_BLOCK_INDEX: GLuint = 37629;
pub const GL_ARRAY_STRIDE: GLuint = 37630;
pub const GL_MATRIX_STRIDE: GLuint = 37631;
pub const GL_IS_ROW_MAJOR: GLuint = 37632;
pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: GLuint = 37633;
pub const GL_BUFFER_BINDING: GLuint = 37634;
pub const GL_BUFFER_DATA_SIZE: GLuint = 37635;
pub const GL_NUM_ACTIVE_VARIABLES: GLuint = 37636;
pub const GL_ACTIVE_VARIABLES: GLuint = 37637;
pub const GL_REFERENCED_BY_VERTEX_SHADER: GLuint = 37638;
pub const GL_REFERENCED_BY_FRAGMENT_SHADER: GLuint = 37642;
pub const GL_REFERENCED_BY_COMPUTE_SHADER: GLuint = 37643;
pub const GL_TOP_LEVEL_ARRAY_SIZE: GLuint = 37644;
pub const GL_TOP_LEVEL_ARRAY_STRIDE: GLuint = 37645;
pub const GL_LOCATION: GLuint = 37646;
pub const GL_VERTEX_SHADER_BIT: GLuint = 1;
pub const GL_FRAGMENT_SHADER_BIT: GLuint = 2;
pub const GL_ALL_SHADER_BITS: GLuint = 4294967295;
pub const GL_PROGRAM_SEPARABLE: GLuint = 33368;
pub const GL_ACTIVE_PROGRAM: GLuint = 33369;
pub const GL_PROGRAM_PIPELINE_BINDING: GLuint = 33370;
pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: GLuint = 37569;
pub const GL_ATOMIC_COUNTER_BUFFER_START: GLuint = 37570;
pub const GL_ATOMIC_COUNTER_BUFFER_SIZE: GLuint = 37571;
pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLuint = 37580;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLuint = 37584;
pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLuint = 37585;
pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: GLuint = 37586;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: GLuint = 37590;
pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: GLuint = 37591;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLuint = 37592;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLuint = 37596;
pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: GLuint = 37593;
pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: GLuint = 37595;
pub const GL_MAX_IMAGE_UNITS: GLuint = 36664;
pub const GL_MAX_VERTEX_IMAGE_UNIFORMS: GLuint = 37066;
pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS: GLuint = 37070;
pub const GL_MAX_COMBINED_IMAGE_UNIFORMS: GLuint = 37071;
pub const GL_IMAGE_BINDING_NAME: GLuint = 36666;
pub const GL_IMAGE_BINDING_LEVEL: GLuint = 36667;
pub const GL_IMAGE_BINDING_LAYERED: GLuint = 36668;
pub const GL_IMAGE_BINDING_LAYER: GLuint = 36669;
pub const GL_IMAGE_BINDING_ACCESS: GLuint = 36670;
pub const GL_IMAGE_BINDING_FORMAT: GLuint = 36974;
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLuint = 1;
pub const GL_ELEMENT_ARRAY_BARRIER_BIT: GLuint = 2;
pub const GL_UNIFORM_BARRIER_BIT: GLuint = 4;
pub const GL_TEXTURE_FETCH_BARRIER_BIT: GLuint = 8;
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: GLuint = 32;
pub const GL_COMMAND_BARRIER_BIT: GLuint = 64;
pub const GL_PIXEL_BUFFER_BARRIER_BIT: GLuint = 128;
pub const GL_TEXTURE_UPDATE_BARRIER_BIT: GLuint = 256;
pub const GL_BUFFER_UPDATE_BARRIER_BIT: GLuint = 512;
pub const GL_FRAMEBUFFER_BARRIER_BIT: GLuint = 1024;
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: GLuint = 2048;
pub const GL_ATOMIC_COUNTER_BARRIER_BIT: GLuint = 4096;
pub const GL_ALL_BARRIER_BITS: GLuint = 4294967295;
pub const GL_IMAGE_2D: GLuint = 36941;
pub const GL_IMAGE_3D: GLuint = 36942;
pub const GL_IMAGE_CUBE: GLuint = 36944;
pub const GL_IMAGE_2D_ARRAY: GLuint = 36947;
pub const GL_INT_IMAGE_2D: GLuint = 36952;
pub const GL_INT_IMAGE_3D: GLuint = 36953;
pub const GL_INT_IMAGE_CUBE: GLuint = 36955;
pub const GL_INT_IMAGE_2D_ARRAY: GLuint = 36958;
pub const GL_UNSIGNED_INT_IMAGE_2D: GLuint = 36963;
pub const GL_UNSIGNED_INT_IMAGE_3D: GLuint = 36964;
pub const GL_UNSIGNED_INT_IMAGE_CUBE: GLuint = 36966;
pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: GLuint = 36969;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: GLuint = 37063;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLuint = 37064;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLuint = 37065;
pub const GL_READ_ONLY: GLuint = 35000;
pub const GL_WRITE_ONLY: GLuint = 35001;
pub const GL_READ_WRITE: GLuint = 35002;
pub const GL_SHADER_STORAGE_BUFFER: GLuint = 37074;
pub const GL_SHADER_STORAGE_BUFFER_BINDING: GLuint = 37075;
pub const GL_SHADER_STORAGE_BUFFER_START: GLuint = 37076;
pub const GL_SHADER_STORAGE_BUFFER_SIZE: GLuint = 37077;
pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLuint = 37078;
pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLuint = 37082;
pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLuint = 37083;
pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLuint = 37084;
pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLuint = 37085;
pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: GLuint = 37086;
pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLuint = 37087;
pub const GL_SHADER_STORAGE_BARRIER_BIT: GLuint = 8192;
pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLuint = 36665;
pub const GL_DEPTH_STENCIL_TEXTURE_MODE: GLuint = 37098;
pub const GL_STENCIL_INDEX: GLuint = 6401;
pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLuint = 36446;
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLuint = 36447;
pub const GL_SAMPLE_POSITION: GLuint = 36432;
pub const GL_SAMPLE_MASK: GLuint = 36433;
pub const GL_SAMPLE_MASK_VALUE: GLuint = 36434;
pub const GL_TEXTURE_2D_MULTISAMPLE: GLuint = 37120;
pub const GL_MAX_SAMPLE_MASK_WORDS: GLuint = 36441;
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: GLuint = 37134;
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: GLuint = 37135;
pub const GL_MAX_INTEGER_SAMPLES: GLuint = 37136;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: GLuint = 37124;
pub const GL_TEXTURE_SAMPLES: GLuint = 37126;
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GLuint = 37127;
pub const GL_TEXTURE_WIDTH: GLuint = 4096;
pub const GL_TEXTURE_HEIGHT: GLuint = 4097;
pub const GL_TEXTURE_DEPTH: GLuint = 32881;
pub const GL_TEXTURE_INTERNAL_FORMAT: GLuint = 4099;
pub const GL_TEXTURE_RED_SIZE: GLuint = 32860;
pub const GL_TEXTURE_GREEN_SIZE: GLuint = 32861;
pub const GL_TEXTURE_BLUE_SIZE: GLuint = 32862;
pub const GL_TEXTURE_ALPHA_SIZE: GLuint = 32863;
pub const GL_TEXTURE_DEPTH_SIZE: GLuint = 34890;
pub const GL_TEXTURE_STENCIL_SIZE: GLuint = 35057;
pub const GL_TEXTURE_SHARED_SIZE: GLuint = 35903;
pub const GL_TEXTURE_RED_TYPE: GLuint = 35856;
pub const GL_TEXTURE_GREEN_TYPE: GLuint = 35857;
pub const GL_TEXTURE_BLUE_TYPE: GLuint = 35858;
pub const GL_TEXTURE_ALPHA_TYPE: GLuint = 35859;
pub const GL_TEXTURE_DEPTH_TYPE: GLuint = 35862;
pub const GL_TEXTURE_COMPRESSED: GLuint = 34465;
pub const GL_SAMPLER_2D_MULTISAMPLE: GLuint = 37128;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: GLuint = 37129;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLuint = 37130;
pub const GL_VERTEX_ATTRIB_BINDING: GLuint = 33492;
pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: GLuint = 33493;
pub const GL_VERTEX_BINDING_DIVISOR: GLuint = 33494;
pub const GL_VERTEX_BINDING_OFFSET: GLuint = 33495;
pub const GL_VERTEX_BINDING_STRIDE: GLuint = 33496;
pub const GL_VERTEX_BINDING_BUFFER: GLuint = 36687;
pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLuint = 33497;
pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: GLuint = 33498;
pub const GL_MAX_VERTEX_ATTRIB_STRIDE: GLuint = 33509;