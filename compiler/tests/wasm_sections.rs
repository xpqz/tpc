mod common;
use common::*;
use tp5c::wasm::*;

fn void_ty(m: &mut Module) -> u32 {
    m.ty(&[], &[])
}

#[test]
fn empty_module_is_the_header() {
    assert_eq!(Module::new().finish(), HEADER);
}

#[test]
fn type_section_bytes() {
    let mut m = Module::new();
    assert_eq!(m.ty(&[I32, I32], &[I32]), 0);
    assert_eq!(m.ty(&[], &[]), 1);
    let want = concat(&[
        &HEADER,
        &section(1, &[2, 0x60, 2, 0x7F, 0x7F, 1, 0x7F, 0x60, 0, 0]),
    ]);
    assert_eq!(m.finish(), want);
}

#[test]
fn type_section_encodes_every_value_type() {
    let mut m = Module::new();
    m.ty(&[I64, F64], &[I64]);
    let want = concat(&[&HEADER, &section(1, &[1, 0x60, 2, 0x7E, 0x7C, 1, 0x7E])]);
    assert_eq!(m.finish(), want);
}

#[test]
fn identical_signatures_share_one_type_index() {
    let mut m = Module::new();
    let a = m.ty(&[I32], &[I32]);
    let b = m.ty(&[I32, I32], &[I32]);
    let c = m.ty(&[I32], &[I32]);
    let d = m.ty(&[I32], &[]);
    let e = m.ty(&[], &[I32]);
    assert_eq!((a, b, c, d, e), (0, 1, 0, 2, 3));
    assert_eq!(m.ty(&[I32, I32], &[I32]), 1);
    // Params and results are distinct fields: [i32] -> [] differs from [] -> [i32].
    assert_ne!(d, e);
    let bytes = m.finish();
    assert_eq!(bytes[8], 1);
    assert_eq!(bytes[10], 4, "four distinct types");
}

#[test]
fn import_section_bytes() {
    let mut m = Module::new();
    let t = m.ty(&[I32, I32], &[]);
    assert_eq!(m.import_func("host", "write", t), 0);
    let want = concat(&[
        &HEADER,
        &section(1, &[1, 0x60, 2, 0x7F, 0x7F, 0]),
        &section(
            2,
            &[
                1, 4, b'h', b'o', b's', b't', 5, b'w', b'r', b'i', b't', b'e', 0, 0,
            ],
        ),
    ]);
    assert_eq!(m.finish(), want);
}

#[test]
fn function_and_code_sections_bytes() {
    let mut m = Module::new();
    let t = void_ty(&mut m);
    assert_eq!(m.func(t, &[], Code::new()), 0);
    let want = concat(&[
        &HEADER,
        &section(1, &[1, 0x60, 0, 0]),
        &section(3, &[1, 0]),
        &section(10, &[1, 2, 0, 0x0B]),
    ]);
    assert_eq!(m.finish(), want);
}

#[test]
fn two_functions_keep_order_and_types() {
    let mut m = Module::new();
    let t0 = void_ty(&mut m);
    let t1 = m.ty(&[I32], &[]);
    assert_eq!(m.func(t1, &[], Code::new()), 0);
    assert_eq!(m.func(t0, &[], Code::new()), 1);
    let want = concat(&[
        &HEADER,
        &section(1, &[2, 0x60, 0, 0, 0x60, 1, 0x7F, 0]),
        &section(3, &[2, 1, 0]),
        &section(10, &[2, 2, 0, 0x0B, 2, 0, 0x0B]),
    ]);
    assert_eq!(m.finish(), want);
}

#[test]
fn locals_are_run_length_encoded() {
    let mut m = Module::new();
    let t = void_ty(&mut m);
    let mut c = Code::new();
    c.op(NOP);
    m.func(t, &[I32, I32, F64], c);
    let bytes = m.finish();
    let want_code = section(10, &[1, 7, 2, 2, 0x7F, 1, 0x7C, NOP, 0x0B]);
    assert!(bytes.ends_with(&want_code), "{bytes:02x?}");
}

#[test]
fn only_adjacent_equal_locals_are_merged() {
    let mut m = Module::new();
    let t = void_ty(&mut m);
    m.func(t, &[I32, F64, I32], Code::new());
    let want_code = section(10, &[1, 8, 3, 1, 0x7F, 1, 0x7C, 1, 0x7F, 0x0B]);
    assert!(m.finish().ends_with(&want_code));
}

#[test]
fn table_section_bytes() {
    let mut m = Module::new();
    m.table(3);
    assert_eq!(
        m.finish(),
        concat(&[&HEADER, &section(4, &[1, 0x70, 0, 3])])
    );
}

#[test]
fn memory_section_bytes() {
    let mut m = Module::new();
    m.memory(16);
    assert_eq!(m.finish(), concat(&[&HEADER, &section(5, &[1, 0, 16])]));
}

#[test]
fn memory_page_count_is_leb_encoded() {
    let mut m = Module::new();
    m.memory(300);
    assert_eq!(
        m.finish(),
        concat(&[&HEADER, &section(5, &[1, 0, 0xAC, 0x02])])
    );
}

#[test]
fn global_section_bytes() {
    let mut m = Module::new();
    assert_eq!(m.global_i32(true, 65536), 0);
    assert_eq!(m.global_i32(false, -1), 1);
    let want = concat(&[
        &HEADER,
        &section(
            6,
            &[
                2, 0x7F, 1, 0x41, 0x80, 0x80, 0x04, 0x0B, 0x7F, 0, 0x41, 0x7F, 0x0B,
            ],
        ),
    ]);
    assert_eq!(m.finish(), want);
}

#[test]
fn export_section_bytes() {
    let mut m = Module::new();
    m.memory(1);
    m.global_i32(true, 0);
    m.export("mem", MEM, 0);
    m.export("sp", GLOBAL, 0);
    let bytes = m.finish();
    let want_export = section(7, &[2, 3, b'm', b'e', b'm', 2, 0, 2, b's', b'p', 3, 0]);
    assert!(bytes.ends_with(&want_export), "{bytes:02x?}");
}

#[test]
fn function_and_table_exports_use_their_kind_bytes() {
    let mut m = Module::new();
    let t = void_ty(&mut m);
    m.func(t, &[], Code::new());
    m.table(1);
    m.export("f", FUNC, 0);
    m.export("t", TABLE, 0);
    let want_export = section(7, &[2, 1, b'f', 0, 0, 1, b't', 1, 0]);
    let bytes = m.finish();
    let at = bytes
        .windows(2)
        .position(|w| w == [7, 9])
        .expect("export section");
    assert_eq!(bytes[at..at + 11], want_export[..]);
}

#[test]
fn element_section_bytes() {
    let mut m = Module::new();
    let t = void_ty(&mut m);
    m.func(t, &[], Code::new());
    m.table(2);
    m.elem(1, &[0]);
    let want = concat(&[
        &HEADER,
        &section(1, &[1, 0x60, 0, 0]),
        &section(3, &[1, 0]),
        &section(4, &[1, 0x70, 0, 2]),
        &section(9, &[1, 0, 0x41, 1, 0x0B, 1, 0]),
        &section(10, &[1, 2, 0, 0x0B]),
    ]);
    assert_eq!(m.finish(), want);
}

#[test]
fn element_segments_list_several_functions() {
    let mut m = Module::new();
    let t = void_ty(&mut m);
    m.func(t, &[], Code::new());
    m.func(t, &[], Code::new());
    m.table(3);
    m.elem(1, &[1, 0]);
    let bytes = m.finish();
    assert!(
        bytes
            .windows(9)
            .any(|w| w == [9, 8, 1, 0, 0x41, 1, 0x0B, 2, 1]),
        "{bytes:02x?}"
    );
}

#[test]
fn data_section_bytes() {
    let mut m = Module::new();
    m.memory(1);
    m.data(256, b"hi");
    let want = concat(&[
        &HEADER,
        &section(5, &[1, 0, 1]),
        &section(11, &[1, 0, 0x41, 0x80, 0x02, 0x0B, 2, b'h', b'i']),
    ]);
    assert_eq!(m.finish(), want);
}

#[test]
fn several_data_segments_in_insertion_order() {
    let mut m = Module::new();
    m.memory(1);
    m.data(256, b"a");
    m.data(512, b"");
    let bytes = m.finish();
    let want = section(
        11,
        &[
            2, 0, 0x41, 0x80, 0x02, 0x0B, 1, b'a', 0, 0x41, 0x80, 0x04, 0x0B, 0,
        ],
    );
    assert!(bytes.ends_with(&want), "{bytes:02x?}");
}

#[test]
fn large_section_sizes_use_multibyte_leb() {
    let mut m = Module::new();
    m.memory(1);
    m.data(256, &[7u8; 200]);
    let bytes = m.finish();
    // data section: id 11, size = 1 (count) + 1 (flag) + 4 (offset) + 2 (len 200) + 200 = 208
    let at = 8 + 5;
    assert_eq!(bytes[at..at + 3], [11, 0xD0, 0x01]);
    assert_eq!(bytes.len(), at + 3 + 208);
}

#[test]
fn sections_come_out_in_canonical_order_regardless_of_call_order() {
    let mut m = Module::new();
    m.data(256, b"x");
    m.export("m", MEM, 0);
    m.global_i32(true, 0);
    m.memory(1);
    m.table(1);
    let t = void_ty(&mut m);
    m.func(t, &[], Code::new());
    let bytes = m.finish();
    let mut ids = Vec::new();
    let mut at = 8;
    while at < bytes.len() {
        ids.push(bytes[at]);
        let (len, n) = decode_leb(&bytes[at + 1..], false);
        at += 1 + n + len as usize;
    }
    assert_eq!(at, bytes.len());
    assert_eq!(ids, [1, 3, 4, 5, 6, 7, 10, 11]);
}

#[test]
fn unused_sections_are_omitted() {
    let mut m = Module::new();
    m.memory(1);
    assert_eq!(m.finish().len(), 8 + 5);
}

#[test]
fn imports_precede_defined_functions_in_the_index_space() {
    let mut m = Module::new();
    let t = void_ty(&mut m);
    assert_eq!(m.import_func("host", "a", t), 0);
    assert_eq!(m.import_func("host", "b", t), 1);
    assert_eq!(m.func(t, &[], Code::new()), 2);
    assert_eq!(m.func(t, &[], Code::new()), 3);
}

#[test]
#[should_panic(expected = "import after function")]
fn import_after_a_defined_function_is_rejected() {
    let mut m = Module::new();
    let t = void_ty(&mut m);
    m.func(t, &[], Code::new());
    m.import_func("host", "late", t);
}
