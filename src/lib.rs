pub mod keys {
    const KICK_A: u8 = 36;
    const KICK_B: u8 = 35;
    const SNARE_A: u8 = 38;
    const SNARE_B: u8 = 40;
    const FLOOR_A: u8 = 41;
    const FLOOR_B: u8 = 43;
    const TOM_A: u8 = 45;
    const TOM_B: u8 = 47;
    const TOM_C: u8 = 48;
    // const hatC: u8= 42;
    const HAT_B: u8 = 44;
    const HAT_A: u8 = 46;
    const CRASH_A: u8 = 49;
    // const CRASH_B: u8= 57;
    const RIDE_A: u8 = 51;
    // const RIDE_B: u8= 53;
    const RIDE_C: u8 = 56;
    // const rideD: u8= 59;
    // const miscA: u8= 52;
    // const miscB: u8= 37;

    pub fn map_key(c: char) -> Option<u8> {
        let n = match c {
            'j' | 'h' => KICK_A,
            'l' => SNARE_A,
            'i' | 'ı' => HAT_A,
            'f' | 'k' => RIDE_A,
            'a' => CRASH_A,
            'q' => FLOOR_A,
            'g' => FLOOR_B,
            'e' => TOM_A,
            's' | 'p' => HAT_B,
            'r' => TOM_B,
            'w' | 'o' => SNARE_B,
            't' => TOM_C,
            'b' => HAT_B,
            'n' | 'm' => KICK_B,
            'd' => RIDE_C,
            _ => return None,
        };
        Some(n)
    }
}
