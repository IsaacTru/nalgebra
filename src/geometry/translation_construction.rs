#[cfg(feature = "arbitrary")]
use crate::base::storage::Owned;
#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

use num::{One, Zero};
#[cfg(feature = "rand-no-std")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use simba::scalar::{ClosedAdd, SupersetOf};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimName, U1, U2, U3, U4, U5, U6};
use crate::base::{DefaultAllocator, Scalar, VectorN};

use crate::geometry::Translation;

impl<N: Scalar, D: DimName> Translation<N, D>
where
    DefaultAllocator: Allocator<N, D>,
{
    /// Creates a new identity translation.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Point2, Point3, Translation2, Translation3};
    /// let t = Translation2::identity();
    /// let p = Point2::new(1.0, 2.0);
    /// assert_eq!(t * p, p);
    ///
    /// // Works in all dimensions.
    /// let t = Translation3::identity();
    /// let p = Point3::new(1.0, 2.0, 3.0);
    /// assert_eq!(t * p, p);
    /// ```
    #[inline]
    pub fn identity() -> Translation<N, D>
    where
        N: Zero,
    {
        Self::from(VectorN::<N, D>::from_element(N::zero()))
    }

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Translation2;
    /// let tra = Translation2::new(1.0f64, 2.0);
    /// let tra2 = tra.cast::<f32>();
    /// assert_eq!(tra2, Translation2::new(1.0f32, 2.0));
    /// ```
    pub fn cast<To: Scalar>(self) -> Translation<To, D>
    where
        Translation<To, D>: SupersetOf<Self>,
        DefaultAllocator: Allocator<To, D>,
    {
        crate::convert(self)
    }
}

impl<N: Scalar + Zero + ClosedAdd, D: DimName> One for Translation<N, D>
where
    DefaultAllocator: Allocator<N, D>,
{
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}

#[cfg(feature = "rand-no-std")]
impl<N: Scalar, D: DimName> Distribution<Translation<N, D>> for Standard
where
    DefaultAllocator: Allocator<N, D>,
    Standard: Distribution<N>,
{
    /// Generate an arbitrary random variate for testing purposes.
    #[inline]
    fn sample<'a, G: Rng + ?Sized>(&self, rng: &'a mut G) -> Translation<N, D> {
        Translation::from(rng.gen::<VectorN<N, D>>())
    }
}

#[cfg(feature = "arbitrary")]
impl<N: Scalar + Arbitrary + Send, D: DimName> Arbitrary for Translation<N, D>
where
    DefaultAllocator: Allocator<N, D>,
    Owned<N, D>: Send,
{
    #[inline]
    fn arbitrary(rng: &mut Gen) -> Self {
        let v: VectorN<N, D> = Arbitrary::arbitrary(rng);
        Self::from(v)
    }
}

/*
 *
 * Small translation construction from components.
 *
 */
macro_rules! componentwise_constructors_impl(
    ($($doc: expr; $D: ty, $($args: ident:$irow: expr),*);* $(;)*) => {$(
        impl<N: Scalar> Translation<N, $D>
            where DefaultAllocator: Allocator<N, $D> {
            #[doc = "Initializes this translation from its components."]
            #[doc = "# Example\n```"]
            #[doc = $doc]
            #[doc = "```"]
            #[inline]
            pub fn new($($args: N),*) -> Self {
                Self::from(VectorN::<N, $D>::new($($args),*))
            }
        }
    )*}
);

componentwise_constructors_impl!(
    "# use nalgebra::Translation1;\nlet t = Translation1::new(1.0);\nassert!(t.vector.x == 1.0);";
    U1, x:0;
    "# use nalgebra::Translation2;\nlet t = Translation2::new(1.0, 2.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0);";
    U2, x:0, y:1;
    "# use nalgebra::Translation3;\nlet t = Translation3::new(1.0, 2.0, 3.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0);";
    U3, x:0, y:1, z:2;
    "# use nalgebra::Translation4;\nlet t = Translation4::new(1.0, 2.0, 3.0, 4.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0);";
    U4, x:0, y:1, z:2, w:3;
    "# use nalgebra::Translation5;\nlet t = Translation5::new(1.0, 2.0, 3.0, 4.0, 5.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0 && t.vector.a == 5.0);";
    U5, x:0, y:1, z:2, w:3, a:4;
    "# use nalgebra::Translation6;\nlet t = Translation6::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);\nassert!(t.vector.x == 1.0 && t.vector.y == 2.0 && t.vector.z == 3.0 && t.vector.w == 4.0 && t.vector.a == 5.0 && t.vector.b == 6.0);";
    U6, x:0, y:1, z:2, w:3, a:4, b:5;
);
