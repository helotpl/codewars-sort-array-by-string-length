fn main() {
    println!("{:?}", sort_by_length(&["wdf".to_string(),"c".to_string(),"bb".to_string(),"xyz".to_string()]));
}

fn sort_by_length(arr: &[String]) -> Vec<String> {
    let mut v = Vec::from(arr);
    v.sort_by(|a, b| a.len().cmp(&b.len()));
    v
}
