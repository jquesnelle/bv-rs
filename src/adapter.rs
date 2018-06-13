//! Lazy bit vector adapters, including bit-wise logic.
//!
//! This module defines an extension trait [`BitsLogic`] that is implemented
//! for every type that implements [`Bits`]. The trait provides bit-wise
//! logical operations on bit-vector-likes.
//!
//! [`Bits`]: ../trait.Bits.html
//! [`BitsLogic`]: trait.BitsLogic.html

use Bits;
use BitSliceable;

use std::cmp;

/// Extension trait for bit-wise logical operators on bit slices.
///
/// The methods return lazy adapter objects that query the underlying bit vectors
/// and perform logic operations as needed. To eagerly evaluate a result, copy
/// it into a vector using [`BitVec::from_bits`], as in the example below.
///
/// [`BitVec::from_bits`]: ../struct.BitVec.html#method.from_bits
///
/// # Examples
///
/// ```
/// use bv::*;
/// use bv::adapter::BitsLogic;
///
/// let bv1: BitVec = bit_vec![false, false, true, true];
/// let bv2: BitVec = bit_vec![false, true, false, true];
///
/// let and_bv = bv1.bits_and(&bv2);
///
/// assert_eq!( and_bv.get_bit(0), false );
/// assert_eq!( and_bv.get_bit(1), false );
/// assert_eq!( and_bv.get_bit(2), false );
/// assert_eq!( and_bv.get_bit(3), true );
///
/// let bv3 = BitVec::from_bits(and_bv);
/// assert_eq!( bv3, bit_vec![false, false, false, true] );
/// ```
pub trait BitsLogic: Bits {

    /// Returns an object that inverts the values of all the bits in `self`.
    fn bits_not(&self) -> BitsNot<&Self> {
        BitsNot(self)
    }

    /// Returns an object that inverts the values of all the bits in `self`.
    ///
    /// Consumes `self`.
    fn into_bits_not(self) -> BitsNot<Self>
        where Self: Sized
    {
        BitsNot(self)
    }

    /// Returns an object that lazily computes the bit-wise conjunction
    /// of two bit-vector-likes.
    fn bits_and<Other>(&self, other: Other) -> BitsAnd<&Self, Other>
        where Other: Bits<Block = Self::Block> {

        BitsAnd(BitsBinOp::new(self, other))
    }

    /// Returns an object that lazily computes the bit-wise conjunction
    /// of two bit-vector-likes.
    ///
    /// Consumes `self`.
    fn into_bits_and<Other>(self, other: Other) -> BitsAnd<Self, Other>
        where Self: Sized,
              Other: Bits<Block = Self::Block> {

        BitsAnd(BitsBinOp::new(self, other))
    }

    /// Returns an object that lazily computes the bit-wise disjunction
    /// of two bit-vector-likes.
    fn bits_or<Other>(&self, other: Other) -> BitsOr<&Self, Other>
        where Other: Bits<Block = Self::Block> {

        BitsOr(BitsBinOp::new(self, other))
    }

    /// Returns an object that lazily computes the bit-wise disjunction
    /// of two bit-vector-likes.
    ///
    /// Consumes `self`.
    fn into_bits_or<Other>(self, other: Other) -> BitsOr<Self, Other>
        where Self: Sized,
              Other: Bits<Block = Self::Block> {

        BitsOr(BitsBinOp::new(self, other))
    }

    /// Returns an object that lazily computes the bit-wise xor of two
    /// bit-vector-likes.
    fn bits_xor<Other>(&self, other: Other) -> BitsXor<&Self, Other>
        where Other: Bits<Block = Self::Block> {

        BitsXor(BitsBinOp::new(self, other))
    }

    /// Returns an object that lazily computes the bit-wise xor of two
    /// bit-vector-likes.
    ///
    /// Consumes `self`.
    fn into_bits_xor<Other>(self, other: Other) -> BitsXor<Self, Other>
        where Self: Sized,
              Other: Bits<Block = Self::Block> {

        BitsXor(BitsBinOp::new(self, other))
    }
}

impl<T: Bits> BitsLogic for T {}

/// The result of [`BitsLogic::bits_not`](trait.BitsLogic.html#method.bits_not).
///
/// The resulting bit vector adapter *not*s the bits of the underlying
/// bit-vector-like.
#[derive(Clone, Debug)]
pub struct BitsNot<T>(T);

/// The result of [`BitsLogic::bits_and`](trait.BitsLogic.html#method.bits_and).
///
/// The resulting bit vector adapter *and*s the bits of the two underlying
/// bit-vector-likes.
#[derive(Clone, Debug)]
pub struct BitsAnd<T, U>(BitsBinOp<T, U>);

/// The result of [`BitsLogic::bits_or`](trait.BitsLogic.html#method.bits_or).
///
/// The resulting bit vector adapter *or*s the bits of the two underlying
/// bit-vector-likes.
#[derive(Clone, Debug)]
pub struct BitsOr<T, U>(BitsBinOp<T, U>);

/// The result of [`BitsLogic::bits_xor`](trait.BitsLogic.html#method.bits_xor).
///
/// The resulting bit vector adapter *xor*s the bits of the two underlying
/// bit-vector-likes.
#[derive(Clone, Debug)]
pub struct BitsXor<T, U>(BitsBinOp<T, U>);

/// Used to store the two operands to a bitwise logical operation on
/// `Bits`es, along with the length of the result (min the length of
/// the operands) and the offset of the result (see invariant below).
/// (Note that both `len` and `off` are derivable from `op1` and `op2`,
/// but it probably makes sense to cache them.)
#[derive(Clone, Debug)]
struct BitsBinOp<T, U> {
    op1: T,
    op2: U,
    len: u64,
}

impl<T: Bits, U: Bits<Block = T::Block>> BitsBinOp<T, U> {
    fn new(op1: T, op2: U) -> Self {
        let len = cmp::min(op1.bit_len(), op2.bit_len());
        BitsBinOp { op1, op2, len, }
    }

    fn bit1(&self, position: u64) -> bool {
        self.op1.get_bit(position)
    }

    fn bit2(&self, position: u64) -> bool {
        self.op2.get_bit(position)
    }

    fn block1(&self, position: usize) -> T::Block {
        self.op1.get_block(position)
    }

    fn block2(&self, position: usize) -> T::Block {
        self.op2.get_block(position)
    }
}

impl<T: Bits> Bits for BitsNot<T> {
    type Block = T::Block;

    fn bit_len(&self) -> u64 {
        self.0.bit_len()
    }

    fn get_bit(&self, position: u64) -> bool {
        !self.0.get_bit(position)
    }

    fn get_block(&self, position: usize) -> Self::Block {
        !self.0.get_block(position)
    }
}

impl<T, U> Bits for BitsAnd<T, U>
    where T: Bits,
          U: Bits<Block = T::Block>
{
    type Block = T::Block;

    fn bit_len(&self) -> u64 {
        self.0.len
    }

    fn get_bit(&self, position: u64) -> bool {
        self.0.bit1(position) & self.0.bit2(position)
    }

    fn get_block(&self, position: usize) -> Self::Block {
        self.0.block1(position) & self.0.block2(position)
    }
}

impl<T, U> Bits for BitsOr<T, U>
    where T: Bits,
          U: Bits<Block = T::Block>
{
    type Block = T::Block;

    fn bit_len(&self) -> u64 {
        self.0.len
    }

    fn get_bit(&self, position: u64) -> bool {
        self.0.bit1(position) | self.0.bit2(position)
    }

    fn get_block(&self, position: usize) -> Self::Block {
        self.0.block1(position) | self.0.block2(position)
    }
}

impl<T, U> Bits for BitsXor<T, U>
    where T: Bits,
          U: Bits<Block = T::Block>
{
    type Block = T::Block;

    fn bit_len(&self) -> u64 {
        self.0.len
    }

    fn get_bit(&self, position: u64) -> bool {
        self.0.bit1(position) ^ self.0.bit2(position)
    }

    fn get_block(&self, position: usize) -> Self::Block {
        self.0.block1(position) ^ self.0.block2(position)
    }
}

impl<R, T> BitSliceable<R> for BitsNot<T>
    where T: BitSliceable<R> {

    type Slice = BitsNot<T::Slice>;

    fn bit_slice(self, range: R) -> Self::Slice {
        BitsNot(self.0.bit_slice(range))
    }
}

impl<R, T, U> BitSliceable<R> for BitsAnd<T, U>
    where R: Clone,
          T: BitSliceable<R> + Bits,
          U: BitSliceable<R> + Bits<Block = T::Block>,
          T::Slice: Bits<Block = T::Block>,
          U::Slice: Bits<Block = T::Block> {

    type Slice = BitsAnd<T::Slice, U::Slice>;

    fn bit_slice(self, range: R) -> Self::Slice {
        BitsAnd(BitsBinOp::new(self.0.op1.bit_slice(range.clone()),
                               self.0.op2.bit_slice(range)))
    }
}

impl<R, T, U> BitSliceable<R> for BitsOr<T, U>
    where R: Clone,
          T: BitSliceable<R> + Bits,
          U: BitSliceable<R> + Bits<Block = T::Block>,
          T::Slice: Bits<Block = T::Block>,
          U::Slice: Bits<Block = T::Block> {

    type Slice = BitsOr<T::Slice, U::Slice>;

    fn bit_slice(self, range: R) -> Self::Slice {
        BitsOr(BitsBinOp::new(self.0.op1.bit_slice(range.clone()),
                              self.0.op2.bit_slice(range)))
    }
}

impl<R, T, U> BitSliceable<R> for BitsXor<T, U>
    where R: Clone,
          T: BitSliceable<R> + Bits,
          U: BitSliceable<R> + Bits<Block = T::Block>,
          T::Slice: Bits<Block = T::Block>,
          U::Slice: Bits<Block = T::Block> {

    type Slice = BitsXor<T::Slice, U::Slice>;

    fn bit_slice(self, range: R) -> Self::Slice {
        BitsXor(BitsBinOp::new(self.0.op1.bit_slice(range.clone()),
                               self.0.op2.bit_slice(range)))
    }
}

#[cfg(test)]
mod test {
    use {Bits, BitVec, BitSliceable};
    use super::BitsLogic;

    fn assert_0001<T: Bits>(bits: &T) {
        assert_eq!( bits.bit_len(), 4 );

        assert!( !bits.get_bit(0) );
        assert!( !bits.get_bit(1) );
        assert!( !bits.get_bit(2) );
        assert!(  bits.get_bit(3) );

        let bv = BitVec::from_bits(bits);
        assert_eq!( bv, bit_vec![false, false, false, true] );
    }

    #[test]
    fn simple_not() {
        let bv: BitVec = bit_vec![true, true, true, false,];
        let not_bits = bv.bits_not();
        assert_0001(&not_bits);
    }

    #[test]
    fn simple_and() {
        let bv1: BitVec<u8> = bit_vec![ false, false, true, true, ];
        let bv2: BitVec<u8> = bit_vec![ false, true, false, true, ];
        let and_bits = bv1.bits_and(&bv2);
        assert_0001(&and_bits);
    }

    #[test]
    fn and_with_same_offset() {
        let bv1: BitVec<u8> = bit_vec![ true, false, false, true, true ];
        let bv2: BitVec<u8> = bit_vec![ true, false, true, false, true ];
        let bv_slice1 = bv1.bit_slice(1..);
        let bv_slice2 = bv2.bit_slice(1..);
        let and_bits = bv_slice1.bits_and(&bv_slice2);
        assert_0001(&and_bits);
    }

    #[test]
    fn and_with_different_offset() {
        let bv1: BitVec<u8> = bit_vec![ true, true, false, false, true, true ];
        let bv2: BitVec<u8> = bit_vec![ true, false, true, false, true ];
        let bv_slice1 = bv1.bit_slice(2..);
        let bv_slice2 = bv2.bit_slice(1..);
        let and_bits = bv_slice1.bits_and(&bv_slice2);
        assert_0001(&and_bits);
    }
}