//! Cache the pipeline objects to disk for faster reloads.
//! 
//! A pipeline cache is an opaque type that allow you to cache your graphics and compute
//! pipelines on the disk.
//! 
//! You can create either an empty cache or a cache from some initial data. Whenever you create a
//! graphics or compute pipeline, you have the possibility to pass a reference to that cache.
//! The Vulkan implementation will then look in the cache for an existing entry, or add one if it
//! doesn't exist.
//! 
//! Once that is done, you can extract the data from the cache and store it.
//!
use std::mem;
use std::ptr;
use std::sync::Arc;

use device::Device;

use OomError;
use VulkanObject;
use VulkanPointers;
use check_errors;
use vk;

/// Opaque cache that contains pipeline objects.
pub struct PipelineCache {
    device: Arc<Device>,
    cache: vk::PipelineCache,
}

impl PipelineCache {
    /// Builds a new pipeline cache.
    ///
    /// You can pass optional data to initialize the cache with. If you don't pass any data, the
    /// cache will be empty.
    // TODO: is that unsafe? is it safe to pass garbage data?
    pub unsafe fn new(device: &Arc<Device>, initial_data: Option<&[u8]>)
                      -> Result<Arc<PipelineCache>, OomError>
    {
        let vk = device.pointers();

        let cache = unsafe {
            let infos = vk::PipelineCacheCreateInfo {
                sType: vk::STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,   // reserved
                initialDataSize: initial_data.map(|d| d.len()).unwrap_or(0),
                pInitialData: initial_data.map(|d| d.as_ptr() as *const _).unwrap_or(ptr::null()),
            };

            let mut output = mem::uninitialized();
            try!(check_errors(vk.CreatePipelineCache(device.internal_object(), &infos,
                                                     ptr::null(), &mut output)));
            output
        };

        Ok(Arc::new(PipelineCache {
            device: device.clone(),
            cache: cache,
        }))
    }

    /// Merges other pipeline caches into this one.
    ///
    /// # Panic
    ///
    /// - Panicks if `self` is included in the list of other pipelines.
    ///
    pub fn merge<'a, I>(&self, pipelines: I) -> Result<(), OomError>
        where I: IntoIterator<Item = &'a &'a Arc<PipelineCache>>
    {
        unsafe {
            let vk = self.device.pointers();

            let pipelines = pipelines.into_iter().map(|pipeline| {
                assert!(&***pipeline as *const _ != &*self as *const _);
                pipeline.cache
            }).collect::<Vec<_>>();

            try!(check_errors(vk.MergePipelineCaches(self.device.internal_object(), self.cache,
                                                     pipelines.len() as u32, pipelines.as_ptr())));

            Ok(())
        }
    }

    /// Obtains the data from the cache.
    ///
    /// This data can be stored and then reloaded and passed to `PipelineCache::new`.
    pub fn get_data(&self) -> Result<Vec<u8>, OomError> {
        unsafe {
            let vk = self.device.pointers();

            let mut num = 0;
            try!(check_errors(vk.GetPipelineCacheData(self.device.internal_object(), self.cache,
                                                      &mut num, ptr::null_mut())));

            let mut data: Vec<u8> = Vec::with_capacity(num as usize);
            try!(check_errors(vk.GetPipelineCacheData(self.device.internal_object(), self.cache,
                                                      &mut num, data.as_mut_ptr() as *mut _)));
            data.set_len(num as usize);

            Ok(data)
        }
    }
}

unsafe impl VulkanObject for PipelineCache {
    type Object = vk::PipelineCache;

    #[inline]
    fn internal_object(&self) -> vk::PipelineCache {
        self.cache
    }
}

impl Drop for PipelineCache {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let vk = self.device.pointers();
            vk.DestroyPipelineCache(self.device.internal_object(), self.cache, ptr::null());
        }
    }
}

#[cfg(test)]
mod tests {
    use instance;

    #[test]
    //#[should_panic]
    fn merge_self() {
        let instance = instance!();

        // let pipeline = PipelineCache::new(&device).unwrap();
        // pipeline.merge(&[&pipeline]).unwrap();
    }
}
