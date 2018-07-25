
use hal::{self, format as f, image};
use {Backend as B, Device, PhysicalDevice, QueueFamily, Starc};
use std::clone;
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum GLES_VERSION {
    ES20,
    ES30,
    ES31,
    ES32,
}

impl Clone for GLES_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy, Debug, Clone)]
pub struct Env {
    width: u32,
    height: u32,
    gl_version: GLES_VERSION,
    num_sampler: u32,
    out_fbo: u32,
    formats: Vec<f::Format>,
}

impl Clone for Env {
    //&non-owning pointer
    fn clone(&self) -> Env {
        *self
    }
}

// hal constraint Swapchain class
#[derive(Copy, Debug, Clone)]
pub struct Swapchain{
    pub env : Starc<Env>,
}

impl Clone for Swapchain {
    fn clone(&self)->Swapchain {
        *self
    }
}

impl Swapchain<B> for Swapchain {
    fn acquire_image(&mut self, sync: hal::FrameSync<B>) -> Result<hal::SwapImageIndex, ()> {
        Ok(0)
    }
}

//hal constraint Surface class
pub struct Surface {
    pub env: Starc<Env>,
}

impl Surface{
    fn from_Env(env: &Env)->Self {
        Surface{
            env: Starc::new(env.clone())
        }
    }

    fn get_Env(&self)->&Env {
        &self.env
    }

    fn swapchain_format(&self) -> Vec<f::Format> {
        let mut copyFormats: Vec<f::Format> = Vec::new();
        for format in &self.env.formats {
            let tempForm = *format;
            copyFormats.push(tempForm);
        }

        copyFormats
    }
}

impl hal::Surface<B> for Surface {
    fn kind(&self) -> image::Kind {
        let width:u32 = self.env.width;
        let height: u32 = self.env.height;
        let samplers : u32 = self.env.num_sampler;
        return hal::image::Kind::D2(width, height, 1, samplers as _);
    }

    fn supports_queue_family(&self, family: &B::QueueFamily) -> bool {
        true
    }

    fn compatibility(
        &self, physical_device: &B::PhysicalDevice
    ) -> (hal::SurfaceCapabilities, Option<Vec<Format>>, Vec<hal::PresentMode>) {
        let extentStart = hal::Extent2D{
            width: self.env.width,
            height: self.env.height,
        };

        let extentEnd = hal::Extent2D{
            width: self.env.width + 1,
            height: self.env.height + 1,
        };

        let present_modes = vec![hal::PresentMode::Fifo];

        let caps = hal::SurfaceCapabilities {
            image_count: {1..2},
            current_extent: extentStart,
            extents: extentStart .. extentEnd,
            max_image_layers: 1,
        };

        (caps, Some(self.swapchain_format()), present_modes)
    }
}

impl Device {
    pub(crate) fn create_swapchain_impl(&self,
                                        surface: &mut Surface,
                                        config: hal::SwapchainConfig)
                                        -> (B::Swapchain, hal::Backbuffer<B>){
        let backbuffer = hal::Backbuffer::Framebuffer(out_fbo);
        let swapchain :Swapchain = Swapchain {
            env:  Starc::new(env.clone()),
        };

        (swapchain, backbuffer)
    }
}

pub struct Instance {
    env: Env,
}

impl Instance {
    fn from_env(e: Env) -> Instance{
        Instance{
            env : e,
        }
    }

    fn create_surface(&self) -> Surface {
        Surface::from_Env(&self.env)
    }
}

impl hal::Instance for Instance {
    type Backend = B;
    fn enumerate_adapters(&self) -> Vec<hal::Adapter<B>> {
        let adapters = PhysicalDevice::new_adapter(&self.env.gl_version);
        vec![adapters]
    }
}
