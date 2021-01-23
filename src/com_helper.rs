pub const fn translate_guid(guid: winapi::shared::guiddef::GUID) -> com::sys::GUID {
    com::sys::GUID {
        data1: guid.Data1,
        data2: guid.Data2,
        data3: guid.Data3,
        data4: guid.Data4,
    }
}

#[repr(transparent)]
pub struct ComPtr<T: winapi::Interface> {
    inner: std::ptr::NonNull<T>,
}

unsafe impl<T: winapi::Interface> com::Interface for ComPtr<T> {
    type VTable = <T as winapi::Interface>::VTable;
    type Super = ComPtr<<T as winapi::Interface>::Super>;
    const IID: com::sys::IID = translate_guid(<T as winapi::Interface>::IID);
}

impl<T: winapi::Interface> std::ops::Deref for ComPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.as_ptr() }
    }
}

impl<T: winapi::Interface> Drop for ComPtr<T> {
    fn drop(&mut self) {
        unsafe {
            <Self as ::com::Interface>::as_iunknown(self).Release();
        }
    }
}

impl<T: winapi::Interface> ::std::clone::Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        unsafe {
            <Self as ::com::Interface>::as_iunknown(self).AddRef();
        }
        Self { inner: self.inner }
    }
}
