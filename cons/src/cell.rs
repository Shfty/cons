//! Machinery for handling cons cells

pub type Cons<CAR, CDR> = (CAR, CDR);

/// A pair of values
pub trait ConsCell {
    /// Left-hand value (__Contents__ _of the_ __Address__ _part of_ __Register__)
    type CAR;

    /// Right-hand value: (__Contents__ _of the_ __Decrement__ _part of_ __Register__)
    type CDR;

    fn car(&self) -> &Self::CAR;
    fn cdr(&self) -> &Self::CDR;

    fn car_mut(&mut self) -> &mut Self::CAR;
    fn cdr_mut(&mut self) -> &mut Self::CDR;

    fn into_car(self) -> Self::CAR;
    fn into_cdr(self) -> Self::CDR;

    fn destructure(&self) -> (&Self::CAR, &Self::CDR);
    fn destructure_mut(&mut self) -> (&mut Self::CAR, &mut Self::CDR);
    fn into_destructure(self) -> (Self::CAR, Self::CDR);
}

impl<CAR, CDR> ConsCell for Cons<CAR, CDR> {
    type CAR = CAR;
    type CDR = CDR;

    fn car(&self) -> &Self::CAR {
        &self.0
    }

    fn cdr(&self) -> &Self::CDR {
        &self.1
    }

    fn car_mut(&mut self) -> &mut Self::CAR {
        &mut self.0
    }

    fn cdr_mut(&mut self) -> &mut Self::CDR {
        &mut self.1
    }

    fn into_car(self) -> Self::CAR {
        self.0
    }

    fn into_cdr(self) -> Self::CDR {
        self.1
    }

    fn destructure(&self) -> (&Self::CAR, &Self::CDR) {
        (&self.0, &self.1)
    }

    fn destructure_mut(&mut self) -> (&mut Self::CAR, &mut Self::CDR) {
        (&mut self.0, &mut self.1)
    }

    fn into_destructure(self) -> (Self::CAR, Self::CDR) {
        (self.0, self.1)
    }
}

impl<CAR, CDR> ConsCell for &Cons<CAR, CDR> {
    type CAR = CAR;
    type CDR = CDR;

    fn car(&self) -> &Self::CAR {
        &self.0
    }

    fn cdr(&self) -> &Self::CDR {
        &self.1
    }

    fn car_mut(&mut self) -> &mut Self::CAR {
        panic!()
    }

    fn cdr_mut(&mut self) -> &mut Self::CDR {
        panic!()
    }

    fn into_car(self) -> Self::CAR {
        panic!()
    }

    fn into_cdr(self) -> Self::CDR {
        panic!()
    }

    fn destructure(&self) -> (&Self::CAR, &Self::CDR) {
        (&self.0, &self.1)
    }

    fn destructure_mut(&mut self) -> (&mut Self::CAR, &mut Self::CDR) {
        panic!()
    }

    fn into_destructure(self) -> (Self::CAR, Self::CDR) {
        panic!()
    }
}

impl<CAR, CDR> ConsCell for &mut Cons<CAR, CDR> {
    type CAR = CAR;
    type CDR = CDR;

    fn car(&self) -> &Self::CAR {
        &self.0
    }

    fn cdr(&self) -> &Self::CDR {
        &self.1
    }

    fn car_mut(&mut self) -> &mut Self::CAR {
        &mut self.0
    }

    fn cdr_mut(&mut self) -> &mut Self::CDR {
        &mut self.1
    }

    fn into_car(self) -> Self::CAR {
        panic!()
    }

    fn into_cdr(self) -> Self::CDR {
        panic!()
    }

    fn destructure(&self) -> (&Self::CAR, &Self::CDR) {
        (&self.0, &self.1)
    }

    fn destructure_mut(&mut self) -> (&mut Self::CAR, &mut Self::CDR) {
        (&mut self.0, &mut self.1)
    }

    fn into_destructure(self) -> (Self::CAR, Self::CDR) {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cons_cell() {
        let cons_cell = (1, 2.0);
        let _proof: &dyn ConsCell<CAR = _, CDR = _> = &cons_cell;
        let _car = cons_cell.car();
        let _cdr = cons_cell.cdr();
    }
}
