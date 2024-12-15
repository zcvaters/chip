use na::{Matrix3, Point3, RealField, Vector3};

pub trait ToFromC<T> {
    fn from_c(x: T) -> Self;
    fn to_c(&self) -> T;
}

impl<N: RealField> ToFromC<[N; 3]> for Vector3<N> {
    fn from_c(x: [N; 3]) -> Self {
        Vector3::new(x[0].clone(), x[1].clone(), x[2].clone())
    }

    fn to_c(&self) -> [N; 3] {
        [self.x.clone(), self.y.clone(), self.z.clone()]
    }
}

impl<N: RealField> ToFromC<[N; 3]> for Point3<N> {
    fn from_c(x: [N; 3]) -> Self {
        Point3::new(x[0].clone(), x[1].clone(), x[2].clone())
    }

    fn to_c(&self) -> [N; 3] {
        [self.x.clone(), self.y.clone(), self.z.clone()]
    }
}

impl<N: RealField> ToFromC<[N; 9]> for Matrix3<N> {
    fn from_c(x: [N; 9]) -> Self {
        Matrix3::new(
            x[0].clone(),
            x[1].clone(),
            x[2].clone(),
            x[3].clone(),
            x[4].clone(),
            x[5].clone(),
            x[6].clone(),
            x[7].clone(),
            x[8].clone(),
        )
    }

    fn to_c(&self) -> [N; 9] {
        [
            self[(0, 0)].clone(),
            self[(0, 1)].clone(),
            self[(0, 2)].clone(),
            self[(1, 0)].clone(),
            self[(1, 1)].clone(),
            self[(1, 2)].clone(),
            self[(2, 0)].clone(),
            self[(2, 1)].clone(),
            self[(2, 2)].clone(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use extend::ToFromC;
    use na::{Matrix3, Point3, Vector3};

    #[test]
    fn vector3_to_c() {
        let x = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(x.to_c(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn vector3_from_c() {
        let x = [1.0, 2.0, 3.0];
        assert_eq!(Vector3::from_c(x), Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn point3_to_c() {
        let x = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(x.to_c(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn point3_from_c() {
        let x = [1.0, 2.0, 3.0];
        assert_eq!(Point3::from_c(x), Point3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn matrix3_to_c() {
        let x = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(x.to_c(), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    }

    #[test]
    fn matrix3_from_c() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let expected = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(Matrix3::from_c(x), expected);
    }
}
