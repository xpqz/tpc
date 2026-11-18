mod common;
use common::decode_leb;
use tp5c::wasm::{sleb, uleb};

fn u(v: u64) -> Vec<u8> {
    let mut o = Vec::new();
    uleb(&mut o, v);
    o
}

fn s(v: i64) -> Vec<u8> {
    let mut o = Vec::new();
    sleb(&mut o, v);
    o
}

#[test]
fn unsigned_known_sequences() {
    let cases: &[(u64, &[u8])] = &[
        (0, &[0x00]),
        (1, &[0x01]),
        (63, &[0x3F]),
        (64, &[0x40]),
        (127, &[0x7F]),
        (128, &[0x80, 0x01]),
        (255, &[0xFF, 0x01]),
        (300, &[0xAC, 0x02]),
        (624485, &[0xE5, 0x8E, 0x26]),
        (u32::MAX as u64, &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]),
        (
            u64::MAX,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
        ),
    ];
    for (v, bytes) in cases {
        assert_eq!(u(*v), *bytes, "uleb({v})");
    }
}

#[test]
fn signed_known_sequences() {
    let cases: &[(i64, &[u8])] = &[
        (0, &[0x00]),
        (-1, &[0x7F]),
        (63, &[0x3F]),
        (64, &[0xC0, 0x00]),
        (-64, &[0x40]),
        (-65, &[0xBF, 0x7F]),
        (127, &[0xFF, 0x00]),
        (-128, &[0x80, 0x7F]),
        (-123456, &[0xC0, 0xBB, 0x78]),
        (i32::MIN as i64, &[0x80, 0x80, 0x80, 0x80, 0x78]),
        (i32::MAX as i64, &[0xFF, 0xFF, 0xFF, 0xFF, 0x07]),
        (
            i64::MIN,
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7F],
        ),
        (
            i64::MAX,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00],
        ),
    ];
    for (v, bytes) in cases {
        assert_eq!(s(*v), *bytes, "sleb({v})");
    }
}

#[test]
fn encoders_append_to_existing_output() {
    let mut o = vec![0xAA];
    uleb(&mut o, 128);
    sleb(&mut o, -1);
    assert_eq!(o, [0xAA, 0x80, 0x01, 0x7F]);
}

#[test]
fn round_trip_around_every_group_boundary() {
    for shift in 0..64 {
        for d in -2i64..=2 {
            let v = (1i64 << shift).wrapping_add(d);
            let e = s(v);
            assert_eq!(decode_leb(&e, true), (v as i128, e.len()), "sleb {v}");
            let e = s(v.wrapping_neg());
            assert_eq!(
                decode_leb(&e, true),
                (v.wrapping_neg() as i128, e.len()),
                "sleb -{v}"
            );
            let uv = v as u64;
            let e = u(uv);
            assert_eq!(decode_leb(&e, false), (uv as i128, e.len()), "uleb {uv}");
        }
    }
}

#[test]
fn encodings_are_minimal() {
    assert_eq!(u(127).len(), 1);
    assert_eq!(u(128).len(), 2);
    assert_eq!(s(63).len(), 1);
    assert_eq!(s(-64).len(), 1);
    assert_eq!(s(64).len(), 2);
    assert_eq!(s(-65).len(), 2);
}
