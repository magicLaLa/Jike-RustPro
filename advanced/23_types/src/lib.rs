use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Identifier<T> {
    inner: u64,
    _tag: PhantomData<T>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    id: Identifier<Self>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Product {
    id: Identifier<Self>,
}

/// 第二部分

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

impl Personal for Customer<PersionlPlan> {
    fn advance_feature(&self) {
        println!(
            "Dear {}(as our valuable customer {}), enjoy this advanced feature!",
            self.name, self.id,
        )
    }
}

pub struct FreePlan;
pub struct PersionlPlan(f32);

impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersionlPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

/// 订阅成为付费用户
pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersionlPlan> {
    let _plan = PersionlPlan(payment);
    customer.into()
}

/// 第三部分

#[derive(Debug, Default)]
pub struct Equation<IterMethod> {
    current: u32,
    _method: PhantomData<IterMethod>,
}

#[derive(Debug, Default)]
pub struct Linear;

#[derive(Debug, Default)]
pub struct Quadratic;

impl Iterator for Equation<Linear> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u16::MAX as u32 {
            return None;
        }
        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None;
        }
        Some(self.current * self.current)
    }
}

///第四部分


pub fn comsume_iterator<F, Iter, T>(mut f: F)
where
    F: FnMut(i32) -> Iter,
    Iter: Iterator<Item = T>,
    T: std::fmt::Debug,
{
    for item in f(10) {
        println!("{:?}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_should_not_be_the_same() {
        let user = User::default();
        let procut = Product::default();

        // assert_ne!(user.id, procut.id);

        assert_eq!(user.id.inner, procut.id.inner);
    }

    #[test]
    fn test_customer() {
        let customer = Customer::<FreePlan>::new("Try".into());
        customer.feature1();
        customer.feature2();
        let customer = subscribe(customer, 6.99);
        customer.feature1();
        customer.feature2();
        customer.advance_feature();
    }

    #[test]
    fn test_linear() {
        let mut equation = Equation::<Linear>::default();
        assert_eq!(Some(1), equation.next());
        assert_eq!(Some(2), equation.next());
        assert_eq!(Some(3), equation.next());
    }
    #[test]
    fn test_quadratic() {
        let mut equation = Equation::<Quadratic>::default();
        assert_eq!(Some(1), equation.next());
        assert_eq!(Some(4), equation.next());
        assert_eq!(Some(9), equation.next());
    }

    #[test]
    fn test_comsume_iterator() {
        comsume_iterator(|i| (0..i).into_iter());
    }
}
