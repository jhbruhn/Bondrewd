//! Fast and easy bitfield proc macro
//!
//! Provides a proc macro for compressing a data structure with data which can be expressed with bit
//! lengths that are not a power of Two.
//! 
//! # Derive Bitfields
//! - Implements the [`Bitfields`](https://docs.rs/bondrewd/latest/bondrewd/trait.Bitfields.html) trait 
//! which offers from\into bytes functions that are non-failable and convert the struct from/into sized 
//! u8 arrays ([u8; {total_bit_length * 8}]).
//! - read and write functions that allow the field to be accessed or overwritten within a sized u8 array.
//! - how each field is handled (bit length, endianness, ..), as well as structure wide effects 
//! (bit position, default field endianness, ..), can be found on the [`Bitfields Derive`](Bitfields)
//! page.
//!
//! For example we can define a data structure with 5 total bytes as:
//! - a field named one will be the first 3 bits.
//! - a field named two will be the next 19 bits.
//! - a field named six will be the next 14 bits.
//! - a field named four will be the next 4 bits.
//!
//!
//! ```
//! // Users code
//! use bondrewd::*;
//! #[derive(Bitfields)]
//! #[bondrewd(default_endianness = "be")]
//! struct SimpleExample {
//!     // fields that are as expected do not require attributes.
//!     one: bool,
//!     two: f32,
//!     #[bondrewd(bit_length = 14)]
//!     three: i16,
//!     #[bondrewd(bit_length = 6)]
//!     four: u8,
//! }
//! ```
//! ```compile_fail
//! // Generated Code
//! impl Bitfields<7usize> for SimpleExample {
//!     const BIT_SIZE: usize = 53usize;
//!     fn into_bytes(self) -> [u8; 7usize] { .. }
//!     fn from_bytes(mut input_byte_buffer: [u8; 7usize]) -> Self { .. }
//! }
//! impl SimpleExample {
//!     pub fn read_one(input_byte_buffer: &[u8; 7usize]) -> bool { .. }
//!     pub fn read_two(input_byte_buffer: &[u8; 7usize]) -> f32 { .. }
//!     pub fn read_three(input_byte_buffer: &[u8; 7usize]) -> i16 { .. }
//!     pub fn read_four(input_byte_buffer: &[u8; 7usize]) -> u8 { .. }
//!     pub fn write_one(output_byte_buffer: &mut [u8; 7usize], mut one: bool) { .. }
//!     pub fn write_two(output_byte_buffer: &mut [u8; 7usize], mut two: f32) { .. }
//!     pub fn write_three(output_byte_buffer: &mut [u8; 7usize], mut three: i16) { .. }
//!     pub fn write_four(output_byte_buffer: &mut [u8; 7usize], mut four: u8) { .. }
//! }
//! ```
//! # Derive BitfieldEnum
//! - Implements the [`BitfieldEnum`](https://docs.rs/bondrewd/latest/bondrewd/trait.BitfieldEnum.html)
//! trait which offers from\into primitive functions that are non-failable and convert the enum from/into
//! a primitive type (u8 is the only currently testing primitive).
//! - more information about controlling the end result (define variant values, define a catch/invalid
//! variant) can be found on the [`BitfieldEnum Derive`](BitfieldEnum) page.
//! 
//! ```
//! // Users code
//! use bondrewd::*;
//! #[derive(BitfieldEnum, PartialEq, Debug)]
//! enum SimpleEnum {
//!     Zero,
//!     One,
//!     Six = 6,
//!     Two,
//! }
//! #[derive(Bitfields)]
//! #[bondrewd(default_endianness = "le")]
//! struct StructWithEnumExample {
//!     #[bondrewd(bit_length = 3)]
//!     one: u8,
//!     #[bondrewd(enum_primitive = "u8", bit_length = 2)]
//!     two: SimpleEnum,
//!     #[bondrewd(bit_length = 3)]
//!     three: u8,
//! }
//! ```
//! ```compile_fail
//! impl bondrewd::BitfieldEnum for SimpleEnum {
//!     type Primitive = u8;
//!     fn into_primitive(self) -> u8 {
//!         match self {
//!             Self::Zero => 0,
//!             Self::One => 1,
//!             Self::Six => 6,
//!             Self::Two => 2,
//!         }
//!     }
//!     fn from_primitive(input: u8) -> Self {
//!         match input {
//!             0 => Self::Zero,
//!             1 => Self::One,
//!             6 => Self::Six,
//!             _ => Self::Two,
//!         }
//!     }
//! }
//! // Generated Struct Code
//! impl Bitfields<1usize> for StructWithEnumExample {
//! const BIT_SIZE: usize = 8usize;
//!     fn into_bytes(self) -> [u8; 1usize] {..}
//!     fn from_bytes(mut input_byte_buffer: [u8; 1usize]) -> Self {..}
//! }
//! impl StructWithEnumExample {
//!     pub fn read_one(input_byte_buffer: &[u8; 1usize]) -> u8 {..}
//!     pub fn read_two(input_byte_buffer: &[u8; 1usize]) -> SimpleEnum {..}
//!     pub fn read_three(input_byte_buffer: &[u8; 1usize]) -> u8 {..}
//!     pub fn write_one(output_byte_buffer: &mut [u8; 1usize], mut one: u8) {..}
//!     pub fn write_two(output_byte_buffer: &mut [u8; 1usize], mut two: SimpleEnum) {..}
//!     pub fn write_three(output_byte_buffer: &mut [u8; 1usize], mut three: u8) {..}
//! }
//! ```
//! 
//! # Other Crate Features
//! * `slice_fns` generates slice functions:
//!     * `fn read_slice_{field}(&[u8]) -> [Result<{field_type}, bondrewd::BondrewdSliceError>] {}`
//!     * `fn set_slice_{field}(&mut [u8], {field_type}) -> [Result<(), bondrewd::BondrewdSliceError>] {}`
//! * `hex_fns` provided from/into hex functions like from/into bytes. the hex inputs/outputs are \[u8;N\]
//! where N is double the calculated bondrewd STRUCT_SIZE. hex encoding and decoding is based off the
//! [hex](https://crates.io/crates/hex) crate's from/into slice functions but with statically sized
//! arrays so we could eliminate sizing errors.
//!
//! ### Full Example Generated code
//! ```
//! use bondrewd::*;
//! struct SimpleExample {
//!     one: bool,
//!     two: f32,
//!     three: i16,
//!     four: u8,
//! }
//! impl Bitfields<7usize> for SimpleExample {
//!     const BIT_SIZE: usize = 53usize;
//!     fn into_bytes(self) -> [u8; 7usize] {
//!         let mut output_byte_buffer: [u8; 7usize] = [0u8; 7usize];
//!         let one = self.one;
//!         output_byte_buffer[0usize] |= ((one as u8) << 7usize) & 128u8;
//!         let two = self.two;
//!         let two_bytes = (two.to_bits().rotate_left(7u32)).to_be_bytes();
//!         output_byte_buffer[0usize] |= two_bytes[3usize] & 127u8;
//!         output_byte_buffer[1usize] |= two_bytes[3usize] & 128u8;
//!         let three = self.three;
//!         let three_bytes = (three.rotate_left(1u32)).to_be_bytes();
//!         output_byte_buffer[4usize] |= three_bytes[0usize] & 127u8;
//!         output_byte_buffer[5usize] |= three_bytes[1usize] & 254u8;
//!         let four = self.four;
//!         let four_bytes = (four.rotate_right(5u32)).to_be_bytes();
//!         output_byte_buffer[5usize] |= four_bytes[0usize] & 1u8;
//!         output_byte_buffer[6usize] |= four_bytes[0] & 248u8;
//!         output_byte_buffer
//!     }
//!     fn from_bytes(mut input_byte_buffer: [u8; 7usize]) -> Self {
//!         let one = Self::read_one(&input_byte_buffer);
//!         let two = Self::read_two(&input_byte_buffer);
//!         let three = Self::read_three(&input_byte_buffer);
//!         let four = Self::read_four(&input_byte_buffer);
//!         Self {
//!             one,
//!             two,
//!             three,
//!             four,
//!         }
//!     }
//! }
//! impl SimpleExample {
//!     #[inline]
//!     pub fn read_one(input_byte_buffer: &[u8; 7usize]) -> bool {
//!         (((input_byte_buffer[0usize] & 128u8) >> 7usize) != 0)
//!     }
//!     #[inline]
//!     pub fn read_two(input_byte_buffer: &[u8; 7usize]) -> f32 {
//!         f32::from_bits(
//!             u32::from_be_bytes({
//!                 let mut two_bytes: [u8; 4usize] = [0u8; 4usize];
//!                 two_bytes[3usize] |= input_byte_buffer[0usize] & 127u8;
//!                 two_bytes[3usize] |= input_byte_buffer[1usize] & 128u8;
//!                 two_bytes
//!             })
//!             .rotate_right(7u32),
//!         )
//!     }
//!     #[inline]
//!     pub fn read_three(input_byte_buffer: &[u8; 7usize]) -> i16 {
//!         i16::from_be_bytes({
//!             let mut three_bytes: [u8; 2usize] = if (input_byte_buffer[4usize] & 64u8) == 64u8 {
//!                 [128u8, 1u8]
//!             } else {
//!                 [0u8; 2usize]
//!             };
//!             three_bytes[0usize] |= input_byte_buffer[4usize] & 127u8;
//!             three_bytes[1usize] |= input_byte_buffer[5usize] & 254u8;
//!             three_bytes
//!         })
//!         .rotate_right(1u32)
//!     }
//!     #[inline]
//!     pub fn read_four(input_byte_buffer: &[u8; 7usize]) -> u8 {
//!         u8::from_be_bytes({
//!             let mut four_bytes: [u8; 1usize] = [0u8; 1usize];
//!             four_bytes[0usize] |= input_byte_buffer[5usize] & 1u8;
//!             four_bytes[0] |= input_byte_buffer[6usize] & 248u8;
//!             four_bytes
//!         })
//!         .rotate_left(5u32)
//!     }
//!     #[inline]
//!     pub fn write_one(output_byte_buffer: &mut [u8; 7usize], mut one: bool) {
//!         output_byte_buffer[0usize] &= 127u8;
//!         output_byte_buffer[0usize] |= ((one as u8) << 7usize) & 128u8;
//!     }
//!     #[inline]
//!     pub fn write_two(output_byte_buffer: &mut [u8; 7usize], mut two: f32) {
//!         output_byte_buffer[0usize] &= 128u8;
//!         output_byte_buffer[1usize] &= 127u8;
//!         let two_bytes = (two.to_bits().rotate_left(7u32)).to_be_bytes();
//!         output_byte_buffer[0usize] |= two_bytes[3usize] & 127u8;
//!         output_byte_buffer[1usize] |= two_bytes[3usize] & 128u8;
//!     }
//!     #[inline]
//!     pub fn write_three(output_byte_buffer: &mut [u8; 7usize], mut three: i16) {
//!         output_byte_buffer[4usize] &= 128u8;
//!         output_byte_buffer[5usize] &= 1u8;
//!         let three_bytes = (three.rotate_left(1u32)).to_be_bytes();
//!         output_byte_buffer[4usize] |= three_bytes[0usize] & 127u8;
//!         output_byte_buffer[5usize] |= three_bytes[1usize] & 254u8;
//!     }
//!     #[inline]
//!     pub fn write_four(output_byte_buffer: &mut [u8; 7usize], mut four: u8) {
//!         output_byte_buffer[5usize] &= 254u8;
//!         output_byte_buffer[6usize] &= 7u8;
//!         let four_bytes = (four.rotate_right(5u32)).to_be_bytes();
//!         output_byte_buffer[5usize] |= four_bytes[0usize] & 1u8;
//!         output_byte_buffer[6usize] |= four_bytes[0] & 248u8;
//!     }
//! }
//! ```
extern crate proc_macro;
mod enums;
use enums::parse::EnumInfo;
mod structs;
use structs::common::StructInfo;
use structs::from_bytes::create_from_bytes_field_quotes;
use structs::into_bytes::create_into_bytes_field_quotes;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

/// Generates an implementation of the bondrewd::Bitfield trait, as well as peek and set functions for direct
/// sized u8 arrays access.
/// 
/// # Supported Field Types
/// - All primitives other than usize and isize (i believe ambiguous sizing is bad for this type of work).
///     - Floats currently must be full sized.
///     - Its important to know that there is a small runtime cost for signed numbers.
/// - Enums which implement the BitfieldEnum trait in bondrewd.
/// - Structs which implement the Bitfield trait in bondrewd.
///
/// # Struct Attributes
/// - `default_endianness = {"le" or "be"}` describes a default endianness for primitive fields.
/// - `read_from = {"msb0" or "lsb0"}` defines bit positioning. which end of the byte array to start at.
/// - `enforce_bytes = {BYTES}` defines a required resulting BIT_SIZE divided by 8 of the structure in condensed form.
/// - `enforce_bits = {BYTES}` defines a required resulting BIT_SIZE of the structure in condensed form.
/// - `enforce_full_bytes` defines that the resulting BIT_SIZE is required to be a multiple of 8.
/// - `reverse` defines that the entire byte array should be reversed before reading. no runtime cost.
///
/// # Field Attributes
/// - `bit_length = {BITS}` define the total amount of bits to use when condensed.
/// - `byte_length = {BYTES}` define the total amount of bytes to use when condensed.
/// - `endianness = {"le" or "be"}` define per field endianess.
/// - `block_bit_length = {BITS}` describes a bit length for the entire array dropping lower indexes first. (default array type)
/// - `block_byte_length = {BYTES}` describes a byte length for the entire array dropping lower indexes first. (default array type)
/// - `element_bit_length = {BITS}` describes a bit length for each element of an array.
/// - `element_byte_length = {BYTES}` describes a byte length for each element of an array.
/// - `enum_primitive = "u8"` defines the size of the enum. the BitfieldEnum currently only supports u8.
/// - `struct_size = {SIZE}` defines the field as a struct which implements the Bitfield trait and the BYTE_SIZE const defined in said trait.
/// - `reserve` defines that this field should be ignored in from and into bytes functions.
/// - /!Untested!\ `bits = "RANGE"` - define the bit indexes yourself rather than let the proc macro figure
/// it out. using a rust range in quotes.
#[proc_macro_derive(Bitfields, attributes(bondrewd,))]
pub fn derive_bitfields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // parse the input into a StructInfo which contains all the information we
    // along with some helpful structures to generate our Bitfield code.
    let struct_info = match StructInfo::parse(&input) {
        Ok(parsed_struct) => parsed_struct,
        Err(err) => {
            return TokenStream::from(err.to_compile_error());
        }
    };
    // get the struct size and name so we can use them in a quote.
    let struct_size = struct_info.total_bytes();
    let struct_name = format_ident!("{}", struct_info.name);

    // get a list of all fields from_bytes logic which gets there bytes from an array called
    // input_byte_buffer.
    let slice_fns: bool;
    #[cfg(not(feature = "slice_fns"))]
    {
        slice_fns = false;
    }
    #[cfg(feature = "slice_fns")]
    {
        slice_fns = true;
    }
    // get a list of all fields into_bytes logic which puts there bytes into an array called
    // output_byte_buffer.
    let fields_into_bytes = match create_into_bytes_field_quotes(&struct_info, slice_fns) {
        Ok(ftb) => ftb,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };
    let fields_from_bytes = match create_from_bytes_field_quotes(&struct_info, slice_fns) {
        Ok(ffb) => ffb,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };
    // combine all of the into_bytes quotes separated by newlines
    let into_bytes_quote = fields_into_bytes.into_bytes_fn;
    let mut set_quotes = fields_into_bytes.set_field_fns;

    if let Some(set_slice_quote) = fields_into_bytes.set_slice_field_fns {
        set_quotes = quote! {
            #set_quotes
            #set_slice_quote
        }
    }

    let from_bytes_quote = fields_from_bytes.from_bytes_fn;
    let mut peek_quotes = fields_from_bytes.peek_field_fns;

    if let Some(peek_slice_quote) = fields_from_bytes.peek_slice_field_fns {
        peek_quotes = quote! {
            #peek_quotes
            #peek_slice_quote
        }
    }

    let setters: bool;
    #[cfg(not(feature = "setters"))]
    {
        setters = false;
    }
    #[cfg(feature = "setters")]
    {
        setters = true;
    }
    let setters_quote = if setters {
        match structs::struct_fns::create_into_bytes_field_quotes(&struct_info) {
            Ok(parsed_struct) => parsed_struct,
            Err(err) => {
                return TokenStream::from(err.to_compile_error());
            }
        }
    } else {
        quote! {}
    };

    let getter_setters_quotes = quote! {
        impl #struct_name {
            #peek_quotes
            #set_quotes
            #setters_quote
        }
    };
    let hex;
    #[cfg(feature = "hex_fns")]
    {
        hex = true;
    }
    #[cfg(not(feature = "hex_fns"))]
    {
        hex = false;
    }
    let hex_size = struct_size * 2;
    let hex_fns_quote = if hex {
        quote! {
            impl BitfieldHex<#hex_size> for #struct_name {
                fn from_hex(hex: [u8;#hex_size]) -> Result<Self, BitfieldHexError> {
                    let bytes: [u8; #struct_size] = [0;#struct_size];
                    let mut bytes: [u8; Self::BYTE_SIZE] = [0;Self::BYTE_SIZE];
                    for i in 0usize..#struct_size {
                        let index = i * 2;
                        let index2 = index + 1;
                        let decode_nibble = |c, c_i| match c {
                            b'A'..=b'F' => Ok(c - b'A' + 10u8),
                            b'a'..=b'f' => Ok(c - b'a' + 10u8),
                            b'0'..=b'9' => Ok(c - b'0'),
                            _ => return Err(BitfieldHexError(
                                c as char,
                                c_i,
                            )),
                        };
                        bytes[i] = ((decode_nibble(hex[index], index)? & 0b00001111) << 4) | decode_nibble(hex[index2], index2)?;
                    }
                    Ok(Self::from_bytes(bytes))

                }

                fn into_hex_upper(self) -> [u8;#hex_size] {
                    let bytes = self.into_bytes();
                    let mut output: [u8;#hex_size] = [0; #hex_size];
                    for (i, byte) in (0..#hex_size).step_by(2).zip(bytes) {
                        output[i] = (Self::UPPERS[((byte & 0b11110000) >> 4) as usize]);
                        output[i + 1] = (Self::UPPERS[(byte & 0b00001111) as usize]);
                    }
                    output
                }

                fn into_hex_lower(self) -> [u8;#hex_size] {
                    let bytes = self.into_bytes();
                    let mut output: [u8;#hex_size] = [0; #hex_size];
                    for (i, byte) in (0..#hex_size).step_by(2).zip(bytes) {
                        output[i] = (Self::LOWERS[((byte & 0b11110000) >> 4) as usize]);
                        output[i + 1] = (Self::LOWERS[(byte & 0b00001111) as usize]);
                    }
                    output
                }
            }
        }
    } else {
        quote! {}
    };

    // get the bit size of the entire set of fields to fill in trait requirement.
    let bit_size = struct_info.total_bits();

    // put it all together.
    // to_bytes_quote will put all of the fields in self into a array called output_byte_buffer.
    // so for into_bytes all we need is the fn declaration, the output_byte_buffer, and to return
    // that buffer.
    // from_bytes is essentially the same minus a variable because input_byte_buffer is the input.
    // slap peek quotes inside a impl block at the end and we good to go
    let to_bytes_quote = quote! {
        impl Bitfields<#struct_size> for #struct_name {
            const BIT_SIZE: usize = #bit_size;
            #into_bytes_quote
            #from_bytes_quote
        }
        #getter_setters_quotes
        #hex_fns_quote
    };

    TokenStream::from(to_bytes_quote)
}

/// Generates an implementation of bondrewd::BitfieldEnum trait.
/// 
/// # Features
/// - Generates code for the BitfieldEnum trait which allows an enum to be used by Bitfield structs.
/// - Literal values. ex. `Variant = 0,`
/// - Automatic Value Assignment for non-literal variants. Variants are assigned values starting from 0
/// incrementing by 1 skipping values taken by literal definitions (That means you can mix and match
/// inferred values a code defined literal values).
/// - Catch Variants
///     - Catch Value is a variant that will store values that don't match the reset of the variants.
///     using a Catch Value is as simple as making a variant with a primitive value (if the bondrewd_enum
///     attribute is present the primitive types must match). ex `InvalidVariant(u8),`.
///     - Catch All variant is used to insure that Results are not needed. Catch all will generate a
///     `_ => {..}` match arm so that enums don't need to have as many variants as there are values in 
///     the defined primitive. Catch all can be defined with a `#[invalid]` attribute or last variant will
///     Automatically become a catch all if no Catch is defined.
/// 
/// # Other Features
/// - Support for implementation of [`std::cmp::PartialEq`] for the given primitive (currently only u8)
#[proc_macro_derive(BitfieldEnum, attributes(bondrewd_enum))]
pub fn derive_bondrewd_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_info = match EnumInfo::parse(&input) {
        Ok(parsed_enum) => parsed_enum,
        Err(err) => {
            return TokenStream::from(err.to_compile_error());
        }
    };
    let into = match enums::into_bytes::generate_into_bytes(&enum_info) {
        Ok(i) => i,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };
    let from = match enums::from_bytes::generate_from_bytes(&enum_info) {
        Ok(f) => f,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };
    let partial_eq = enums::partial_eq::generate_partial_eq(&enum_info);
    let enum_name = enum_info.name;
    let primitive = enum_info.primitive;
    TokenStream::from(quote! {
        impl bondrewd::BitfieldEnum for #enum_name {
            type Primitive = #primitive;
            #into
            #from
        }

        #partial_eq
    })
}
