
mod analyzer {

    pub fn calculate_intrinsic_value() {
        
    }

    fn calculate_growth_rate(cash_flow_list: Vec<i32>) -> f32 {

        let mut all_growth_rates: Vec<i32> = Vec::new();

        // let next_years_cashflow = 0;
        let mut index = 0;

        for cashflow in &cash_flow_list {
            let current_cashflow = cashflow;
            let next_years_cashflow = cash_flow_list[index + 1];
            let growth_rate = ((next_years_cashflow - current_cashflow) / current_cashflow.abs()) + 1;

            all_growth_rates.push(growth_rate);

            if &cash_flow_list.len() > &index {
                index += 1;
            }
        }

        let avg_growth_rate: f32 = (all_growth_rates.iter().sum::<i32>() as f32) / all_growth_rates.len() as f32;
        return avg_growth_rate;
    }

}