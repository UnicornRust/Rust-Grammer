pub mod varray;
pub mod vbool;
pub mod vchar;
pub mod vcommon;
pub mod vconst;
pub mod venum;
pub mod vhashmap;
pub mod vnumber;
pub mod vslice;
pub mod vstring;
pub mod vstruct;
pub mod vtuple;
pub mod vvec;


pub fn run() {
    vcommon::variable();
    vstring::string_variable();
    vnumber::number_variable();
    vbool::bool_variable();
    vchar::char_variable();
    vtuple::tuple_type();
    vslice::slice_reference();
    vstruct::struct_variable();
    venum::test_enum();
    vvec::vec_type();
    vhashmap::hashmap_type();
    varray::run();
    vslice::slice_reference();
}

