// Copyright The rust-url developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate abstracts over a Unicode back end for the `idna`
//! crate.
//!
//! To work around the lack of [`global-features`][1] in Cargo, this
//! crate allows the top level `Cargo.lock` to choose an alternative
//! Unicode back end for the `idna` crate by pinning a version of this
//! crate.
//!
//! See the README for more details.
//!
//! [1]: https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618

use unicode_normalization::UnicodeNormalization;

/// Turns a joining type into a mask for comparing with multiple type at once.
const fn joining_type_to_mask(jt: unicode_joining_type::JoiningType) -> u32 {
    1u32 << (jt as u32)
}

/// Mask for checking for both left and dual joining.
pub const LEFT_OR_DUAL_JOINING_MASK: JoiningTypeMask = JoiningTypeMask(
    joining_type_to_mask(unicode_joining_type::JoiningType::LeftJoining)
        | joining_type_to_mask(unicode_joining_type::JoiningType::DualJoining),
);

/// Mask for checking for both left and dual joining.
pub const RIGHT_OR_DUAL_JOINING_MASK: JoiningTypeMask = JoiningTypeMask(
    joining_type_to_mask(unicode_joining_type::JoiningType::RightJoining)
        | joining_type_to_mask(unicode_joining_type::JoiningType::DualJoining),
);

/// Turns a bidi class into a mask for comparing with multiple classes at once.
const fn bidi_class_to_mask(bc: unicode_bidi::BidiClass) -> u32 {
    1u32 << (bc as u32)
}

/// Mask for checking if the domain is a bidi domain.
pub const RTL_MASK: BidiClassMask = BidiClassMask(
    bidi_class_to_mask(unicode_bidi::BidiClass::R)
        | bidi_class_to_mask(unicode_bidi::BidiClass::AL)
        | bidi_class_to_mask(unicode_bidi::BidiClass::AN),
);

/// Mask for allowable bidi classes in the first character of a label
/// (either LTR or RTL) in a bidi domain.
pub const FIRST_BC_MASK: BidiClassMask = BidiClassMask(
    bidi_class_to_mask(unicode_bidi::BidiClass::L)
        | bidi_class_to_mask(unicode_bidi::BidiClass::R)
        | bidi_class_to_mask(unicode_bidi::BidiClass::AL),
);

// Mask for allowable bidi classes of the last (non-Non-Spacing Mark)
// character in an LTR label in a bidi domain.
pub const LAST_LTR_MASK: BidiClassMask = BidiClassMask(
    bidi_class_to_mask(unicode_bidi::BidiClass::L)
        | bidi_class_to_mask(unicode_bidi::BidiClass::EN),
);

// Mask for allowable bidi classes of the last (non-Non-Spacing Mark)
// character in an RTL label in a bidi domain.
pub const LAST_RTL_MASK: BidiClassMask = BidiClassMask(
    bidi_class_to_mask(unicode_bidi::BidiClass::R)
        | bidi_class_to_mask(unicode_bidi::BidiClass::AL)
        | bidi_class_to_mask(unicode_bidi::BidiClass::EN)
        | bidi_class_to_mask(unicode_bidi::BidiClass::AN),
);

// Mask for allowable bidi classes of the middle characters in an LTR label in a bidi domain.
pub const MIDDLE_LTR_MASK: BidiClassMask = BidiClassMask(
    bidi_class_to_mask(unicode_bidi::BidiClass::L)
        | bidi_class_to_mask(unicode_bidi::BidiClass::EN)
        | bidi_class_to_mask(unicode_bidi::BidiClass::ES)
        | bidi_class_to_mask(unicode_bidi::BidiClass::CS)
        | bidi_class_to_mask(unicode_bidi::BidiClass::ET)
        | bidi_class_to_mask(unicode_bidi::BidiClass::ON)
        | bidi_class_to_mask(unicode_bidi::BidiClass::BN)
        | bidi_class_to_mask(unicode_bidi::BidiClass::NSM),
);

// Mask for allowable bidi classes of the middle characters in an RTL label in a bidi domain.
pub const MIDDLE_RTL_MASK: BidiClassMask = BidiClassMask(
    bidi_class_to_mask(unicode_bidi::BidiClass::R)
        | bidi_class_to_mask(unicode_bidi::BidiClass::AL)
        | bidi_class_to_mask(unicode_bidi::BidiClass::AN)
        | bidi_class_to_mask(unicode_bidi::BidiClass::EN)
        | bidi_class_to_mask(unicode_bidi::BidiClass::ES)
        | bidi_class_to_mask(unicode_bidi::BidiClass::CS)
        | bidi_class_to_mask(unicode_bidi::BidiClass::ET)
        | bidi_class_to_mask(unicode_bidi::BidiClass::ON)
        | bidi_class_to_mask(unicode_bidi::BidiClass::BN)
        | bidi_class_to_mask(unicode_bidi::BidiClass::NSM),
);

/// Value for the Joining_Type Unicode property.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct JoiningType(unicode_joining_type::JoiningType);

impl JoiningType {
    /// Returns the corresponding `JoiningTypeMask`.
    #[inline(always)]
    pub fn to_mask(self) -> JoiningTypeMask {
        JoiningTypeMask(joining_type_to_mask(self.0))
    }

    // `true` iff this value is the Transparent value.
    #[inline(always)]
    pub fn is_transparent(self) -> bool {
        self.0 == unicode_joining_type::JoiningType::Transparent
    }
}

/// A mask representing potentially multiple `JoiningType`
/// values.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct JoiningTypeMask(u32);

impl JoiningTypeMask {
    /// `true` iff both masks have at `JoiningType` in common.
    #[inline(always)]
    pub fn intersects(self, other: JoiningTypeMask) -> bool {
        self.0 & other.0 != 0
    }
}

/// Value for the Bidi_Class Unicode property.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct BidiClass(unicode_bidi::BidiClass);

impl BidiClass {
    /// Returns the corresponding `BidiClassMask`.
    #[inline(always)]
    pub fn to_mask(self) -> BidiClassMask {
        BidiClassMask(bidi_class_to_mask(self.0))
    }

    /// `true` iff this value is Left_To_Right
    #[inline(always)]
    pub fn is_ltr(self) -> bool {
        self.0 == unicode_bidi::BidiClass::L
    }

    /// `true` iff this value is Nonspacing_Mark
    #[inline(always)]
    pub fn is_nonspacing_mark(self) -> bool {
        self.0 == unicode_bidi::BidiClass::NSM
    }

    /// `true` iff this value is European_Number
    #[inline(always)]
    pub fn is_european_number(self) -> bool {
        self.0 == unicode_bidi::BidiClass::EN
    }

    /// `true` iff this value is Arabic_Number
    #[inline(always)]
    pub fn is_arabic_number(self) -> bool {
        self.0 == unicode_bidi::BidiClass::AN
    }
}

/// A mask representing potentially multiple `BidiClass`
/// values.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct BidiClassMask(u32);

impl BidiClassMask {
    /// `true` iff both masks have at `BidiClass` in common.
    #[inline(always)]
    pub fn intersects(self, other: BidiClassMask) -> bool {
        self.0 & other.0 != 0
    }
}

/// An adapter between a Unicode back end an the `idna` crate.
#[non_exhaustive]
pub struct Adapter {}

#[cfg(feature = "compiled_data")]
impl Default for Adapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Adapter {
    /// Constructor using data compiled into the binary.
    #[cfg(feature = "compiled_data")]
    #[inline(always)]
    pub const fn new() -> Self {
        Self {}
    }

    /// `true` iff the Canonical_Combining_Class of `c` is Virama.
    #[inline(always)]
    pub fn is_virama(&self, c: char) -> bool {
        unicode_normalization::char::canonical_combining_class(c) == 9
    }

    /// `true` iff the General_Category of `c` is Mark, i.e. any of Nonspacing_Mark,
    /// Spacing_Mark, or Enclosing_Mark.
    #[inline(always)]
    pub fn is_mark(&self, c: char) -> bool {
        unicode_normalization::char::is_combining_mark(c)
    }

    /// Returns the Bidi_Class of `c`.
    #[inline(always)]
    pub fn bidi_class(&self, c: char) -> BidiClass {
        BidiClass(unicode_bidi::bidi_class(c))
    }

    /// Returns the Joining_Type of `c`.
    #[inline(always)]
    pub fn joining_type(&self, c: char) -> JoiningType {
        JoiningType(unicode_joining_type::get_joining_type(c))
    }

    /// See the [method of the same name in `icu_normalizer`][1] for the
    /// exact semantics.
    ///
    /// [1]: https://docs.rs/icu_normalizer/latest/icu_normalizer/uts46/struct.Uts46Mapper.html#method.map_normalize
    #[inline(always)]
    pub fn map_normalize<'delegate, I: Iterator<Item = char> + 'delegate>(
        &'delegate self,
        iter: I,
    ) -> impl Iterator<Item = char> + 'delegate {
        idna_mapping::Mapper::new(iter, false).nfc()
    }

    /// See the [method of the same name in `icu_normalizer`][1] for the
    /// exact semantics.
    ///
    /// [1]: https://docs.rs/icu_normalizer/latest/icu_normalizer/uts46/struct.Uts46Mapper.html#method.normalize_validate
    #[inline(always)]
    pub fn normalize_validate<'delegate, I: Iterator<Item = char> + 'delegate>(
        &'delegate self,
        iter: I,
    ) -> impl Iterator<Item = char> + 'delegate {
        idna_mapping::Mapper::new(iter, true).nfc()
    }
}
