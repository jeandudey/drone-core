#![feature(allocator_api)]
#![feature(slice_ptr_get)]

use crate::core::mem::size_of;
use std as core;

use drone_core::heap;

drone_core::config_override! { "
[memory.flash]
size = \"128K\"
origin = 0x08000000

[memory.ram]
size = \"20K\"
origin = 0x20000000

[heap.main]
size = \"10K\"
pools = [
    { block = \"4\", capacity = 896 },
    { block = \"32\", capacity = 80 },
    { block = \"256\", capacity = 16 },
]

[heap.secondary]
origin = 0x40000000
size = \"6K\"
pools = [
    { block = \"4\", capacity = 896 },
    { block = \"32\", capacity = 80 },
]

[linker]
platform = \"arm\"
" }

heap! {
    config => main;
    /// Test doc attribute
    #[doc = "test attribute"]
    metadata => pub HeapMain;
    global => false;
}

heap! {
    config => secondary;
    metadata => pub HeapSecondary;
    global => false;
}

#[test]
fn size() {
    assert_eq!(size_of::<HeapMain>(), size_of::<heap::Pool>() * 3);
    assert_eq!(size_of::<HeapSecondary>(), size_of::<heap::Pool>() * 2);
}
