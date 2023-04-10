// test
#[cfg(test)]
mod tests {
    use crate::analyzer::calculate_growth_rate;

    #[test]
    fn growth_rate_test() {
        let input = vec![1, 2, 3, 4, 5];
        let result = calculate_growth_rate(input);
        assert_eq!(result, 1.25);
    }
}


mod analyzer {

    pub fn calculate_intrinsic_value() {
        
    }

    pub fn calculate_growth_rate(cash_flow_list: Vec<i32>) -> f32 {

        let mut all_growth_rates: Vec<i32> = Vec::new();

        // let next_years_cashflow = 0;
        let mut index = 1;

        for cashflow in &cash_flow_list {
            let current_cashflow = cashflow;
            println!("{}", &index);
            
            if &cash_flow_list.len() > &index {
                let next_years_cashflow = cash_flow_list[index];
                let growth_rate = ((next_years_cashflow - current_cashflow) / current_cashflow.abs()) + 1;
    
                all_growth_rates.push(growth_rate);
                index += 1;
            }
        }

        let avg_growth_rate: f32 = (all_growth_rates.iter().sum::<i32>() as f32) / all_growth_rates.len() as f32;
        return avg_growth_rate;
    }

}