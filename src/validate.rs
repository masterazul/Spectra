use crate::util::digits;

pub fn cpf(input: &str) -> bool {
    let d = digits(input);
    if d.len() != 11 {
        return false;
    }
    let n: Vec<u32> = d.bytes().map(|b| u32::from(b - b'0')).collect();
    if n.iter().all(|&x| x == n[0]) {
        return false;
    }
    for len in [9usize, 10usize] {
        let sum: u32 = n[..len]
            .iter()
            .enumerate()
            .map(|(i, &x)| x * (len + 1 - i) as u32)
            .sum();
        let mut dv = (sum * 10) % 11;
        if dv == 10 {
            dv = 0;
        }
        if dv != n[len] {
            return false;
        }
    }
    true
}

pub fn cnpj(input: &str) -> bool {
    let d = digits(input);
    if d.len() != 14 {
        return false;
    }
    let n: Vec<u32> = d.bytes().map(|b| u32::from(b - b'0')).collect();
    if n.iter().all(|&x| x == n[0]) {
        return false;
    }
    let check = |weights: &[u32]| -> u32 {
        let sum: u32 = weights.iter().enumerate().map(|(i, w)| n[i] * w).sum();
        let r = sum % 11;
        if r < 2 {
            0
        } else {
            11 - r
        }
    };
    let w1 = [5u32, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let w2 = [6u32, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    check(&w1) == n[12] && check(&w2) == n[13]
}
