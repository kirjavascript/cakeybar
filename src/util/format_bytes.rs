pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0B".to_string();
    }
    const LEN: usize = 5;
    let bytes = bytes as f64;
    let sizes: [&str; LEN] = ["", "K", "M", "G", "T"];
    let index = ((bytes).ln() / 1024_f64.ln()).floor();
    let val = bytes / (1024_f64.powf(index));
    let index = index as usize;
    let suffix = if index < LEN { sizes[index] } else { "?" };
    format!("{:.*}{}B", if index < 2 { 0 } else { 2 }, val, suffix)
}
