use crate::value::{calculate_intrinsic_value, margin_of_safety};

#[test]
fn growth_rate_test() {
    let cashflow = vec![14641.0, 16105.0, 17716.0, 19487.0, 21436.00];
    let result = calculate_intrinsic_value(&cashflow, &0.15);
    assert_eq!(result, 306674.7);
}
#[test]
fn margin_of_safety_test() {
    let value = 10000.0;
    let safety_margin = 0.3;
    let result = margin_of_safety(&value, &safety_margin);
    assert_eq!(result, 7000.0);
}

use crate::utilities::calculate_compounded_interest;

#[test]
fn calculate_compounded_interest_test() {
    let starting_amount = 10000.0;
    let monthly_payment = 2000.0;
    let yearly_interest = 0.07;
    let amount_of_years = 15;
    let result = calculate_compounded_interest(
        &starting_amount,
        &monthly_payment,
        &yearly_interest,
        &amount_of_years,
    );

    assert_eq!(result, (660248.8360031071, 292248.83600310725));
}
