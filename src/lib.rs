mod com_helper;

use crate::com_helper::ComPtr;
use com::sys::{E_NOINTERFACE, HRESULT};
use winapi::um::strmif::IFilterMapper2;

macro_rules! DEFINE_GUID {
    (
        $name:ident, $l:expr, $w1:expr, $w2:expr,
        $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr
    ) => {
        pub const $name: com::sys::GUID = com::sys::GUID {
            data1: $l,
            data2: $w1,
            data3: $w2,
            data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
        };
    };
}

// a2c3c6ea-a415-4809-a55c-b49934d81ab7
DEFINE_GUID!(CLSID_MY_CLASS, 0xa2c3c6ea, 0xa415, 0x4809, 0xa5, 0x5c, 0xb4, 0x99, 0x34, 0xd8, 0x1a, 0xb7);

com::class! {
    pub class MyClass: ComPtr<IFilterMapper2> {
        a: i32,
    }

    impl ComPtr<IFilterMapper2> for MyClass {
        fn EnumMatchingFilters(&self) -> HRESULT { E_NOINTERFACE }
    }
}

com::inproc_dll_module![(CLSID_MY_CLASS, MyClass),];

#[cfg(test)]
mod tests {
    use crate::com_helper::{translate_guid, ComPtr};
    use crate::CLSID_MY_CLASS;
    use com::runtime::{create_instance, init_runtime};
    use winapi::shared::uuids::CLSID_FilterMapper2;
    use winapi::um::strmif::IFilterMapper2;

    #[test]
    fn test_filter_mapper() {
        unsafe {
            init_runtime().expect("Failed to initialize COM Library");

            let mapper =
                create_instance::<ComPtr<IFilterMapper2>>(&translate_guid(CLSID_FilterMapper2))
                    .expect("FilterMapper2");
            mapper.AddRef();
            mapper.Release();
        }
    }

    #[test]
    fn test_my_class() {
        unsafe {
            init_runtime().expect("Failed to initialize COM Library");

            let mapper =
                create_instance::<ComPtr<IFilterMapper2>>(&CLSID_MY_CLASS).expect("MyClass");
            mapper.AddRef();
            mapper.Release();
        }
    }
}
