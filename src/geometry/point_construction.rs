#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

use num::{Bounded, One, Zero};
#[cfg(feature = "rand-no-std")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimName, DimNameAdd, DimNameSum, U1};
use crate::base::{DefaultAllocator, Scalar, VectorN};
use crate::{
    Point1, Point2, Point3, Point4, Point5, Point6, Vector1, Vector2, Vector3, Vector4, Vector5,
    Vector6,
};
use simba::scalar::{ClosedDiv, SupersetOf};

use crate::geometry::Point;

/// # Other construction methods
impl<N: Scalar, D: DimName> Point<N, D>
where
    DefaultAllocator: Allocator<N, D>,
{
    /// Creates a new point with uninitialized coordinates.
    #[inline]
    pub unsafe fn new_uninitialized() -> Self {
        Self::from(crate::unimplemented_or_uninitialized_generic!(
            D::name(),
            U1
        ))
    }

    /// Creates a new point with all coordinates equal to zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::{Point2, Point3};
    /// // This works in any dimension.
    /// // The explicit crate::<f32> type annotation may not always be needed,
    /// // depending on the context of type inference.
    /// let pt = Point2::<f32>::origin();
    /// assert!(pt.x == 0.0 && pt.y == 0.0);
    ///
    /// let pt = Point3::<f32>::origin();
    /// assert!(pt.x == 0.0 && pt.y == 0.0 && pt.z == 0.0);
    /// ```
    #[inline]
    pub fn origin() -> Self
    where
        N: Zero,
    {
        Self::from(VectorN::from_element(N::zero()))
    }

    /// Creates a new point from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::{Point2, Point3};
    /// let data = [ 1.0, 2.0, 3.0 ];
    ///
    /// let pt = Point2::from_slice(&data[..2]);
    /// assert_eq!(pt, Point2::new(1.0, 2.0));
    ///
    /// let pt = Point3::from_slice(&data);
    /// assert_eq!(pt, Point3::new(1.0, 2.0, 3.0));
    /// ```
    #[inline]
    pub fn from_slice(components: &[N]) -> Self {
        Self::from(VectorN::from_row_slice(components))
    }

    /// Creates a new point from its homogeneous vector representation.
    ///
    /// In practice, this builds a D-dimensional points with the same first D component as `v`
    /// divided by the last component of `v`. Returns `None` if this divisor is zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::{Point2, Point3, Vector3, Vector4};
    ///
    /// let coords = Vector4::new(1.0, 2.0, 3.0, 1.0);
    /// let pt = Point3::from_homogeneous(coords);
    /// assert_eq!(pt, Some(Point3::new(1.0, 2.0, 3.0)));
    ///
    /// // All component of the result will be divided by the
    /// // last component of the vector, here 2.0.
    /// let coords = Vector4::new(1.0, 2.0, 3.0, 2.0);
    /// let pt = Point3::from_homogeneous(coords);
    /// assert_eq!(pt, Some(Point3::new(0.5, 1.0, 1.5)));
    ///
    /// // Fails because the last component is zero.
    /// let coords = Vector4::new(1.0, 2.0, 3.0, 0.0);
    /// let pt = Point3::from_homogeneous(coords);
    /// assert!(pt.is_none());
    ///
    /// // Works also in other dimensions.
    /// let coords = Vector3::new(1.0, 2.0, 1.0);
    /// let pt = Point2::from_homogeneous(coords);
    /// assert_eq!(pt, Some(Point2::new(1.0, 2.0)));
    /// ```
    #[inline]
    pub fn from_homogeneous(v: VectorN<N, DimNameSum<D, U1>>) -> Option<Self>
    where
        N: Scalar + Zero + One + ClosedDiv,
        D: DimNameAdd<U1>,
        DefaultAllocator: Allocator<N, DimNameSum<D, U1>>,
    {
        if !v[D::dim()].is_zero() {
            let coords = v.fixed_slice::<D, U1>(0, 0) / v[D::dim()].inlined_clone();
            Some(Self::from(coords))
        } else {
            None
        }
    }

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Point2;
    /// let pt = Point2::new(1.0f64, 2.0);
    /// let pt2 = pt.cast::<f32>();
    /// assert_eq!(pt2, Point2::new(1.0f32, 2.0));
    /// ```
    pub fn cast<To: Scalar>(self) -> Point<To, D>
    where
        Point<To, D>: SupersetOf<Self>,
        DefaultAllocator: Allocator<To, D>,
    {
        crate::convert(self)
    }
}

/*
 *
 * Traits that build points.
 *
 */
impl<N: Scalar + Bounded, D: DimName> Bounded for Point<N, D>
where
    DefaultAllocator: Allocator<N, D>,
{
    #[inline]
    fn max_value() -> Self {
        Self::from(VectorN::max_value())
    }

    #[inline]
    fn min_value() -> Self {
        Self::from(VectorN::min_value())
    }
}

#[cfg(feature = "rand-no-std")]
impl<N: Scalar, D: DimName> Distribution<Point<N, D>> for Standard
where
    DefaultAllocator: Allocator<N, D>,
    Standard: Distribution<N>,
{
     /// Generate a `Point` where each coordinate is an independent variate from `[0, 1)`.
    #[inline]
    fn sample<'a, G: Rng + ?Sized>(&self, rng: &mut G) -> Point<N, D> {
        Point::from(rng.gen::<VectorN<N, D>>())
    }
}

#[cfg(feature = "arbitrary")]
impl<N: Scalar + Arbitrary + Send, D: DimName> Arbitrary for Point<N, D>
where
    DefaultAllocator: Allocator<N, D>,
    <DefaultAllocator as Allocator<N, D>>::Buffer: Send,
{
    #[inline]
    fn arbitrary(g: &mut Gen) -> Self {
        Self::from(VectorN::arbitrary(g))
    }
}

/*
 *
 * Small points construction from components.
 *
 */
// NOTE: the impl for Point1 is not with the others so that we
// can add a section with the impl block comment.
/// # Construction from individual components
impl<N: Scalar> Point1<N> {
    /// Initializes this point from its components.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::Point1;
    /// let p = Point1::new(1.0);
    /// assert_eq!(p.x, 1.0);
    /// ```
    #[inline]
    pub fn new(x: N) -> Self {
        Vector1::new(x).into()
    }
}
macro_rules! componentwise_constructors_impl(
    ($($doc: expr; $Point: ident, $Vector: ident, $($args: ident:$irow: expr),*);* $(;)*) => {$(
        impl<N: Scalar> $Point<N> {
            #[doc = "Initializes this point from its components."]
            #[doc = "# Example\n```"]
            #[doc = $doc]
            #[doc = "```"]
            #[inline]
            pub fn new($($args: N),*) -> Self {
                $Vector::new($($args),*).into()
            }
        }
    )*}
);

componentwise_constructors_impl!(
    "# use nalgebra::Point2;\nlet p = Point2::new(1.0, 2.0);\nassert!(p.x == 1.0 && p.y == 2.0);";
    Point2, Vector2, x:0, y:1;
    "# use nalgebra::Point3;\nlet p = Point3::new(1.0, 2.0, 3.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0);";
    Point3, Vector3, x:0, y:1, z:2;
    "# use nalgebra::Point4;\nlet p = Point4::new(1.0, 2.0, 3.0, 4.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0 && p.w == 4.0);";
    Point4, Vector4, x:0, y:1, z:2, w:3;
    "# use nalgebra::Point5;\nlet p = Point5::new(1.0, 2.0, 3.0, 4.0, 5.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0 && p.w == 4.0 && p.a == 5.0);";
    Point5, Vector5, x:0, y:1, z:2, w:3, a:4;
    "# use nalgebra::Point6;\nlet p = Point6::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);\nassert!(p.x == 1.0 && p.y == 2.0 && p.z == 3.0 && p.w == 4.0 && p.a == 5.0 && p.b == 6.0);";
    Point6, Vector6, x:0, y:1, z:2, w:3, a:4, b:5;
);

macro_rules! from_array_impl(
    ($($Point: ident, $len: expr);*) => {$(
      impl <N: Scalar> From<[N; $len]> for $Point<N> {
          fn from(coords: [N; $len]) -> Self {
              Self {
                coords: coords.into()
              }
          }
      }
    )*}
);

from_array_impl!(Point1, 1; Point2, 2; Point3, 3; Point4, 4; Point5, 5; Point6, 6);
