pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let s = s.as_bytes();
        if num_rows < 1 || s.is_empty() {
            panic!("Precondition failed!");
        }
        let num_rows = num_rows as usize;
        let len = s.len();

        let mut res: Vec<u8> = Vec::with_capacity(len);
        let period = std::cmp::max(2 * num_rows - 2, 1);
        let periods = s.len().div_ceil(period);

        for row in 0..num_rows {
            for block in 0..periods {
                let fst = block * period + row;
                let snd = (block + 1) * period - row;
                if fst < len {
                    res.push(s[fst]);
                }
                if snd < len && row != 0 && row != (num_rows - 1) {
                    res.push(s[snd]);
                }
            }
        }

        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 3;
        assert_eq!(Solution::convert(s, num_rows), "PAHNAPLSIIGYIR");

        let s = String::from("PAYPALISHIRING");
        let num_rows = 4;
        assert_eq!(Solution::convert(s, num_rows), "PINALSIGYAHRPI");

        let s = String::from("A");
        let num_rows = 1;
        assert_eq!(Solution::convert(s, num_rows), "A");
    }
}
