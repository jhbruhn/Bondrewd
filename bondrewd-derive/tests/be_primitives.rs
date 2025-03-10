use bondrewd::*;

#[derive(Bitfields, Clone, PartialEq, Eq, Debug)]
#[bondrewd(default_endianness = "be")]
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
fn be_into_bytes_simple() -> anyhow::Result<()> {
    let simple = Simple {
        one: 2,
        two: 6345,
        three: 2145,
        four: 66,
    };
    assert_eq!(Simple::BYTE_SIZE, 7);
    let bytes = simple.clone().into_bytes();
    assert_eq!(bytes.len(), 7);
    assert_eq!(bytes[0], 0b010_00000);
    assert_eq!(bytes[1], 0b00000000);
    assert_eq!(bytes[2], 0b01100011);
    assert_eq!(bytes[3], 0b001001_00);
    assert_eq!(bytes[4], 0b10000110);
    assert_eq!(bytes[5], 0b0001_0100);
    // this last 4 bits here don't exist in the struct
    assert_eq!(bytes[6], 0b0010_0000);
    #[cfg(feature = "slice_fns")]
    {
        //peeks
        assert_eq!(simple.one, Simple::read_slice_one(&bytes)?);
        assert_eq!(simple.two, Simple::read_slice_two(&bytes)?);
        assert_eq!(simple.three, Simple::read_slice_three(&bytes)?);
        assert_eq!(simple.four, Simple::read_slice_four(&bytes)?);
    }

    // from_bytes
    let new_simple = Simple::from_bytes(bytes);
    assert_eq!(simple, new_simple);
    Ok(())
}

#[derive(Bitfields, Clone, PartialEq, Eq, Debug)]
#[bondrewd(default_endianness = "be", reverse)]
struct SimpleWithFlip {
    one: bool,
    #[bondrewd(bit_length = 10)]
    two: u16,
    #[bondrewd(bit_length = 5)]
    three: u8,
}
#[test]
fn be_into_bytes_simple_with_reverse() -> anyhow::Result<()> {
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
        assert_eq!(simple.one, SimpleWithFlip::read_slice_one(&bytes)?);
        assert_eq!(simple.two, SimpleWithFlip::read_slice_two(&bytes)?);
        assert_eq!(simple.three, SimpleWithFlip::read_slice_three(&bytes)?);
    }

    // from_bytes
    let new_simple = SimpleWithFlip::from_bytes(bytes);
    assert_eq!(simple, new_simple);
    Ok(())
}

#[derive(Bitfields, Clone, PartialEq, Eq, Debug)]
#[bondrewd(default_endianness = "be", read_from = "lsb0")]
struct SimpleWithReadFromBack {
    one: bool,
    #[bondrewd(bit_length = 10)]
    two: u16,
    #[bondrewd(bit_length = 5)]
    three: u8,
}
#[test]
fn be_into_bytes_simple_with_read_from_back() -> anyhow::Result<()> {
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
        assert_eq!(simple.one, SimpleWithReadFromBack::read_slice_one(&bytes)?);
        assert_eq!(simple.two, SimpleWithReadFromBack::read_slice_two(&bytes)?);
        assert_eq!(
            simple.three,
            SimpleWithReadFromBack::read_slice_three(&bytes)?
        );
    }

    // from_bytes
    let new_simple = SimpleWithReadFromBack::from_bytes(bytes);
    assert_eq!(simple, new_simple);
    Ok(())
}

#[derive(Bitfields, Clone, PartialEq, Eq, Debug)]
#[bondrewd(default_endianness = "be", read_from = "msb0")]
struct SimpleWithReserve {
    #[bondrewd(bit_length = 9)]
    one: u16,
    #[bondrewd(bit_length = 3, reserve)]
    reserve: u8,
    #[bondrewd(bit_length = 4)]
    two: i8,
}

#[test]
fn be_into_bytes_simple_with_reserve_field() -> anyhow::Result<()> {
    let mut simple = SimpleWithReserve {
        one: 341,
        reserve: u8::MAX,
        two: -1,
    };
    assert_eq!(SimpleWithReserve::BYTE_SIZE, 2);
    #[cfg(feature = "slice_fns")]
    let mut bytes: [u8; 2] = simple.clone().into_bytes();
    #[cfg(not(feature = "slice_fns"))]
    let bytes: [u8; 2] = simple.clone().into_bytes();
    assert_eq!(bytes.len(), 2);

    assert_eq!(bytes[0], 0b10101010);
    assert_eq!(bytes[1], 0b10001111);
    #[cfg(feature = "slice_fns")]
    {
        //peeks
        assert_eq!(simple.one, SimpleWithReserve::read_slice_one(&bytes)?);
        assert_eq!(0, SimpleWithReserve::read_slice_reserve(&bytes)?);
        assert_eq!(simple.two, SimpleWithReserve::read_slice_two(&bytes)?);
        // TODO write more set slice tests
        SimpleWithReserve::write_slice_one(&mut bytes, 0)?;
        SimpleWithReserve::write_slice_reserve(&mut bytes, 7)?;
        SimpleWithReserve::write_slice_two(&mut bytes, 0)?;
        simple.one = 0;
        simple.two = 0;
    }
    #[cfg(feature = "slice_fns")]
    assert_eq!(7, SimpleWithReserve::read_reserve(&bytes));
    #[cfg(not(feature = "slice_fns"))]
    assert_eq!(0, SimpleWithReserve::read_reserve(&bytes));
    assert!(SimpleWithReserve::read_reserve(&bytes) != simple.reserve);
    simple.reserve = 0;
    // from_bytes
    let new_simple = SimpleWithReserve::from_bytes(bytes);
    assert_eq!(simple, new_simple);
    Ok(())
}
