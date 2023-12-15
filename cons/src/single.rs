//! Machinery for handling cons cells

pub type Single<T> = (T,);

/// A single value
pub trait ConsSingle {
    /// Value (__Contents__ _of the_ __Address__ _part of_ __Register__)
    type CAR;

    /// Fetch value by reference
    fn car(&self) -> &Self::CAR;

    fn car_mut(&mut self) -> &mut Self::CAR;

    /// Fetch value
    fn into_car(self) -> Self::CAR;
}

impl<CAR> ConsSingle for (CAR,) {
    type CAR = CAR;

    fn car(&self) -> &Self::CAR {
        &self.0
    }

    fn car_mut(&mut self) -> &mut Self::CAR {
        &mut self.0
    }

    fn into_car(self) -> Self::CAR {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cons_single() {
        let mut cons_single = (1,);
        let _proof: &dyn ConsSingle<CAR = _> = &cons_single;
        let _car = cons_single.car();
        let _car = cons_single.car_mut();
        let _car = cons_single.into_car();
    }
}
