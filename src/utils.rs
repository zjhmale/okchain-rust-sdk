use regex::Regex;

pub fn is_product(product: &str) -> bool {
    if let Ok(re) = Regex::new(r"^[a-zA-Z0-9-]+_[a-zA-Z0-9-]+$") {
        re.is_match(product)
    } else {
        false
    }
}

// FIXME: the side should be enum
#[allow(dead_code)]
pub fn is_productside(side: &str) -> bool {
    side == "BUY" || side == "SELL"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_product() {
        assert!(is_product("btc_usd"));
        assert_ne!(is_product("btcusd"), true);
    }

    #[test]
    fn test_is_productside() {
        assert!(is_productside("BUY"));
        assert_ne!(is_productside("bbb"), true);
    }
}
