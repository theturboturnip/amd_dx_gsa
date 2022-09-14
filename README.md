Rust library for loading `atidxx{64, 32}.dll` and compiling shaders with it.

Shamelessly steals the header and DLL from RenderDoc:

- `assets/AmdDxGsaCompile.h` from [https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/amd_isa_win32.cpp](https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/amd_isa_win32.cpp)
- `assets/atidxx64.dll` from my RenderDoc v1.19 install
- `assets/devices.h` from [https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/official/RGA/Common/AsicReg/devices.h](https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/official/RGA/Common/AsicReg/devices.h)

NOTE: To use this library you need `atidxx64.dll`.
The examples assume you have a copy in `assets/atidxx64.dll`.
I am not allowed to redistribute `atidxx64.dll`.
RenderDoc provides instructions for this [in its wiki](https://github.com/baldurk/renderdoc/wiki/GCN-ISA#d3d11-and-d3d12-disassembly-with-amd-driver)


Shader is compiled from `assets/example.hlsl`:
- `fxc -T ps_4_0 example.hlsl /Fo example.fxc`
- `dxc -T ps_6_5 example.hlsl -Fo example.dxc`