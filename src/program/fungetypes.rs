pub trait FungeInteger:
    num_traits::PrimInt
    + num_traits::Signed
    + num_traits::ops::overflowing::OverflowingAdd
    + num_traits::ops::overflowing::OverflowingMul
    + num_traits::ops::overflowing::OverflowingSub
    + num_traits::ToPrimitive
    + Default
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::fmt::Display
    + std::fmt::Binary
    + std::fmt::LowerHex
    + std::fmt::Octal
    + std::fmt::Debug
{
}

impl<
        T: num_traits::PrimInt
            + num_traits::Signed
            + num_traits::ops::overflowing::OverflowingAdd
            + num_traits::ops::overflowing::OverflowingMul
            + num_traits::ops::overflowing::OverflowingSub
            + num_traits::ToPrimitive
            + Default
            + std::ops::AddAssign
            + std::ops::SubAssign
            + std::ops::MulAssign
            + std::fmt::Display
            + std::fmt::Binary
            + std::fmt::LowerHex
            + std::fmt::Octal
            + std::fmt::Debug,
    > FungeInteger for T
{
}
