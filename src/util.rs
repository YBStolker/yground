use tera::Context;

use crate::TEMPLATES;

pub fn get_template(path: &str, context: Option<Context>) -> Option<String> {
    TEMPLATES
        .render(path, &context.unwrap_or_default())
        .ok()
        .map(|value| (value))
}

pub fn hex_count(size: u32) -> u32 {
    if size == 0 {
        return 0;
    }

    size.pow(2) + (size - 1).pow(2) * 2 + size - 1
}

#[cfg(test)]
mod tests {
    use crate::util::hex_count;

    #[test]
    fn test_hex_count() {
        assert_eq!(1, hex_count(1));
        assert_eq!(7, hex_count(2));
        assert_eq!(19, hex_count(3));
        assert_eq!(37, hex_count(4));
        assert_eq!(61, hex_count(5));
        assert_eq!(91, hex_count(6));
        assert_eq!(127, hex_count(7));
        assert_eq!(169, hex_count(8));
        assert_eq!(217, hex_count(9));
        assert_eq!(271, hex_count(10));
    }
}
