Rust library for loading `atidxx{64, 32}.dll` and compiling shaders with it.

Shamelessly steals the header and DLL from RenderDoc:

- `assets/AmdDxGsaCompile.h` from [https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/amd_isa_win32.cpp](https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/amd_isa_win32.cpp)
- `assets/atidxx64.dll` from my RenderDoc v1.19 install
- `assets/devices.h` from [https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/official/RGA/Common/AsicReg/devices.h](https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/official/RGA/Common/AsicReg/devices.h)