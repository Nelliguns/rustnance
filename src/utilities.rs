/// Returnes a tuple of the total accumalated amount and how much of that which is interest: (total_amount, total_accumalated_interest)
///
/// # Arguments
///
/// * 'starting_value' - A flat amount from where the calculation starts
/// * 'monthly_payment' - A monthly amount that will be added to the amount that accrues interest
/// * 'yearly_interest' - The rate at which the investment will increase 8% -> 0.08
/// * 'amount_of_years' - The amount of years in which the amount will grow
///
pub fn calculate_compounded_interest(
    starting_amount: &f64,
    monthly_payment: &f64,
    yearly_interest: &f64,
    amount_of_years: &i32,
) -> (f64, f64) {
    let monthly_interest = (yearly_interest / 12.0) + 1.0;
    let amount_of_months = amount_of_years * 12;
    let mut total_amount: f64 = 0.0;
    let mut total_accumalated_interest: f64 = 0.0;

    total_amount += starting_amount;

    for _ in 1..amount_of_months {
        // need to be the interest and monthly payments are added to the total amount
        total_accumalated_interest = total_accumalated_interest
            + ((total_amount + monthly_payment) * monthly_interest)
            - (total_amount + monthly_payment);
        total_amount = (total_amount + monthly_payment) * monthly_interest;
    }

    return (total_amount, total_accumalated_interest);
}

/// Returnes the total amount paid for a loan given a payment plan.
///
/// # Arguments
///
/// * 'loaned_amount' - The amount that has been loaned
/// * 'interest_rate' - The percentual interest rate
/// * 'amortization' - The percentual rate that the loan will be paid off at, example 2.0 
///
pub fn cost_from_payment_plan(loaned_amount: &f64, interest_rate: &f64, amortization_rate: f64) -> f64 {
    
    let mut amount_paid: f64 = 0.0;
    let mut current_loan = loaned_amount.copy();
    let interest_rate = interest_rate;
    let amortization = (amortization_rate * loaned_amount) - loaned_amount;
    let monthly_amortization = amortization / 12.0;
    let years = 100.0 / amortization;
    let months = years * 12.0;
    let monthly_interest_rate = interest_rate / 12.0;

    for _i in 0..months as i32 {
        
        let monthly_payment = ((current_loan * monthly_interest_rate) - current_loan) + monthly_amortization;
        amount_paid += monthly_payment;
        current_loan -= monthly_amortization;
    }

    amount_paid
    
}
