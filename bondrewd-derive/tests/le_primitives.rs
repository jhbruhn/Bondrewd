use bondrewd::*;

#[derive(Bitfields, Clone, PartialEq, Eq, Debug)]
#[bondrewd(default_endianness = "le")]
struct Simple {
    #[bondrewd(bit_length = 3)]
    one: u8,
    #[bondrewd(bit_length = 27)]
    two: u32,
    #[bondrewd(bit_length = 14)]
    three: u16,
    four: u8,
}

#[test]
fn le_into_bytes_simple() -> anyhow::Result<()> {
    let simple = Simple {
        one: 2,
        two: 6345,
        three: 2145,
        four: 66,
    };
    assert_eq!(Simple::BYTE_SIZE, 7);
    let bytes = simple.clone().into_bytes();
    assert_eq!(bytes.len(), 7);
    assert_eq!(bytes[0], 0b010_11001);
    assert_eq!(bytes[1], 0b00100011);
    assert_eq!(bytes[2], 0b00000000);
    assert_eq!(bytes[3], 0b000000_01);
    assert_eq!(bytes[4], 0b10000100);
    assert_eq!(bytes[5], 0b1000_0100);
    // this last 4 bits here don't exist in the struct
    assert_eq!(bytes[6], 0b0010_0000);
    #[cfg(feature = "slice_fns")]
    {
        //peeks
        assert_eq!(simple.one, Simple::peek_slice_one(&bytes)?);
        assert_eq!(simple.two, Simple::peek_slice_two(&bytes)?);
        assert_eq!(simple.three, Simple::peek_slice_three(&bytes)?);
        assert_eq!(simple.four, Simple::peek_slice_four(&bytes)?);
    }

    // from_bytes
    let new_simple = Simple::from_bytes(bytes);
    assert_eq!(simple, new_simple);
    Ok(())
}

#[derive(Bitfields, Clone, PartialEq, Eq, Debug)]
#[bondrewd(default_endianness = "le", reverse)]
struct SimpleWithFlip {
    one: bool,
    #[bondrewd(bit_length = 10)]
    two: u16,
    #[bondrewd(bit_length = 5)]
    three: u8,
}
#[test]
fn le_into_bytes_simple_with_reverse() -> anyhow::Result<()> {
    let simple = SimpleWithFlip {
        one: false,
        two: u16::MAX & 0b0000001111111111,
        three: 0,
    };
    assert_eq!(SimpleWithFlip::BYTE_SIZE, 2);
    let bytes = simple.clone().into_bytes();
    assert_eq!(bytes.len(), 2);

    assert_eq!(bytes[1], 0b01111111);
    assert_eq!(bytes[0], 0b11100000);
    #[cfg(feature = "slice_fns")]
    {
        //peeks
        assert_eq!(simple.one, SimpleWithFlip::peek_slice_one(&bytes)?);
        assert_eq!(simple.two, SimpleWithFlip::peek_slice_two(&bytes)?);
        assert_eq!(simple.three, SimpleWithFlip::peek_slice_three(&bytes)?);
    }

    // from_bytes
    let new_simple = SimpleWithFlip::from_bytes(bytes);
    assert_eq!(simple, new_simple);
    Ok(())
}

#[derive(Bitfields, Clone, PartialEq, Eq, Debug)]
#[bondrewd(default_endianness = "le", read_from = "lsb0")]
struct SimpleWithReadFromBack {
    one: bool,
    #[bondrewd(bit_length = 10)]
    two: u16,
    #[bondrewd(bit_length = 5)]
    three: u8,
}
#[test]
fn le_into_bytes_simple_with_read_from_back() -> anyhow::Result<()> {
    let simple = SimpleWithReadFromBack {
        one: false,
        two: u16::MAX & 0b0000001111111111,
        three: 0,
    };
    assert_eq!(SimpleWithReadFromBack::BYTE_SIZE, 2);
    let bytes = simple.clone().into_bytes();
    assert_eq!(bytes.len(), 2);

    assert_eq!(bytes[0], 0b00000111);
    assert_eq!(bytes[1], 0b11111110);
    #[cfg(feature = "slice_fns")]
    {
        //peeks
        assert_eq!(simple.one, SimpleWithReadFromBack::peek_slice_one(&bytes)?);
        assert_eq!(simple.two, SimpleWithReadFromBack::peek_slice_two(&bytes)?);
        assert_eq!(
            simple.three,
            SimpleWithReadFromBack::peek_slice_three(&bytes)?
        );
    }

    // from_bytes
    let new_simple = SimpleWithReadFromBack::from_bytes(bytes);
    assert_eq!(simple, new_simple);
    Ok(())
}

#[derive(Bitfields, Clone, PartialEq, Debug)]
#[bondrewd(default_endianness = "le")]
struct SimpleWithFloats {
    #[bondrewd(bit_length = 27)]
    one: f32,
    #[bondrewd(bit_length = 60)]
    two: f64,
    #[bondrewd(bit_length = 19)]
    three: f32,
}

#[test]
fn le_into_bytes_simple_floating_point() -> anyhow::Result<()> {
    let simple = SimpleWithFloats {
        one: f32::from_bits(0x00000000_u32),
        two: f64::from_bits(0x09A1D45EE54D1A90_u64),
        three: f32::from_bits(0x0001D45E_u32),
    };
    let bytes = simple.clone().into_bytes();
    for byte in bytes {
        print!("{:08b}", byte);
    }
    print!("\n");
    #[cfg(feature = "slice_fns")]
    {
        //peeks
        assert_eq!(simple.one, SimpleWithFloats::peek_slice_one(&bytes)?);
        //assert_eq!(simple.two, SimpleWithFloats::peek_slice_two(&bytes)?);
        //assert_eq!(simple.three, SimpleWithFloats::peek_slice_three(&bytes)?);
    }

    // from_bytes
    let new_simple = SimpleWithFloats::from_bytes(bytes);
    assert_eq!(simple, new_simple);
    Ok(())
}
