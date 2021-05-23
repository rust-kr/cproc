#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[no_mangle]
extern "C" fn __qbe_emit_name(n: &name)  {

}

#[no_mangle]
extern "C" fn __qbe_emit_value(n: &value)  {

}

#[no_mangle]
extern "C" fn __qbe_emit_repr(n: &repr, v: &value, ext: bool)  {

}

#[no_mangle]
extern "C" fn __qbe_emit_type(t: &type_)  {

}

#[no_mangle]
extern "C" fn __qbe_emit_inst(t: &inst)  {

}

#[no_mangle]
extern "C" fn __qbe_emit_jump(t: &jump)  {

}

#[no_mangle]
extern "C" fn __qbe_emit_func(t: &func)  {

}

#[no_mangle]
extern "C" fn __qbe_emit_data(t: &decl, i: init)  {

}
