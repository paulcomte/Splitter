// Generate custom color levels for the color cube
const CUBELEVELS: [u8; 6] = [0x00, 0x33, 0x66, 0x99, 0xcc, 0xff];
// Generate a list of midpoints of the above list
const SNAPS: [u8; 5] = [
    (CUBELEVELS[0] + CUBELEVELS[1]) / 2,
    (CUBELEVELS[1] + CUBELEVELS[2]) / 2,
    (CUBELEVELS[2] + CUBELEVELS[3]) / 2,
    ((CUBELEVELS[3] as u16 + CUBELEVELS[4] as u16) as u16 / 2) as u8,
    ((CUBELEVELS[4] as u16 + CUBELEVELS[5] as u16) / 2) as u8,
];

pub fn rgb_to_short(r: u8, g: u8, b: u8) -> [u8; 3] {
    let mut r_idx = SNAPS.iter().position(|&x| x > r).unwrap_or(5);
    let mut g_idx = SNAPS.iter().position(|&x| x > g).unwrap_or(5);
    let mut b_idx = SNAPS.iter().position(|&x| x > b).unwrap_or(5);

    // Simple colorcube transform
    if r_idx >= CUBELEVELS.len() {
        r_idx = CUBELEVELS.len() - 1;
    }
    if g_idx >= CUBELEVELS.len() {
        g_idx = CUBELEVELS.len() - 1;
    }
    if b_idx >= CUBELEVELS.len() {
        b_idx = CUBELEVELS.len() - 1;
    }

    [r_idx as u8 * 36 + g_idx as u8 * 6 + b_idx as u8 + 16; 3]
}
