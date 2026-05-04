use wasm_bindgen::{JsValue, prelude::*};

#[derive(Debug, PartialEq, Eq)]
struct OrderTotals {
    subtotal_cents: u32,
    discount_cents: u32,
    shipping_cents: u32,
    tax_cents: u32,
    total_cents: u32,
}

#[derive(Clone, Copy)]
enum Region {
    Jp,
    UsCa,
    Eu,
    Intl,
}

#[wasm_bindgen]
pub fn calculate_total_cents(
    subtotal_cents: u32,
    item_count: u32,
    region_code: &str,
    coupon_code: &str,
) -> Result<u32, JsValue> {
    Ok(compute_order(subtotal_cents, item_count, region_code, coupon_code)
        .map_err(|error| JsValue::from_str(&error))?
        .total_cents)
}

#[wasm_bindgen]
pub fn order_summary(
    subtotal_cents: u32,
    item_count: u32,
    region_code: &str,
    coupon_code: &str,
) -> Result<String, JsValue> {
    let totals = compute_order(subtotal_cents, item_count, region_code, coupon_code)
        .map_err(|error| JsValue::from_str(&error))?;
    let discounted_subtotal = totals.subtotal_cents.saturating_sub(totals.discount_cents);

    Ok(format!(
        "subtotal: {}\ndiscount: {}\nshipping: {}\ntax: {}\ntotal: {}",
        format_money(totals.subtotal_cents),
        format_money(totals.discount_cents),
        format_money(totals.shipping_cents),
        format_money(totals.tax_cents),
        format_money(discounted_subtotal + totals.shipping_cents + totals.tax_cents),
    ))
}

fn compute_order(
    subtotal_cents: u32,
    item_count: u32,
    region_code: &str,
    coupon_code: &str,
) -> Result<OrderTotals, String> {
    let region = parse_region(region_code)?;
    let discount_cents = discount_cents(subtotal_cents, item_count, coupon_code)?;
    let discounted_subtotal = subtotal_cents.saturating_sub(discount_cents);
    let shipping_cents = shipping_cents(discounted_subtotal, region, coupon_code);
    let tax_cents = tax_cents(discounted_subtotal + shipping_cents, region);
    let total_cents = discounted_subtotal + shipping_cents + tax_cents;

    Ok(OrderTotals {
        subtotal_cents,
        discount_cents,
        shipping_cents,
        tax_cents,
        total_cents,
    })
}

fn parse_region(region_code: &str) -> Result<Region, String> {
    match region_code.trim().to_ascii_lowercase().as_str() {
        "jp" => Ok(Region::Jp),
        "us-ca" => Ok(Region::UsCa),
        "eu" => Ok(Region::Eu),
        "intl" => Ok(Region::Intl),
        other => Err(format!("unsupported region code: {other}")),
    }
}

fn discount_cents(subtotal_cents: u32, item_count: u32, coupon_code: &str) -> Result<u32, String> {
    match coupon_code.trim().to_ascii_uppercase().as_str() {
        "" => Ok(0),
        "SAVE10" => Ok((subtotal_cents / 10).min(2_500)),
        "BULK5" if item_count >= 5 => Ok(500),
        "BULK5" => Ok(0),
        "FREESHIP" => Ok(0),
        other => Err(format!("unsupported coupon code: {other}")),
    }
}

fn shipping_cents(subtotal_cents: u32, region: Region, coupon_code: &str) -> u32 {
    if coupon_code.trim().eq_ignore_ascii_case("FREESHIP") && !matches!(region, Region::Intl) {
        return 0;
    }

    match region {
        Region::Jp => {
            if subtotal_cents >= 5_000 {
                0
            } else {
                600
            }
        }
        Region::UsCa => {
            if subtotal_cents >= 7_500 {
                0
            } else {
                800
            }
        }
        Region::Eu => {
            if subtotal_cents >= 9_000 {
                0
            } else {
                900
            }
        }
        Region::Intl => 2_400,
    }
}

fn tax_cents(taxable_cents: u32, region: Region) -> u32 {
    let basis_points = match region {
        Region::Jp => 1_000_u32,
        Region::UsCa => 825_u32,
        Region::Eu => 2_000_u32,
        Region::Intl => 0_u32,
    };

    (((taxable_cents as u64) * (basis_points as u64) + 5_000) / 10_000) as u32
}

fn format_money(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save10_affects_discount_and_taxable_total() {
        let totals = compute_order(12_000, 3, "jp", "SAVE10").unwrap();
        assert_eq!(totals.discount_cents, 1_200);
        assert_eq!(totals.shipping_cents, 0);
        assert_eq!(totals.total_cents, 11_880);
    }

    #[test]
    fn freeship_only_waives_supported_regions() {
        let jp = compute_order(4_000, 1, "jp", "FREESHIP").unwrap();
        let intl = compute_order(4_000, 1, "intl", "FREESHIP").unwrap();
        assert_eq!(jp.shipping_cents, 0);
        assert_eq!(intl.shipping_cents, 2_400);
    }

    #[test]
    fn invalid_region_returns_error() {
        let error = compute_order(3_000, 1, "moon", "").unwrap_err();
        assert_eq!(error, "unsupported region code: moon");
    }
}
