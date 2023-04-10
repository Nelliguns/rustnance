// test
#[cfg(test)]
mod tests {
    use crate::value::calculate_intrinsic_value;

    #[test]
    fn growth_rate_test() {
        let input = vec![1, 2, 3, 4, 5];
        let result = calculate_intrinsic_value(input, 1.0);
        assert_eq!(result, 5.0);
    }
}


mod value {

    fn calculate_growth_rate(cash_flow_list: &Vec<i32>) -> f32 {
        
        let mut all_growth_rates: Vec<i32> = Vec::new();

        // let next_years_cashflow = 0;
        let mut index = 1;

        for cashflow in cash_flow_list {
            let current_cashflow = cashflow;
            
            if &cash_flow_list.len() > &index {
                let next_years_cashflow = cash_flow_list[index];
                let growth_rate = ((next_years_cashflow - current_cashflow) / current_cashflow.abs()) + 1;
    
                all_growth_rates.push(growth_rate);
                index += 1;
            }
        }

        let avg_growth_rate: f32 = (all_growth_rates.iter().sum::<i32>() as f32) / all_growth_rates.len() as f32;
        println!("avg growth: {}", avg_growth_rate);
        return avg_growth_rate;
    }

    fn calculate_future_cashflow(cash_flow_list: &Vec<i32>) -> Vec<f32> {
        // Should create a new list based on the growth rate and the last item of cash flow list
        let growth_rate = calculate_growth_rate(cash_flow_list);
        
        let mut future_cashflow: Vec<f32> = Vec::new();
        for &cashflow in cash_flow_list {
            
            future_cashflow.push(cashflow as f32 * growth_rate)
        }
        println!("Future cashflow: {:?}", future_cashflow);
        return future_cashflow
    }

    pub fn calculate_intrinsic_value(cash_flow_list: Vec<i32>, r: f32) -> f32 {
    
        let growth_rate = calculate_growth_rate(&cash_flow_list);
        let future_cash_flow_list = calculate_future_cashflow(&cash_flow_list);
    
        let mut discounted_cashflow = 0.0;
        let mut n = 0;
        
        for cashflow in &future_cash_flow_list {
            n += 1;
            discounted_cashflow += cashflow / (1.0 + &r).powf(n as f32);
        }
        println!("Discounted cashflow: {}", discounted_cashflow);
        println!("Final year cashflow: {}", (future_cash_flow_list[future_cash_flow_list.len() - 1]));
        println!("Discount rate - growth rate: {}", (&r - growth_rate));
        
        //                                              [Final Year FCF * (1 + Perpetuity Growth Rate)] ÷ (Discount Rate – Perpetuity Growth Rate)
        let intrisic_value = discounted_cashflow + (future_cash_flow_list[future_cash_flow_list.len() - 1] * (1.0 + growth_rate)) / (&r - growth_rate);
        
        println!("This is added to discounted cashflow: {}", ((future_cash_flow_list[future_cash_flow_list.len() - 1] * (1.0 + growth_rate)) / (&r - growth_rate)));

        return intrisic_value
    }
    
    pub fn intrinsic_value_per_stock(intrinsic_value: f32, outstanding_shares: u32) -> f32 {
        return intrinsic_value / outstanding_shares as f32
    }
    
}