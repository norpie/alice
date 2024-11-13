use super::item::WppItem;

pub fn format(item: &WppItem) -> String {
    let mut formatted = String::new();
    formatted.push_str(&format!("[{}(\"{}\")", item.item_type(), item.name()));
    formatted.push_str("\n{\n");
    for attribute in item.attributes() {
        formatted.push_str(&format!("    {}(", attribute.name()));
        let value_count = attribute.values().len();
        for (i, value) in attribute.values().iter().enumerate() {
            formatted.push_str(&format!("\"{}\"", value));
            if i < value_count - 1 {
                formatted.push_str(" + ");
            }
        }
        formatted.push_str(")\n");
    }
    formatted.push_str("}]");
    formatted
}
