pub fn bin_to_int(bstr: &Vec<bool>) -> i64 {
    let mut converted: i64 = 0;
    let mut power = 0;
    for i in (0..bstr.len()).rev() {
        if bstr[i as usize]{
            converted += 2i64.pow(power);
        }
        power += 1;
    }
    converted
}
