pub struct SimonCipher {
    rk: [u64; 72],
}

const Z: [u64; 62] = [
    1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0,
    0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1,
];

impl SimonCipher {
    pub fn new(k: &[u64]) -> Self {
        let mut rk = [0u64; 72];

        for i in 0..4 {
            rk[i] = k[i];
        }

        for i in 4..72 {
            let mut t = rk[i - 1].rotate_right(3);
            t ^= rk[i - 3];
            t ^= t.rotate_right(1);
            rk[i] = !rk[i - 4] ^ t ^ Z[(i - 4) % 62] ^ 3;
        }

        Self { rk }
    }

    pub fn encrypt(&self, pt: &[u64]) -> Vec<u64> {
        let mut ct = vec![0u64; pt.len()];
        ct[0] = pt[0];
        ct[1] = pt[1];

        for i in 0..72 {
            let t = ct[1];
            ct[1] = ct[0]
                ^ (ct[1].rotate_left(1) & ct[1].rotate_left(8))
                ^ ct[1].rotate_left(2)
                ^ self.rk[i];
            ct[0] = t;
        }

        ct
    }

    pub fn decrypt(&self, ct: &[u64]) -> Vec<u64> {
        let mut pt = vec![0u64; ct.len()];
        pt[0] = ct[0];
        pt[1] = ct[1];

        for i in (0..72).rev() {
            let t = pt[0];
            pt[0] = pt[1]
                ^ (pt[0].rotate_left(1) & pt[0].rotate_left(8))
                ^ pt[0].rotate_left(2)
                ^ self.rk[i];
            pt[1] = t
        }

        pt
    }
}

#[test]
fn matches_test_vector() {
    let key: [u64; 4] = [
        0x0706050403020100,
        0x0f0e0d0c0b0a0908,
        0x1716151413121110,
        0x1f1e1d1c1b1a1918,
    ];

    let pt: [u64; 2] = [0x6d69732061207369, 0x74206e69206d6f6f];
    let cipher = SimonCipher::new(&key);
    let ct = cipher.encrypt(&pt);

    assert_eq!(ct, [0x3bf72a87efe7b868, 0x8d2b5579afc8a3a0]);
}
