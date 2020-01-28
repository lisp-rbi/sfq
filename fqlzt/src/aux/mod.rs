#[macro_use]
extern crate cpp;
use cpp::cpp;


use crate::Lzt;


// This si binding that does not work now but i'll fix in on thurstay


cpp!{{

Lzt::Lzt() {
    this->internal =
        rust!(Lzt_constructor [] -> *mut Lzt as "void *" {
            let b = Box::new(Lzt::default());
            Box::into_raw(b)
        });
}

Lzt::~Lzt() {
    rust!(Lzt_destructor [internal: *mut Lzt as "void *"] {
        let _b = unsafe {
            Box::from_raw(internal)
        };
    });
}

void Lzt::test_binding(int64_t value) {
    rust!(Lzt_get_records [
        internal: &mut Lzt as "void *",
        value: i64 as "int64_t"
    ] {
        internal.test_binding(value);
    });
}

}}
