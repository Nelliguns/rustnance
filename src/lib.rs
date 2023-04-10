// test
#[cfg(test)]
mod tests {
    use crate::value::calculate_intrinsic_value;

    #[test]
    fn growth_rate_test() {
        let input = vec![1.0, 1.1, 1.2, 1.3, 1.4];
        let result = calculate_intrinsic_value(input, 0.08);
        assert_eq!(result, 5.0);
    }
}


mod value {

    fn calculate_growth_rate(cash_flow_list: &Vec<f32>) -> f32 {
        
        let mut all_growth_rates: Vec<f32> = Vec::new();

        // let next_years_cashflow = 0;
        let mut index = 1;

        for cashflow in cash_flow_list {
            let current_cashflow = cashflow;
            
            if &cash_flow_list.len() > &index {
                let next_years_cashflow = cash_flow_list[index];
                let growth_rate = ((next_years_cashflow - current_cashflow) / current_cashflow.abs()) + 1.0;
    
                all_growth_rates.push(growth_rate);
                index += 1;
            }
        }

        let avg_growth_rate: f32 = (all_growth_rates.iter().sum::<f32>() as f32) / all_growth_rates.len() as f32;
        println!("avg growth: {}", avg_growth_rate);
        return avg_growth_rate;
    }

    fn calculate_future_cashflow(cash_flow_list: &Vec<f32>) -> Vec<f32> {
        // Should create a new list based on the growth rate and the last item of cash flow list
        let growth_rate = calculate_growth_rate(cash_flow_list);
        
        let mut future_cashflow_list: Vec<f32> = Vec::new();
        let mut cashflow: f32 = cash_flow_list[cash_flow_list.len() - 1] as f32;
        for i in 0..5 {
            println!("This is i: {}", i);
            
            future_cashflow_list.push(cashflow * growth_rate);
            cashflow *= growth_rate;
        }
        println!("Future cashflow: {:?}", future_cashflow_list);
        println!("Length of f cashflow list: {}", future_cashflow_list.len());
        return future_cashflow_list
    }

    pub fn calculate_intrinsic_value(cash_flow_list: Vec<f32>, r: f32) -> f32 {
    
        let growth_rate = calculate_growth_rate(&cash_flow_list);
        let future_cash_flow_list = calculate_future_cashflow(&cash_flow_list);
    
        let mut discounted_cashflow = 0.0;
        let mut discounted_cashflow_list: Vec<f32> = Vec::new();
        let mut n = 0;
        
        for cashflow in &future_cash_flow_list {
            n += 1;
            discounted_cashflow = cashflow / (1.0 + &r).powf(n as f32); // have another look to make sure this is correct
            discounted_cashflow_list.push(discounted_cashflow);
        }
        println!("Discounted cashflow: {:?}", discounted_cashflow_list);
        println!("Final year cashflow: {}", (future_cash_flow_list[future_cash_flow_list.len() - 1]));
        println!("Discount rate - growth rate: {}", (&r - growth_rate));
        
        //                                              [Final Year FCF * (1 + Perpetuity Growth Rate)] ÷ (Discount Rate – Perpetuity Growth Rate)
        // Have another look to make sure these calculations are good.
        let mut total_free_cashflow = 0.0;
        for cashflow in &discounted_cashflow_list {
            total_free_cashflow += cashflow;
        }
        println!("Total free cash flow: {}", total_free_cashflow);

        let intrisic_value = total_free_cashflow + (discounted_cashflow_list[discounted_cashflow_list.len() - 1]) * 10.0;
        
        println!("This is added to discounted cashflow: {}", ((discounted_cashflow_list[discounted_cashflow_list.len() - 1]) * 10.0));

        return intrisic_value
    }
    
    pub fn intrinsic_value_per_stock(intrinsic_value: f32, outstanding_shares: u32) -> f32 {
        return intrinsic_value / outstanding_shares as f32
    }
    
}