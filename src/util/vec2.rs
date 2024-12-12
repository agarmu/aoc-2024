use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use itertools::iproduct;

pub trait Vec2Item = Copy
    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + Ord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vec2<T>
where
    T: Vec2Item,
{
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Vec2Item,
{
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T, U> Mul<T> for Vec2<U>
where
    T: Into<U>,
    U: Vec2Item,
{
    type Output = Self;
    fn mul(self, t: T) -> Self {
        let t = t.into();
        Self {
            x: self.x * t,
            y: self.y * t,
        }
    }
}

impl<T, U> MulAssign<T> for Vec2<U>
where
    T: Into<U>,
    U: Vec2Item,
{
    fn mul_assign(&mut self, rhs: T) {
        let t = rhs.into();
        self.x = self.x * t;
        self.y = self.y * t;
    }
}

impl<T, U> Div<T> for Vec2<U>
where
    T: Into<U>,
    U: Vec2Item,
{
    type Output = Self;
    fn div(self, t: T) -> Self {
        let t = t.into();
        Self {
            x: self.x / t,
            y: self.y / t,
        }
    }
}

impl<T, U> DivAssign<T> for Vec2<U>
where
    T: Into<U>,
    U: Vec2Item,
{
    fn div_assign(&mut self, rhs: T) {
        let t = rhs.into();
        self.x = self.x / t;
        self.y = self.y / t;
    }
}

impl<T> Add for Vec2<T>
where
    T: Vec2Item,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vec2<T>
where
    T: Vec2Item,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T> Sub for Vec2<T>
where
    T: Vec2Item,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign for Vec2<T>
where
    T: Vec2Item,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T> Neg for Vec2<T>
where
    T: Vec2Item + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub trait Access<T>
where
    T: Vec2Item,
{
    type Inner: Clone;
    fn access(&self, v: Vec2<T>) -> Self::Inner;
    fn try_access(&self, v: Vec2<T>) -> Option<Self::Inner>;
    fn mut_access(&mut self, v: Vec2<T>) -> &mut Self::Inner;
    fn try_mut_access(&mut self, v: Vec2<T>) -> Option<&mut Self::Inner>;
}

macro_rules! implement_into_from {
    ($type_name:ty) => {

        impl Vec2<$type_name> {
            pub fn cover<U>(data: &[Vec<U>]) -> impl Iterator<Item = Vec2<$type_name>> {
            let l = data.len();
            let sublens = data.iter().map(|x| x.len()).collect::<Vec<_>>();
            Iterator::zip(0..l, sublens)
                .flat_map(|(x, ymax)| iproduct!(x..=x, 0..ymax))
                .map(|(x, y)| Vec2 {
                    x: x as $type_name,
                    y: y as $type_name,
                })
            }
            pub  const ZZ: Vec2<$type_name> = Vec2 { x: 0, y: 0 };
        }

        impl<U> Access<$type_name> for [Vec<U>]
        where
            U: Clone,
        {
            type Inner = U;
            fn access(&self, v: Vec2<$type_name>) -> Self::Inner {
                self[v.y as usize][v.x as usize].clone()
            }
            fn try_access(&self, v: Vec2<$type_name>) -> Option<Self::Inner> {
                Some(self.get(v.y as usize)?.get(v.x as usize)?.clone())
            }
            fn mut_access(&mut self, v: Vec2<$type_name>) -> &mut Self::Inner {
                &mut self[v.y as usize][v.x as usize]
            }
            fn try_mut_access(&mut self, v: Vec2<$type_name>) -> Option<&mut Self::Inner> {
                self.get_mut(v.y as usize)?.get_mut(v.x as usize)
            }

        }



    };
    ($t1:ty, $($t2:ty),+) => {
        implement_into_from! { $t1 }
        implement_into_from! { $($t2),+ }
    }
}

macro_rules! implement_dirs {
    ($type_name:ty) => {
        impl Vec2<$type_name> {
            pub const NE: Vec2<$type_name> = Vec2 { x: -1, y: -1 };
            pub const SE: Vec2<$type_name> = Vec2 { x: -1, y: 1 };
            pub const NW: Vec2<$type_name> = Vec2 { x: 1, y: -1 };
            pub const SW: Vec2<$type_name> = Vec2 { x: 1, y: 1 };
            pub const N: Vec2<$type_name> = Vec2 { x: 0, y: -1 };
            pub const S: Vec2<$type_name> = Vec2 { x: 0, y: 1 };
            pub const E: Vec2<$type_name> = Vec2 { x: 1, y: 0 };
            pub const W: Vec2<$type_name> = Vec2 { x: -1, y: 0 };

            pub const MOORE: [Vec2<$type_name>; 8] = [
                Self::NE,
                Self::NW,
                Self::SE,
                Self::SW,
                Self::N,
                Self::E,
                Self::S,
                Self::W
            ];
        }
    };
    ($t1:ty, $($t2:ty),+) => {
        implement_dirs! { $t1 }
        implement_dirs! { $($t2),+ }
    }
}

implement_into_from! { i32, i64, i128, u32, u64, u128, isize, usize}
implement_dirs! { i8, i16, i32, i64, i128 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_access() {
        let q = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let p: &[Vec<_>] = &q;
        assert_eq!(p.access(Vec2 { x: 0, y: 0 }), 1);
        assert_eq!(p.access(Vec2 { x: 1, y: 0 }), 2);
        assert_eq!(p.access(Vec2 { x: 2, y: 0 }), 3);
        assert_eq!(p.access(Vec2 { x: 0, y: 1 }), 4);
        assert_eq!(p.access(Vec2 { x: 1, y: 1 }), 5);
        assert_eq!(p.access(Vec2 { x: 2, y: 1 }), 6);
        assert_eq!(p.access(Vec2 { x: 0, y: 2 }), 7);
        assert_eq!(p.access(Vec2 { x: 1, y: 2 }), 8);
        assert_eq!(p.access(Vec2 { x: 2, y: 2 }), 9);
    }

    #[test]
    fn test_cover() {
        let q = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let p: &[Vec<_>] = &q;
        for c in Vec2::<i64>::cover(&q) {
            assert_eq!(p.access(c), q[c.y as usize][c.x as usize]);
        }
    }
}
