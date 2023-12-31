/// CPU flags representing the result of an integer comparison. These flags
/// can be tested with an :type:`intcc` condition code.
pub const IFLAGS: Type = Type(0x1);

/// CPU flags representing the result of a floating point comparison. These
/// flags can be tested with a :type:`floatcc` condition code.
pub const FFLAGS: Type = Type(0x2);

/// A boolean type with 1 bits.
pub const B1: Type = Type(0x70);

/// A boolean type with 8 bits.
pub const B8: Type = Type(0x71);

/// A boolean type with 16 bits.
pub const B16: Type = Type(0x72);

/// A boolean type with 32 bits.
pub const B32: Type = Type(0x73);

/// A boolean type with 64 bits.
pub const B64: Type = Type(0x74);

/// A boolean type with 128 bits.
pub const B128: Type = Type(0x75);

/// An integer type with 8 bits.
/// WARNING: arithmetic on 8bit integers is incomplete
pub const I8: Type = Type(0x76);

/// An integer type with 16 bits.
/// WARNING: arithmetic on 16bit integers is incomplete
pub const I16: Type = Type(0x77);

/// An integer type with 32 bits.
pub const I32: Type = Type(0x78);

/// An integer type with 64 bits.
pub const I64: Type = Type(0x79);

/// An integer type with 128 bits.
pub const I128: Type = Type(0x7a);

/// A 32-bit floating point type represented in the IEEE 754-2008
/// *binary32* interchange format. This corresponds to the :c:type:`float`
/// type in most C implementations.
pub const F32: Type = Type(0x7b);

/// A 64-bit floating point type represented in the IEEE 754-2008
/// *binary64* interchange format. This corresponds to the :c:type:`double`
/// type in most C implementations.
pub const F64: Type = Type(0x7c);

/// An opaque reference type with 32 bits.
pub const R32: Type = Type(0x7e);

/// An opaque reference type with 64 bits.
pub const R64: Type = Type(0x7f);

/// A SIMD vector with 8 lanes containing a `b8` each.
pub const B8X8: Type = Type(0xa1);

/// A SIMD vector with 4 lanes containing a `b16` each.
pub const B16X4: Type = Type(0x92);

/// A SIMD vector with 2 lanes containing a `b32` each.
pub const B32X2: Type = Type(0x83);

/// A SIMD vector with 8 lanes containing a `i8` each.
pub const I8X8: Type = Type(0xa6);

/// A SIMD vector with 4 lanes containing a `i16` each.
pub const I16X4: Type = Type(0x97);

/// A SIMD vector with 2 lanes containing a `i32` each.
pub const I32X2: Type = Type(0x88);

/// A SIMD vector with 2 lanes containing a `f32` each.
pub const F32X2: Type = Type(0x8b);

/// A dynamically-scaled SIMD vector with a minimum of 8 lanes containing `b8` bits each.
pub const B8X8XN: Type = Type(0x121);

/// A dynamically-scaled SIMD vector with a minimum of 4 lanes containing `b16` bits each.
pub const B16X4XN: Type = Type(0x112);

/// A dynamically-scaled SIMD vector with a minimum of 2 lanes containing `b32` bits each.
pub const B32X2XN: Type = Type(0x103);

/// A dynamically-scaled SIMD vector with a minimum of 8 lanes containing `i8` bits each.
pub const I8X8XN: Type = Type(0x126);

/// A dynamically-scaled SIMD vector with a minimum of 4 lanes containing `i16` bits each.
pub const I16X4XN: Type = Type(0x117);

/// A dynamically-scaled SIMD vector with a minimum of 2 lanes containing `i32` bits each.
pub const I32X2XN: Type = Type(0x108);

/// A dynamically-scaled SIMD vector with a minimum of 2 lanes containing `f32` bits each.
pub const F32X2XN: Type = Type(0x10b);

/// A SIMD vector with 16 lanes containing a `b8` each.
pub const B8X16: Type = Type(0xb1);

/// A SIMD vector with 8 lanes containing a `b16` each.
pub const B16X8: Type = Type(0xa2);

/// A SIMD vector with 4 lanes containing a `b32` each.
pub const B32X4: Type = Type(0x93);

/// A SIMD vector with 2 lanes containing a `b64` each.
pub const B64X2: Type = Type(0x84);

/// A SIMD vector with 16 lanes containing a `i8` each.
pub const I8X16: Type = Type(0xb6);

/// A SIMD vector with 8 lanes containing a `i16` each.
pub const I16X8: Type = Type(0xa7);

/// A SIMD vector with 4 lanes containing a `i32` each.
pub const I32X4: Type = Type(0x98);

/// A SIMD vector with 2 lanes containing a `i64` each.
pub const I64X2: Type = Type(0x89);

/// A SIMD vector with 4 lanes containing a `f32` each.
pub const F32X4: Type = Type(0x9b);

/// A SIMD vector with 2 lanes containing a `f64` each.
pub const F64X2: Type = Type(0x8c);

/// A dynamically-scaled SIMD vector with a minimum of 16 lanes containing `b8` bits each.
pub const B8X16XN: Type = Type(0x131);

/// A dynamically-scaled SIMD vector with a minimum of 8 lanes containing `b16` bits each.
pub const B16X8XN: Type = Type(0x122);

/// A dynamically-scaled SIMD vector with a minimum of 4 lanes containing `b32` bits each.
pub const B32X4XN: Type = Type(0x113);

/// A dynamically-scaled SIMD vector with a minimum of 2 lanes containing `b64` bits each.
pub const B64X2XN: Type = Type(0x104);

/// A dynamically-scaled SIMD vector with a minimum of 16 lanes containing `i8` bits each.
pub const I8X16XN: Type = Type(0x136);

/// A dynamically-scaled SIMD vector with a minimum of 8 lanes containing `i16` bits each.
pub const I16X8XN: Type = Type(0x127);

/// A dynamically-scaled SIMD vector with a minimum of 4 lanes containing `i32` bits each.
pub const I32X4XN: Type = Type(0x118);

/// A dynamically-scaled SIMD vector with a minimum of 2 lanes containing `i64` bits each.
pub const I64X2XN: Type = Type(0x109);

/// A dynamically-scaled SIMD vector with a minimum of 4 lanes containing `f32` bits each.
pub const F32X4XN: Type = Type(0x11b);

/// A dynamically-scaled SIMD vector with a minimum of 2 lanes containing `f64` bits each.
pub const F64X2XN: Type = Type(0x10c);

/// A SIMD vector with 32 lanes containing a `b8` each.
pub const B8X32: Type = Type(0xc1);

/// A SIMD vector with 16 lanes containing a `b16` each.
pub const B16X16: Type = Type(0xb2);

/// A SIMD vector with 8 lanes containing a `b32` each.
pub const B32X8: Type = Type(0xa3);

/// A SIMD vector with 4 lanes containing a `b64` each.
pub const B64X4: Type = Type(0x94);

/// A SIMD vector with 2 lanes containing a `b128` each.
pub const B128X2: Type = Type(0x85);

/// A SIMD vector with 32 lanes containing a `i8` each.
pub const I8X32: Type = Type(0xc6);

/// A SIMD vector with 16 lanes containing a `i16` each.
pub const I16X16: Type = Type(0xb7);

/// A SIMD vector with 8 lanes containing a `i32` each.
pub const I32X8: Type = Type(0xa8);

/// A SIMD vector with 4 lanes containing a `i64` each.
pub const I64X4: Type = Type(0x99);

/// A SIMD vector with 2 lanes containing a `i128` each.
pub const I128X2: Type = Type(0x8a);

/// A SIMD vector with 8 lanes containing a `f32` each.
pub const F32X8: Type = Type(0xab);

/// A SIMD vector with 4 lanes containing a `f64` each.
pub const F64X4: Type = Type(0x9c);

/// A dynamically-scaled SIMD vector with a minimum of 32 lanes containing `b8` bits each.
pub const B8X32XN: Type = Type(0x141);

/// A dynamically-scaled SIMD vector with a minimum of 16 lanes containing `b16` bits each.
pub const B16X16XN: Type = Type(0x132);

/// A dynamically-scaled SIMD vector with a minimum of 8 lanes containing `b32` bits each.
pub const B32X8XN: Type = Type(0x123);

/// A dynamically-scaled SIMD vector with a minimum of 4 lanes containing `b64` bits each.
pub const B64X4XN: Type = Type(0x114);

/// A dynamically-scaled SIMD vector with a minimum of 2 lanes containing `b128` bits each.
pub const B128X2XN: Type = Type(0x105);

/// A dynamically-scaled SIMD vector with a minimum of 32 lanes containing `i8` bits each.
pub const I8X32XN: Type = Type(0x146);

/// A dynamically-scaled SIMD vector with a minimum of 16 lanes containing `i16` bits each.
pub const I16X16XN: Type = Type(0x137);

/// A dynamically-scaled SIMD vector with a minimum of 8 lanes containing `i32` bits each.
pub const I32X8XN: Type = Type(0x128);

/// A dynamically-scaled SIMD vector with a minimum of 4 lanes containing `i64` bits each.
pub const I64X4XN: Type = Type(0x119);

/// A dynamically-scaled SIMD vector with a minimum of 2 lanes containing `i128` bits each.
pub const I128X2XN: Type = Type(0x10a);

/// A dynamically-scaled SIMD vector with a minimum of 8 lanes containing `f32` bits each.
pub const F32X8XN: Type = Type(0x12b);

/// A dynamically-scaled SIMD vector with a minimum of 4 lanes containing `f64` bits each.
pub const F64X4XN: Type = Type(0x11c);

/// A SIMD vector with 64 lanes containing a `b8` each.
pub const B8X64: Type = Type(0xd1);

/// A SIMD vector with 32 lanes containing a `b16` each.
pub const B16X32: Type = Type(0xc2);

/// A SIMD vector with 16 lanes containing a `b32` each.
pub const B32X16: Type = Type(0xb3);

/// A SIMD vector with 8 lanes containing a `b64` each.
pub const B64X8: Type = Type(0xa4);

/// A SIMD vector with 4 lanes containing a `b128` each.
pub const B128X4: Type = Type(0x95);

/// A SIMD vector with 64 lanes containing a `i8` each.
pub const I8X64: Type = Type(0xd6);

/// A SIMD vector with 32 lanes containing a `i16` each.
pub const I16X32: Type = Type(0xc7);

/// A SIMD vector with 16 lanes containing a `i32` each.
pub const I32X16: Type = Type(0xb8);

/// A SIMD vector with 8 lanes containing a `i64` each.
pub const I64X8: Type = Type(0xa9);

/// A SIMD vector with 4 lanes containing a `i128` each.
pub const I128X4: Type = Type(0x9a);

/// A SIMD vector with 16 lanes containing a `f32` each.
pub const F32X16: Type = Type(0xbb);

/// A SIMD vector with 8 lanes containing a `f64` each.
pub const F64X8: Type = Type(0xac);

/// A dynamically-scaled SIMD vector with a minimum of 64 lanes containing `b8` bits each.
pub const B8X64XN: Type = Type(0x151);

/// A dynamically-scaled SIMD vector with a minimum of 32 lanes containing `b16` bits each.
pub const B16X32XN: Type = Type(0x142);

/// A dynamically-scaled SIMD vector with a minimum of 16 lanes containing `b32` bits each.
pub const B32X16XN: Type = Type(0x133);

/// A dynamically-scaled SIMD vector with a minimum of 8 lanes containing `b64` bits each.
pub const B64X8XN: Type = Type(0x124);

/// A dynamically-scaled SIMD vector with a minimum of 4 lanes containing `b128` bits each.
pub const B128X4XN: Type = Type(0x115);

/// A dynamically-scaled SIMD vector with a minimum of 64 lanes containing `i8` bits each.
pub const I8X64XN: Type = Type(0x156);

/// A dynamically-scaled SIMD vector with a minimum of 32 lanes containing `i16` bits each.
pub const I16X32XN: Type = Type(0x147);

/// A dynamically-scaled SIMD vector with a minimum of 16 lanes containing `i32` bits each.
pub const I32X16XN: Type = Type(0x138);

/// A dynamically-scaled SIMD vector with a minimum of 8 lanes containing `i64` bits each.
pub const I64X8XN: Type = Type(0x129);

/// A dynamically-scaled SIMD vector with a minimum of 4 lanes containing `i128` bits each.
pub const I128X4XN: Type = Type(0x11a);

/// A dynamically-scaled SIMD vector with a minimum of 16 lanes containing `f32` bits each.
pub const F32X16XN: Type = Type(0x13b);

/// A dynamically-scaled SIMD vector with a minimum of 8 lanes containing `f64` bits each.
pub const F64X8XN: Type = Type(0x12c);

