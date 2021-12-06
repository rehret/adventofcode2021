pub fn parse_binary_string(binary: &String) -> Result<u32, String> {
    u32::from_str_radix(binary, 2).or_else(|err| {
        Err(format!(
            "Could not parse '{}' into integer:\n{}",
            binary,
            err.to_string()
        ))
    })
}