use ta_common::traits::Indicator;

pub struct TypPrice {}

impl TypPrice {
    pub fn new() -> TypPrice {
        Self {}
    }
}


impl Indicator<[f64; 3], f64> for TypPrice {
    fn next(&mut self, input: [f64; 3]) -> f64 {
        return (input[0] + input[1] + input[2]) / 3_f64;
    }

    fn reset(&mut self) {
        ()
    }
}

#[cfg(test)]
mod tests {
    use crate::TypPrice;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut typprice = TypPrice::new();
        assert_eq!(typprice.next([82.15, 81.29, 81.59]), 81.67666666666666);
        assert_eq!(typprice.next([81.89, 80.64, 81.06]), 81.19666666666667);
        assert_eq!(typprice.next([83.03, 81.31, 82.87]), 82.40333333333334);
        assert_eq!(typprice.next([83.30, 82.65, 83.00]), 82.98333333333333);
        assert_eq!(typprice.next([83.85, 83.07, 83.61]), 83.50999999999999);
        assert_eq!(typprice.next([83.90, 83.11, 83.15]), 83.38666666666667);
        assert_eq!(typprice.next([83.33, 82.49, 82.84]), 82.88666666666667);
        assert_eq!(typprice.next([84.30, 82.30, 83.99]), 83.52999999999999);
        assert_eq!(typprice.next([84.84, 84.15, 84.55]), 84.51333333333334);
        assert_eq!(typprice.next([85.00, 84.11, 84.36]), 84.49000000000001);
        assert_eq!(typprice.next([85.90, 84.03, 85.53]), 85.15333333333334);
        assert_eq!(typprice.next([86.58, 85.39, 86.54]), 86.17);
        assert_eq!(typprice.next([86.98, 85.76, 86.89]), 86.54333333333334);
        assert_eq!(typprice.next([88.00, 87.17, 87.77]), 87.64666666666666);
        assert_eq!(typprice.next([87.87, 87.01, 87.29]), 87.39);
    }
}
