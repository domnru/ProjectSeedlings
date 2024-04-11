#[allow(dead_code)]
pub fn untrust_input(data: String) -> String {
    let input = untrusted::Input::from(data.as_bytes());
    return std::str::from_utf8(input.as_slice_less_safe()).unwrap().to_owned();
}

#[allow(dead_code)]
pub fn clean_string(data: String) -> String {
    return ammonia::clean(&data);
}

#[allow(dead_code)]
pub fn secure_string(data: String) -> secstr::SecStr {
    secstr::SecStr::new(data.as_bytes().to_vec())
}
