//! Finance tests.

#[cfg(test)]
mod tests {
    use super::*;
    use cm_core::economy::Money;
    use crate::ticketing::calculate_matchday_revenue;

    #[test]
    fn test_matchday_revenue() {
        let revenue = calculate_matchday_revenue(40_000, Money::from_major(30));
        assert_eq!(revenue.major(), 1_200_000);
    }
}
