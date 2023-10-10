pub fn billion_users_days(growth_rates: &[f32]) -> i32 {
    let max_users: i32 = 10_i32.pow(9);
    let mut computed_days: i32 = 0;
    let mut users: i32 = 0;

    while users <= max_users {
        users = 0;
        computed_days += 1;
        for i in growth_rates {
            users = i.powi(computed_days) as i32;
        }
    }

    return computed_days;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn billion_users_test() {
        let growth_rates: [f32; 3] = [1.1, 1.2, 1.3];
        assert_eq!(billion_users_days(&growth_rates), 79);
    }

    #[test]
    fn billion_users_test_1() {
        let growth_rates: [f32; 2] = [1.1, 1.2];
        assert_eq!(billion_users_days(&growth_rates), 114);
    }
}
