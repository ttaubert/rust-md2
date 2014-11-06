/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this file,
* You can obtain one at http://mozilla.org/MPL/2.0/. */

static S: [u8, ..256] = [
    0x29, 0x2E, 0x43, 0xC9, 0xA2, 0xD8, 0x7C, 0x01, 0x3D, 0x36, 0x54, 0xA1, 0xEC, 0xF0, 0x06, 0x13,
    0x62, 0xA7, 0x05, 0xF3, 0xC0, 0xC7, 0x73, 0x8C, 0x98, 0x93, 0x2B, 0xD9, 0xBC, 0x4C, 0x82, 0xCA,
    0x1E, 0x9B, 0x57, 0x3C, 0xFD, 0xD4, 0xE0, 0x16, 0x67, 0x42, 0x6F, 0x18, 0x8A, 0x17, 0xE5, 0x12,
    0xBE, 0x4E, 0xC4, 0xD6, 0xDA, 0x9E, 0xDE, 0x49, 0xA0, 0xFB, 0xF5, 0x8E, 0xBB, 0x2F, 0xEE, 0x7A,
    0xA9, 0x68, 0x79, 0x91, 0x15, 0xB2, 0x07, 0x3F, 0x94, 0xC2, 0x10, 0x89, 0x0B, 0x22, 0x5F, 0x21,
    0x80, 0x7F, 0x5D, 0x9A, 0x5A, 0x90, 0x32, 0x27, 0x35, 0x3E, 0xCC, 0xE7, 0xBF, 0xF7, 0x97, 0x03,
    0xFF, 0x19, 0x30, 0xB3, 0x48, 0xA5, 0xB5, 0xD1, 0xD7, 0x5E, 0x92, 0x2A, 0xAC, 0x56, 0xAA, 0xC6,
    0x4F, 0xB8, 0x38, 0xD2, 0x96, 0xA4, 0x7D, 0xB6, 0x76, 0xFC, 0x6B, 0xE2, 0x9C, 0x74, 0x04, 0xF1,
    0x45, 0x9D, 0x70, 0x59, 0x64, 0x71, 0x87, 0x20, 0x86, 0x5B, 0xCF, 0x65, 0xE6, 0x2D, 0xA8, 0x02,
    0x1B, 0x60, 0x25, 0xAD, 0xAE, 0xB0, 0xB9, 0xF6, 0x1C, 0x46, 0x61, 0x69, 0x34, 0x40, 0x7E, 0x0F,
    0x55, 0x47, 0xA3, 0x23, 0xDD, 0x51, 0xAF, 0x3A, 0xC3, 0x5C, 0xF9, 0xCE, 0xBA, 0xC5, 0xEA, 0x26,
    0x2C, 0x53, 0x0D, 0x6E, 0x85, 0x28, 0x84, 0x09, 0xD3, 0xDF, 0xCD, 0xF4, 0x41, 0x81, 0x4D, 0x52,
    0x6A, 0xDC, 0x37, 0xC8, 0x6C, 0xC1, 0xAB, 0xFA, 0x24, 0xE1, 0x7B, 0x08, 0x0C, 0xBD, 0xB1, 0x4A,
    0x78, 0x88, 0x95, 0x8B, 0xE3, 0x63, 0xE8, 0x6D, 0xE9, 0xCB, 0xD5, 0xFE, 0x3B, 0x00, 0x1D, 0x39,
    0xF2, 0xEF, 0xB7, 0x0E, 0x66, 0x58, 0xD0, 0xE4, 0xA6, 0x77, 0x72, 0xF8, 0xEB, 0x75, 0x4B, 0x0A,
    0x31, 0x44, 0x50, 0xB4, 0x8F, 0xED, 0x1F, 0x1A, 0xDB, 0x99, 0x8D, 0x33, 0x9F, 0x11, 0x83, 0x14
];

#[inline]
fn md2_pad(msg: &mut Vec<u8>) {
    let pad = 16 - msg.len() % 16;
    msg.grow_fn(pad, |_| pad as u8);
    assert!(msg.len() % 16 == 0);
}

#[inline]
fn md2_checksum(msg: &mut Vec<u8>) {
    let mut checksum = [0u8, ..16];
    let mut l = 0u8;

    for chunk in msg.as_slice().chunks(16) {
        for (i, byte) in checksum.iter_mut().enumerate() {
            *byte ^= S[(chunk[i] ^ l) as uint];
            l = *byte;
        }
    }

    msg.push_all(checksum);
}

#[inline]
fn md2_digest(msg: &mut Vec<u8>) -> [u8, ..48] {
    let mut md = [0u8, ..48];

    for chunk in msg.as_slice().chunks(16) {
        for (i, byte) in chunk.iter().enumerate() {
            md[16 + i] = *byte;
            md[32 + i] = byte ^ md[i];
        }

        let mut t = 0u8;
        for i in range(0, 18) {
            for byte in md.iter_mut() {
                *byte ^= S[t as uint];
                t = *byte;
            }

            t += i;
        }
    }

    md
}

pub fn md2(msg: &[u8], out: &mut [u8]) {
    assert!(out.len() == 16);
    let mut msg = msg.to_vec();

    md2_pad(&mut msg);
    md2_checksum(&mut msg);

    let md = md2_digest(&mut msg);
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = md[i];
    }
}

#[cfg(test)]
mod test {
    use md2;

    fn hex(buf: &[u8]) -> String {
        buf.iter().fold(String::new(), |a, &b| format!("{}{:02x}", a, b))
    }

    fn digest(buf: &[u8]) -> [u8, ..16] {
        let mut digest = [0u8, ..16];
        md2(buf, &mut digest);
        digest
    }

    fn cmp(d: &str, s: &str) {
        assert_eq!(d.to_string(), hex(digest(s.as_bytes())));
    }

    #[test]
    fn test_md2() {
        cmp("8350e5a3e24c153df2275c9f80692773", "");
        cmp("32ec01ec4a6dac72c0ab96fb34c0b5d1", "a");
        cmp("da853b0d3f88d99b30283a69e6ded6bb", "abc");
        cmp("ab4f496bfb2a530b219ff33031fe06b0", "message digest");
        cmp("4e8ddff3650292ab5a4108c3aa47940b", "abcdefghijklmnopqrstuvwxyz");
        cmp("da33def2a42df13975352846c30338cd", "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
        cmp("d5976f79d83d3a0dc9806c3c66f3efd8", "12345678901234567890123456789012345678901234567890123456789012345678901234567890");
    }
}
