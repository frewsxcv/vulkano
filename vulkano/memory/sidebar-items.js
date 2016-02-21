initSidebarItems({"enum":[["ChunkProperties",""],["ChunkRange",""]],"struct":[["DeviceLocal","Dummy marker whose strategy is to allocate a new chunk of memory for each allocation.The memory will not be accessible since it is not necessarily in host-visible memory.This is good for large buffers, but inefficient is you use a lot of small buffers.The memory is locked globally. That means that it doesn't matter whether you access the buffer for reading or writing (like a `Mutex`)."],["DeviceLocalChunk","A chunk allocated from a `DeviceLocal`."],["DeviceMemory","Represents memory that has been allocated."],["HostVisible","Dummy marker whose strategy is to allocate a new chunk of memory for each allocation.Guaranteed to allocate from a host-visible memory type.This is good for large buffers, but inefficient is you use a lot of small buffers.The memory is locked globally. That means that it doesn't matter whether you access the buffer for reading or writing (like a `Mutex`)."],["HostVisibleChunk","A chunk allocated from a `HostVisible`."],["MappedDeviceMemory","Represents memory that has been allocated and mapped in CPU accessible space."]],"trait":[["Content","Trait for types of data that can be mapped."],["CpuAccessible","Trait for memory objects that can be accessed from the CPU."],["CpuWriteAccessible","Trait for memory objects that be mutably accessed from the CPU."],["MemorySource","Trait for objects that can be used to fill the memory requirements of a buffer or an image."],["MemorySourceChunk","A chunk of GPU-visible memory."]]});