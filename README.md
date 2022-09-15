# amd_dx_gsa
Rust library for loading `atidxx64.dll` and compiling shaders with it.

Designed for usage with [`turnip_gfx_disasm`](https://github.com/theturboturnip/turnip_gfx_disasm) and [`yk_fxo_disasm`](https://github.com/theturboturnip/yk_fxo_disasm).

## Assets
Shamelessly steals the header and DLL from RenderDoc:

- `assets/AmdDxGsaCompile.h` from [https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/official/RGA/Common/AmdDxGsaCompile.h](https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/official/RGA/Common/AmdDxGsaCompile.h)
- `assets/devices.h` from [https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/official/RGA/Common/AsicReg/devices.h](https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/official/RGA/Common/AsicReg/devices.h)
- `assets/atidxx64.dll` from my RenderDoc v1.19 install

Additionally, parsing code and the usage of the DLL is based on [RenderDoc's parsing code.](https://github.com/baldurk/renderdoc/blob/58437e426f5e5f564ffc2b6b0410a530b45f96cd/renderdoc/driver/ihv/amd/amd_isa_win32.cpp)

NOTE: To use this library you need `atidxx64.dll`.
The examples assume you have a copy in `assets/atidxx64.dll`.
I am not allowed to redistribute `atidxx64.dll`.
RenderDoc provides instructions for getting this DLL [in its wiki.](https://github.com/baldurk/renderdoc/wiki/GCN-ISA#d3d11-and-d3d12-disassembly-with-amd-driver)

Shader is compiled from `assets/example.hlsl`:
- `fxc -T ps_4_0 example.hlsl /Fo example.fxc`
- `dxc -T ps_6_5 example.hlsl -Fo example.dxc`