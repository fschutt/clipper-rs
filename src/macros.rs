macro_rules! near_zero {
    ($val:expr) => ((($val) > -TOLERANCE) && ((val) < TOLERANCE))
}

// Returns [hi, lo] array of int128
// port of Int128Mul
macro_rules! int128mul {
    ($lhs:expr, $rhs:expr) => ({
        let negate = ($lhs < 0) != ($rhs < 0);

        if $lhs < 0 { $lhs = -$lhs };
        let int1_hi = $lhs as u64 >> 32;
        let int1_lo = ($lhs & 0xFFFFFFFF) as u64;

        if $rhs < 0 { $rhs = -$rhs };
        let int2_hi = $rhs as u64 >> 32;
        let int2_lo = ($rhs & 0xFFFFFFFF) as u64;

        //nb: see comments in clipper.pas
        let a = int1_hi * int2_hi;
        let b = int1_lo * int2_lo;
        let c = int1_hi * int2_lo + int1_lo * int2_hi;

        let c1 = c << 32;

        let mut tmp: (i64, u64) = ((a + c1) as i64, (b + c1)); // emulate Int128
        if tmp.1 < b {tmp.0 += 1; };
        if negate {
            if tmp.1 == 0 {
                tmp.0 = -tmp.0;
            } else {
                tmp.0 = !tmp.0;
                tmp.1 = (!tmp.1) - 1;
            }
        };

        tmp
    })
}
