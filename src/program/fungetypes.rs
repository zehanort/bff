pub trait FungeInteger:
    num_traits::PrimInt
    + num_traits::Signed
    + num_traits::ops::overflowing::OverflowingAdd
    + num_traits::ops::overflowing::OverflowingMul
    + num_traits::ops::overflowing::OverflowingSub
    + num_traits::ToPrimitive
    + Default
    + std::fmt::Display
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
            + std::fmt::Display
            + std::fmt::Debug,
    > FungeInteger for T
{
}
