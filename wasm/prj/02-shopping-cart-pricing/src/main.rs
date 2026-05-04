use shopping_cart_pricing::{calculate_total_cents, order_summary};

fn main() {
    let subtotal_cents = 8_400;
    let item_count = 4;
    let region_code = "us-ca";
    let coupon_code = "SAVE10";

    println!("Order summary");
    println!(
        "{}",
        order_summary(subtotal_cents, item_count, region_code, coupon_code).unwrap()
    );
    println!(
        "computed total cents: {}",
        calculate_total_cents(subtotal_cents, item_count, region_code, coupon_code).unwrap()
    );
}
