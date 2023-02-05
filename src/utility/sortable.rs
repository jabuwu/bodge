use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Sortable<S: PartialEq + PartialOrd + SortableCheck>(pub S);

impl<S: PartialOrd + PartialEq + SortableCheck> PartialEq for Sortable<S> {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.0.sortable_check());
        self.0.eq(&other.0)
    }
}

impl<S: PartialOrd + PartialEq + SortableCheck> Eq for Sortable<S> {}

impl<S: PartialOrd + PartialEq + SortableCheck> PartialOrd for Sortable<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert!(self.0.sortable_check());
        assert!(other.0.sortable_check());
        self.0.partial_cmp(&other.0)
    }
}

impl<S: PartialOrd + PartialEq + SortableCheck> Ord for Sortable<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SortableWith<S: PartialEq + PartialOrd + SortableCheck, T>(pub S, pub T);

impl<S: PartialEq + PartialOrd + SortableCheck, T> SortableWith<S, T> {
    pub fn data(&self) -> &T {
        &self.1
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<S: PartialOrd + PartialEq + SortableCheck, T> PartialEq for SortableWith<S, T> {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.0.sortable_check());
        self.0.eq(&other.0)
    }
}

impl<S: PartialOrd + PartialEq + SortableCheck, T> Eq for SortableWith<S, T> {}

impl<S: PartialOrd + PartialEq + SortableCheck, T> PartialOrd for SortableWith<S, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert!(self.0.sortable_check());
        assert!(other.0.sortable_check());
        self.0.partial_cmp(&other.0)
    }
}

impl<S: PartialOrd + PartialEq + SortableCheck, T> Ord for SortableWith<S, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sortable2D<
    S1: PartialEq + PartialOrd + SortableCheck,
    S2: PartialEq + PartialOrd + SortableCheck,
>(pub S1, pub S2);

impl<S1: PartialOrd + PartialEq + SortableCheck, S2: PartialOrd + PartialEq + SortableCheck>
    PartialEq for Sortable2D<S1, S2>
{
    fn eq(&self, other: &Self) -> bool {
        assert!(self.0.sortable_check());
        assert!(self.1.sortable_check());
        self.0.eq(&other.0) && self.1.eq(&other.1)
    }
}

impl<S1: PartialOrd + PartialEq + SortableCheck, S2: PartialOrd + PartialEq + SortableCheck> Eq
    for Sortable2D<S1, S2>
{
}

impl<S1: PartialOrd + PartialEq + SortableCheck, S2: PartialOrd + PartialEq + SortableCheck>
    PartialOrd for Sortable2D<S1, S2>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert!(self.0.sortable_check());
        assert!(other.0.sortable_check());
        assert!(self.1.sortable_check());
        assert!(other.1.sortable_check());
        let cmp = self.0.partial_cmp(&other.0).unwrap();
        if cmp == Ordering::Equal {
            self.1.partial_cmp(&other.1)
        } else {
            Some(cmp)
        }
    }
}

impl<S1: PartialOrd + PartialEq + SortableCheck, S2: PartialOrd + PartialEq + SortableCheck> Ord
    for Sortable2D<S1, S2>
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sortable2DWith<
    S1: PartialEq + PartialOrd + SortableCheck,
    S2: PartialEq + PartialOrd + SortableCheck,
    T,
>(pub S1, pub S2, pub T);

impl<S1: PartialEq + PartialOrd + SortableCheck, S2: PartialEq + PartialOrd + SortableCheck, T>
    Sortable2DWith<S1, S2, T>
{
    pub fn data(&self) -> &T {
        &self.2
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.2
    }
}

impl<S1: PartialOrd + PartialEq + SortableCheck, S2: PartialOrd + PartialEq + SortableCheck, T>
    PartialEq for Sortable2DWith<S1, S2, T>
{
    fn eq(&self, other: &Self) -> bool {
        assert!(self.0.sortable_check());
        assert!(self.1.sortable_check());
        self.0.eq(&other.0) && self.1.eq(&other.1)
    }
}

impl<S1: PartialOrd + PartialEq + SortableCheck, S2: PartialOrd + PartialEq + SortableCheck, T> Eq
    for Sortable2DWith<S1, S2, T>
{
}

impl<S1: PartialOrd + PartialEq + SortableCheck, S2: PartialOrd + PartialEq + SortableCheck, T>
    PartialOrd for Sortable2DWith<S1, S2, T>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert!(self.0.sortable_check());
        assert!(other.0.sortable_check());
        assert!(self.1.sortable_check());
        assert!(other.1.sortable_check());
        let cmp = self.0.partial_cmp(&other.0).unwrap();
        if cmp == Ordering::Equal {
            self.1.partial_cmp(&other.1)
        } else {
            Some(cmp)
        }
    }
}

impl<S1: PartialOrd + PartialEq + SortableCheck, S2: PartialOrd + PartialEq + SortableCheck, T> Ord
    for Sortable2DWith<S1, S2, T>
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sortable3D<
    S1: PartialEq + PartialOrd + SortableCheck,
    S2: PartialEq + PartialOrd + SortableCheck,
    S3: PartialEq + PartialOrd + SortableCheck,
>(pub S1, pub S2, pub S3);

impl<
        S1: PartialOrd + PartialEq + SortableCheck,
        S2: PartialOrd + PartialEq + SortableCheck,
        S3: PartialOrd + PartialEq + SortableCheck,
    > PartialEq for Sortable3D<S1, S2, S3>
{
    fn eq(&self, other: &Self) -> bool {
        assert!(self.0.sortable_check());
        assert!(self.1.sortable_check());
        assert!(self.2.sortable_check());
        self.0.eq(&other.0) && self.1.eq(&other.1) && self.2.eq(&other.2)
    }
}

impl<
        S1: PartialOrd + PartialEq + SortableCheck,
        S2: PartialOrd + PartialEq + SortableCheck,
        S3: PartialOrd + PartialEq + SortableCheck,
    > Eq for Sortable3D<S1, S2, S3>
{
}

impl<
        S1: PartialOrd + PartialEq + SortableCheck,
        S2: PartialOrd + PartialEq + SortableCheck,
        S3: PartialOrd + PartialEq + SortableCheck,
    > PartialOrd for Sortable3D<S1, S2, S3>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert!(self.0.sortable_check());
        assert!(other.0.sortable_check());
        assert!(self.1.sortable_check());
        assert!(other.1.sortable_check());
        assert!(self.2.sortable_check());
        assert!(other.2.sortable_check());
        let cmp = self.0.partial_cmp(&other.0).unwrap();
        if cmp == Ordering::Equal {
            let cmp = self.1.partial_cmp(&other.1).unwrap();
            if cmp == Ordering::Equal {
                self.2.partial_cmp(&other.2)
            } else {
                Some(cmp)
            }
        } else {
            Some(cmp)
        }
    }
}

impl<
        S1: PartialOrd + PartialEq + SortableCheck,
        S2: PartialOrd + PartialEq + SortableCheck,
        S3: PartialOrd + PartialEq + SortableCheck,
    > Ord for Sortable3D<S1, S2, S3>
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sortable3DWith<
    S1: PartialEq + PartialOrd + SortableCheck,
    S2: PartialEq + PartialOrd + SortableCheck,
    S3: PartialEq + PartialOrd + SortableCheck,
    T,
>(pub S1, pub S2, pub S3, pub T);

impl<
        S1: PartialEq + PartialOrd + SortableCheck,
        S2: PartialEq + PartialOrd + SortableCheck,
        S3: PartialEq + PartialOrd + SortableCheck,
        T,
    > Sortable3DWith<S1, S2, S3, T>
{
    pub fn data(&self) -> &T {
        &self.3
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.3
    }
}

impl<
        S1: PartialOrd + PartialEq + SortableCheck,
        S2: PartialOrd + PartialEq + SortableCheck,
        S3: PartialOrd + PartialEq + SortableCheck,
        T,
    > PartialEq for Sortable3DWith<S1, S2, S3, T>
{
    fn eq(&self, other: &Self) -> bool {
        assert!(self.0.sortable_check());
        assert!(self.1.sortable_check());
        assert!(self.2.sortable_check());
        self.0.eq(&other.0) && self.1.eq(&other.1) && self.2.eq(&other.2)
    }
}

impl<
        S1: PartialOrd + PartialEq + SortableCheck,
        S2: PartialOrd + PartialEq + SortableCheck,
        S3: PartialOrd + PartialEq + SortableCheck,
        T,
    > Eq for Sortable3DWith<S1, S2, S3, T>
{
}

impl<
        S1: PartialOrd + PartialEq + SortableCheck,
        S2: PartialOrd + PartialEq + SortableCheck,
        S3: PartialOrd + PartialEq + SortableCheck,
        T,
    > PartialOrd for Sortable3DWith<S1, S2, S3, T>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert!(self.0.sortable_check());
        assert!(other.0.sortable_check());
        assert!(self.1.sortable_check());
        assert!(other.1.sortable_check());
        assert!(self.2.sortable_check());
        assert!(other.2.sortable_check());
        let cmp = self.0.partial_cmp(&other.0).unwrap();
        if cmp == Ordering::Equal {
            let cmp = self.1.partial_cmp(&other.1).unwrap();
            if cmp == Ordering::Equal {
                self.2.partial_cmp(&other.2)
            } else {
                Some(cmp)
            }
        } else {
            Some(cmp)
        }
    }
}

impl<
        S1: PartialOrd + PartialEq + SortableCheck,
        S2: PartialOrd + PartialEq + SortableCheck,
        S3: PartialOrd + PartialEq + SortableCheck,
        T,
    > Ord for Sortable3DWith<S1, S2, S3, T>
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub trait SortableCheck {
    fn sortable_check(&self) -> bool {
        true
    }
}

impl SortableCheck for f32 {
    fn sortable_check(&self) -> bool {
        self.is_finite()
    }
}

impl SortableCheck for f64 {
    fn sortable_check(&self) -> bool {
        self.is_finite()
    }
}

impl SortableCheck for u8 {}
impl SortableCheck for i8 {}
impl SortableCheck for u16 {}
impl SortableCheck for i16 {}
impl SortableCheck for u32 {}
impl SortableCheck for i32 {}
impl SortableCheck for u64 {}
impl SortableCheck for i64 {}
impl SortableCheck for u128 {}
impl SortableCheck for i128 {}
impl SortableCheck for usize {}
impl SortableCheck for isize {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sortable() {
        assert!(Sortable(0.) == Sortable(0.));
        assert!(Sortable(0.) != Sortable(1.));

        let a = Sortable(0.);
        let b = Sortable(0.);
        assert!(a.eq(&b));

        let a = Sortable(1.);
        let b = Sortable(0.);
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable(0.);
        let b = Sortable(1.);
        assert!(a.cmp(&b) == Ordering::Less);
    }

    #[test]
    fn sortable_with() {
        assert!(SortableWith(0., ()) == SortableWith(0., ()));
        assert!(SortableWith(0., ()) != SortableWith(1., ()));

        let a = SortableWith(0., 123);
        let b = SortableWith(0., 321);
        assert!(a.eq(&b));
        assert!(*a.data() == 123);
        assert!(*b.data() == 321);

        let a = SortableWith(1., ());
        let b = SortableWith(0., ());
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = SortableWith(0., ());
        let b = SortableWith(1., ());
        assert!(a.cmp(&b) == Ordering::Less);
    }

    #[test]
    fn sortable2d() {
        assert!(Sortable2D(0., 1.) == Sortable2D(0., 1.));
        assert!(Sortable2D(0., 1.) != Sortable2D(1., 1.));
        assert!(Sortable2D(0., 1.) != Sortable2D(0., 2.));

        let a = Sortable2D(0., 0.);
        let b = Sortable2D(0., 0.);
        assert!(a.eq(&b));

        let a = Sortable2D(1., 0.);
        let b = Sortable2D(0., 0.);
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable2D(0., 0.);
        let b = Sortable2D(1., 0.);
        assert!(a.cmp(&b) == Ordering::Less);

        let a = Sortable2D(0., 1.);
        let b = Sortable2D(0., 0.);
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable2D(0., 0.);
        let b = Sortable2D(0., 1.);
        assert!(a.cmp(&b) == Ordering::Less);
    }

    #[test]
    fn sortable2d_with() {
        assert!(Sortable2DWith(0., 1., ()) == Sortable2DWith(0., 1., ()));
        assert!(Sortable2DWith(0., 1., ()) != Sortable2DWith(1., 1., ()));
        assert!(Sortable2DWith(0., 1., ()) != Sortable2DWith(0., 2., ()));

        let a = Sortable2DWith(0., 0., 123);
        let b = Sortable2DWith(0., 0., 321);
        assert!(a.eq(&b));
        assert!(*a.data() == 123);
        assert!(*b.data() == 321);

        let a = Sortable2DWith(1., 0., ());
        let b = Sortable2DWith(0., 0., ());
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable2DWith(0., 0., ());
        let b = Sortable2DWith(1., 0., ());
        assert!(a.cmp(&b) == Ordering::Less);

        let a = Sortable2DWith(0., 1., ());
        let b = Sortable2DWith(0., 0., ());
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable2DWith(0., 0., ());
        let b = Sortable2DWith(0., 1., ());
        assert!(a.cmp(&b) == Ordering::Less);
    }

    #[test]
    fn sortable3d() {
        assert!(Sortable3D(0., 1., 2.) == Sortable3D(0., 1., 2.));
        assert!(Sortable3D(0., 1., 2.) != Sortable3D(1., 1., 2.));
        assert!(Sortable3D(0., 1., 2.) != Sortable3D(0., 2., 2.));
        assert!(Sortable3D(0., 1., 2.) != Sortable3D(0., 1., 3.));

        let a = Sortable3D(0., 0., 0.);
        let b = Sortable3D(0., 0., 0.);
        assert!(a.eq(&b));

        let a = Sortable3D(1., 0., 0.);
        let b = Sortable3D(0., 0., 0.);
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable3D(0., 0., 0.);
        let b = Sortable3D(1., 0., 0.);
        assert!(a.cmp(&b) == Ordering::Less);

        let a = Sortable3D(0., 1., 0.);
        let b = Sortable3D(0., 0., 0.);
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable3D(0., 0., 0.);
        let b = Sortable3D(0., 1., 0.);
        assert!(a.cmp(&b) == Ordering::Less);

        let a = Sortable3D(0., 0., 1.);
        let b = Sortable3D(0., 0., 0.);
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable3D(0., 0., 0.);
        let b = Sortable3D(0., 0., 1.);
        assert!(a.cmp(&b) == Ordering::Less);
    }

    #[test]
    fn sortable3d_with() {
        assert!(Sortable3DWith(0., 1., 2., ()) == Sortable3DWith(0., 1., 2., ()));
        assert!(Sortable3DWith(0., 1., 2., ()) != Sortable3DWith(1., 1., 2., ()));
        assert!(Sortable3DWith(0., 1., 2., ()) != Sortable3DWith(0., 2., 2., ()));
        assert!(Sortable3DWith(0., 1., 2., ()) != Sortable3DWith(0., 1., 3., ()));

        let a = Sortable3DWith(0., 0., 0., 123);
        let b = Sortable3DWith(0., 0., 0., 321);
        assert!(a.eq(&b));
        assert!(*a.data() == 123);
        assert!(*b.data() == 321);

        let a = Sortable3DWith(1., 0., 0., ());
        let b = Sortable3DWith(0., 0., 0., ());
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable3DWith(0., 0., 0., ());
        let b = Sortable3DWith(1., 0., 0., ());
        assert!(a.cmp(&b) == Ordering::Less);

        let a = Sortable3DWith(0., 1., 0., ());
        let b = Sortable3DWith(0., 0., 0., ());
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable3DWith(0., 0., 0., ());
        let b = Sortable3DWith(0., 1., 0., ());
        assert!(a.cmp(&b) == Ordering::Less);

        let a = Sortable3DWith(0., 0., 1., ());
        let b = Sortable3DWith(0., 0., 0., ());
        assert!(a.cmp(&b) == Ordering::Greater);

        let a = Sortable3DWith(0., 0., 0., ());
        let b = Sortable3DWith(0., 0., 1., ());
        assert!(a.cmp(&b) == Ordering::Less);
    }
}
