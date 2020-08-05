use com::WeakPtr;
#[cfg(any(feature = "libloading", feature = "implicit-link"))]
use winapi::Interface as _;
use winapi::um::d3d12sdklayers;

pub type Debug = WeakPtr<d3d12sdklayers::ID3D12Debug>;

#[cfg(feature = "libloading")]
impl crate::D3D12Lib {
    pub fn get_debug_interface(&self) -> libloading::Result<crate::D3DResult<Debug>> {
        type Fun = extern "system" fn(
            winapi::shared::guiddef::REFIID,
            *mut *mut winapi::ctypes::c_void,
        ) -> crate::HRESULT;

        let mut debug = Debug::null();
        let hr = unsafe {
            let func: libloading::Symbol<Fun> = self.lib.get(b"D3D12GetDebugInterface")?;
            func(
                &d3d12sdklayers::ID3D12Debug::uuidof(),
                debug.mut_void(),
            )
        };

        Ok((debug, hr))
     }
}

impl Debug {
    #[cfg(feature = "implicit-link")]
    pub fn get_interface() -> crate::D3DResult<Self> {
        let mut debug = Debug::null();
        let hr = unsafe {
            winapi::um::d3d12::D3D12GetDebugInterface(
                &d3d12sdklayers::ID3D12Debug::uuidof(),
                debug.mut_void(),
            )
        };

        (debug, hr)
    }

    pub fn enable_layer(&self) {
        unsafe { self.EnableDebugLayer() }
    }
}
