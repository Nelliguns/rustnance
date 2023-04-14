
/// Returned a tuple of the total accumalated amount and how much of that which is interest: (total_amount, total_accumalated_interest)
/// 
/// # Arguments
/// 
/// * 'starting_value' - A flat amount from where the calculation starts
/// * 'monthly_payment' - A monthly amount that will be added to the amount that accrues interest
/// * 'yearly_interest' - The rate at which the investment will increase 8% -> 0.08
/// * 'amount_of_years' - The amount of years in which the amount will grow
/// 
pub fn calculate_compounded_interest(starting_amount: f64, monthly_payment: f64, yearly_interest: f64, amount_of_years: i32) -> (f64, f64) {
    
    
    let monthly_interest = (yearly_interest / 12.0) + 1.0;
    let amount_of_months = amount_of_years * 12;
    let mut total_amount: f64 = 0.0;
    let mut total_accumalated_interest: f64 = 0.0;
    
    total_amount += starting_amount;
    
    for _ in 1..amount_of_months {
        // need to be the interest and monthly payments are added to the total amount
        total_accumalated_interest = total_accumalated_interest + ((total_amount + monthly_payment) * monthly_interest) - (total_amount + monthly_payment);
        total_amount = (total_amount + monthly_payment) * monthly_interest;
    };
    
    return (total_amount, total_accumalated_interest);
}