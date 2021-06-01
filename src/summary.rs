pub fn summary(l: &[u32]) -> Option<u32> {
    let mut temp: u64 = 0;
    for &i in l {
        temp += i as u64;
        if temp > 4294967295 {
            return None;
        }
    }
    Some(temp as u32)
}