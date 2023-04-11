// test
#[cfg(test)]
mod tests {
    use crate::value::{calculate_intrinsic_value, margin_of_safety};

    #[test]
    fn growth_rate_test() {
        let cashflow = vec![14641.0, 16105.0, 17716.0, 19487.0, 21436.00];
        let result = calculate_intrinsic_value(cashflow, 0.15);
        assert_eq!(result, 306674.7);
    }
    #[test]
    fn margin_of_safety_test() {
        let value = 10000.0;
        let safety_margin = 0.3;
        let result = margin_of_safety(value, safety_margin);
        assert_eq!(result, 7000.0);
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
        return avg_growth_rate;
    }

    fn calculate_future_cashflow(cash_flow_list: &Vec<f32>) -> Vec<f32> {
        let growth_rate = calculate_growth_rate(cash_flow_list);
        
        let mut future_cashflow_list: Vec<f32> = Vec::new();
        let mut cashflow: f32 = cash_flow_list[cash_flow_list.len() - 1] as f32;
        for _ in 0..10 {
            future_cashflow_list.push(cashflow * growth_rate);
            cashflow *= growth_rate;
        }
        return future_cashflow_list
    }

    pub fn calculate_intrinsic_value(cash_flow_list: Vec<f32>, r: f32) -> f32 {
    
        let future_cash_flow_list = calculate_future_cashflow(&cash_flow_list);
    
        let mut discounted_cashflow = 0.0;
        let mut discounted_cashflow_list: Vec<f32> = Vec::new();
        let mut total_free_cashflow = 0.0;
        
        let mut n = 0;
        for cashflow in &future_cash_flow_list {
            n += 1;
            discounted_cashflow = cashflow / (1.0 + &r).powf(n as f32); // have another look to make sure this is correct
            discounted_cashflow_list.push(discounted_cashflow);
        }
        for cashflow in &discounted_cashflow_list {
            total_free_cashflow += cashflow;
        }

        let intrisic_value = total_free_cashflow + (discounted_cashflow_list[discounted_cashflow_list.len() - 1]) * 10.0;

        return intrisic_value
    }

    pub fn margin_of_safety(intrinsic_value: f32, safety_margin: f32) -> f32 {
        intrinsic_value * (1.0 - safety_margin)
    }
    
    pub fn intrinsic_value_per_stock(intrinsic_value: f32, outstanding_shares: u32) -> f32 {
        return intrinsic_value / outstanding_shares as f32
    }
    
}