

pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
    let mut shift = 0;
    while shift < purchase_amount {
        shift += 10;
    }
    100 - if shift - purchase_amount <= 5 { shift } else { shift - 10 }
}