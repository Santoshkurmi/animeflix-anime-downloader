use std::collections::HashMap;

pub fn parse_json(data: Option<String>) -> Option<json::JsonValue> {
    if let Some(output) = data {
        let json_out = json::parse(&output);
        if let Ok(out) = json_out {
            return Some(out);
        } else {
            return None;
        }
    } //handle error
    return None;
}

pub fn parse_quality_link(data: String, source: String) -> Option<(u64, String, String)> {
    let regex = regex::Regex::new(r#"NAME="([0-9]+)p"\n(.*)\n"#).unwrap();
    let mut result: HashMap<u64, String> = HashMap::new();
    for caputres in regex.captures_iter(&data) {
        let quality: u64 = caputres
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or_default();

        let link_path = caputres.get(2).unwrap().as_str();
        result.insert(quality, link_path.to_string());
    }
    if result.contains_key(&720) {
        let data = format!(
            "{}/{}",
            source,
            result.get(&720).unwrap_or(&"".to_string()).clone()
        );
        return Some((720, data, source));
    } else if result.contains_key(&480) {
        let data = format!(
            "{}/{}",
            source,
            result.get(&480).unwrap_or(&"".to_string()).clone()
        );

        return Some((480, data, source));
    } else {
        return None;
    }
}

pub fn parse_part(string: String) -> Option<Vec<String>> {
    let mut parts: Vec<String> = Vec::new();
    let re = regex::Regex::new(r"#EXTINF:.*,\n(.*)\n").unwrap();

    for matches in re.captures_iter(&string) {
        parts.push(matches.get(1).unwrap().as_str().to_string());
    }
    if parts.len() == 0 {
        return None;
    } else {
        return Some(parts);
    }
}
