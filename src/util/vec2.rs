use std::ops::{Add, AddAssign, Mul, MulAssign};

use itertools::iproduct;
use num::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T>
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    pub x: T,
    pub y: T,
}

impl<T, U> Mul<T> for Vec2<U>
where
    T: Into<U>,
    U: Copy + Add<U, Output = U> + Mul<U, Output = U>,
{
    type Output = Vec2<U>;
    fn mul(self, t: T) -> Self {
        let t = t.into();
        Self {
            x: self.x * t,
            y: self.y * t,
        }
    }
}

impl<T> Add for Vec2<T>
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
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
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T, U> MulAssign<T> for Vec2<U>
where
    T: Into<U>,
    U: Copy + Add<U, Output = U> + Mul<U, Output = U>,
{
    fn mul_assign(&mut self, rhs: T) {
        let t = rhs.into();
        self.x = self.x * t;
        self.y = self.y * t;
    }
}

pub trait IntoUsize: Copy {
    fn into_usize(self) -> usize;
}

pub trait FromUsize: Copy {
    fn from_usize(x: usize) -> Self;
}

macro_rules! implement_into_from {
    ($type_name:ty) => {
        impl IntoUsize for $type_name {
            fn into_usize(self) -> usize {
                self as usize
            }
        }
        impl FromUsize for $type_name {
            fn from_usize(x: usize) -> $type_name {
                x as $type_name
            }
        }
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
            pub const E: Vec2<$type_name> = Vec2 { x: -1, y: 0 };
            pub const W: Vec2<$type_name> = Vec2 { x: 1, y: 0 };

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

pub trait Access<T>
where
    T: IntoUsize + Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    type Inner: Clone;
    fn access(&self, v: Vec2<T>) -> Self::Inner;
    fn try_access(&self, v: Vec2<T>) -> Option<Self::Inner>;
}

impl<T, U> Access<T> for &[Vec<U>]
where
    T: IntoUsize + Copy + Add<T, Output = T> + Mul<T, Output = T>,
    U: Clone,
{
    type Inner = U;
    fn access(&self, v: Vec2<T>) -> Self::Inner {
        self[v.x.into_usize()][v.y.into_usize()].clone()
    }
    fn try_access(&self, v: Vec2<T>) -> Option<Self::Inner> {
        Some(self.get(v.x.into_usize())?.get(v.y.into_usize())?.clone())
    }
}
