//! Protocol Buffers well-known wrapper types.
//!
//! This module provides implementations of `Message` for Rust standard library types which
//! correspond to a Protobuf well-known wrapper type. The remaining well-known types are defined in
//! the `prost-types` crate in order to avoid a cyclic dependency between `prost` and
//! `prost-build`.

use alloc::string::String;
use alloc::vec::Vec;

use ::bytes::{Buf, BufMut, Bytes};

use crate::{
    encoding::{
        bool, bytes, double, float, int32, int64, skip_field, string, uint32, uint64,
        DecodeContext, WireType,
    },
    DecodeError, Message,
};

/// `google.protobuf.BoolValue`
impl Message for bool {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self {
            bool::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            bool::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self {
            2
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = false;
    }
}

/// `google.protobuf.UInt32Value`
impl Message for u32 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0 {
            uint32::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            uint32::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            uint32::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0;
    }
}

/// `google.protobuf.UInt64Value`
impl Message for u64 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0 {
            uint64::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            uint64::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            uint64::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0;
    }
}

/// `google.protobuf.Int32Value`
impl Message for i32 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0 {
            int32::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            int32::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            int32::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0;
    }
}

/// `google.protobuf.Int64Value`
impl Message for i64 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0 {
            int64::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            int64::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            int64::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0;
    }
}

/// `google.protobuf.FloatValue`
impl Message for f32 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0.0 {
            float::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            float::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0.0 {
            float::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0.0;
    }
}

/// `google.protobuf.DoubleValue`
impl Message for f64 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0.0 {
            double::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            double::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0.0 {
            double::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0.0;
    }
}

/// `google.protobuf.StringValue`
impl Message for String {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if !self.is_empty() {
            string::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            string::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if !self.is_empty() {
            string::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.clear();
    }
}

/// `google.protobuf.BytesValue`
impl Message for Vec<u8> {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if !self.is_empty() {
            bytes::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            bytes::merge_one_copy(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if !self.is_empty() {
            bytes::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.clear();
    }
}

/// `google.protobuf.BytesValue`
impl Message for Bytes {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if !self.is_empty() {
            bytes::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            bytes::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if !self.is_empty() {
            bytes::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.clear();
    }
}

/// `google.protobuf.Empty`
impl Message for () {
    fn encode_raw<B>(&self, _buf: &mut B)
    where
        B: BufMut,
    {
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        skip_field(wire_type, tag, buf, ctx)
    }
    fn encoded_len(&self) -> usize {
        0
    }
    fn clear(&mut self) {}
}

pub mod str {
    use alloc::string::{String, ToString};
    use core::fmt;
    use core::hash::Hash;
    use core::marker::PhantomData;
    use core::ops::Deref;
    use core::str::Utf8Error;

    use ::bytes::{Buf, BufMut, Bytes};

    use crate::encoding::{self, DecodeContext, WireType};
    use crate::error::DecodeError;
    use crate::Message;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
    pub struct Checked;
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
    pub struct Unchecked;

    #[derive(Debug, Clone)]
    pub struct ByteStr<Utf8Mode = Checked> {
        pub(crate) buf: Bytes,
        _marker: PhantomData<Utf8Mode>,
    }
    pub type ByteStrUnchecked = ByteStr<Unchecked>;

    impl ByteStr {
        pub fn from_utf8(buf: Bytes) -> Result<Self, Utf8Error> {
            // Validate that the provided buffer is UTF-8.
            core::str::from_utf8(&buf[..])?;

            Ok(ByteStr {
                buf,
                _marker: PhantomData::default(),
            })
        }
    }

    impl ByteStr<Unchecked> {
        pub fn from_utf8(buf: Bytes) -> Self {
            Self {
                buf,
                _marker: PhantomData::default(),
            }
        }
    }

    impl<T> ByteStr<T> {
        /// Returns the number of bytes in this [`ByteStr`].
        pub fn len(&self) -> usize {
            self.buf.len()
        }

        /// Returns if the [`ByteStr`] is empty.
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Clears the [`ByteStr`].
        pub fn clear(&mut self) {
            self.buf.clear()
        }

        pub fn as_str(&self) -> &str {
            // SAFETY: We checked that the provided buffer was valid UTF-8 at creation.
            unsafe { core::str::from_utf8_unchecked(&self.buf[..]) }
        }

        pub fn as_bytes(&self) -> &[u8] {
            &self.buf
        }

        /// Consume the [`ByteStr`] returning the backing [`bytes::Bytes`].
        pub fn into_bytes(self) -> ::bytes::Bytes {
            self.buf
        }
    }

    impl<T> Deref for ByteStr<T> {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            self.as_str()
        }
    }

    impl<T> AsRef<str> for ByteStr<T> {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }

    impl<T> From<String> for ByteStr<T> {
        fn from(value: String) -> Self {
            // Note: We're creating from a String, which is already guaranteed to be UTF-8.
            Self {
                buf: Bytes::from(value.into_bytes()),
                _marker: PhantomData::default(),
            }
        }
    }

    impl<'a, T> From<&'a str> for ByteStr<T> {
        fn from(value: &'a str) -> Self {
            Self::from(value.to_string())
        }
    }

    impl<T> Default for ByteStr<T> {
        fn default() -> Self {
            Self {
                buf: Bytes::default(),
                _marker: PhantomData::default(),
            }
        }
    }

    impl<T, S: AsRef<str>> PartialEq<S> for ByteStr<T> {
        fn eq(&self, other: &S) -> bool {
            self.as_str() == other.as_ref()
        }
    }

    impl<T> PartialEq<ByteStr<T>> for String {
        fn eq(&self, other: &ByteStr<T>) -> bool {
            self.as_str() == other.as_str()
        }
    }

    impl<T> PartialEq<ByteStr<T>> for &str {
        fn eq(&self, other: &ByteStr<T>) -> bool {
            *self == other.as_str()
        }
    }

    impl<T> Eq for ByteStr<T> {}

    impl<T> Ord for ByteStr<T> {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            self.as_str().cmp(other.as_str())
        }
    }

    impl<T> PartialOrd for ByteStr<T> {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T> Hash for ByteStr<T> {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.as_str().hash(state)
        }
    }

    impl<T> fmt::Display for ByteStr<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.as_str())
        }
    }

    impl Message for ByteStr {
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: BufMut,
        {
            if !self.is_empty() {
                encoding::bytes::encode(1, &self.buf, buf)
            }
        }

        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: WireType,
            buf: &mut B,
            ctx: DecodeContext,
        ) -> Result<(), DecodeError>
        where
            B: Buf,
        {
            if tag == 1 {
                encoding::byte_str::merge(wire_type, self, buf, ctx)
            } else {
                encoding::skip_field(wire_type, tag, buf, ctx)
            }
        }

        fn encoded_len(&self) -> usize {
            if !self.is_empty() {
                encoding::bytes::encoded_len(1, &self.buf)
            } else {
                0
            }
        }

        fn clear(&mut self) {
            self.clear();
        }
    }

    impl Message for ByteStr<Unchecked> {
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: BufMut,
        {
            if !self.is_empty() {
                encoding::bytes::encode(1, &self.buf, buf)
            }
        }

        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: WireType,
            buf: &mut B,
            ctx: DecodeContext,
        ) -> Result<(), DecodeError>
        where
            B: Buf,
        {
            if tag == 1 {
                encoding::byte_str_unchecked::merge(wire_type, self, buf, ctx)
            } else {
                encoding::skip_field(wire_type, tag, buf, ctx)
            }
        }

        fn encoded_len(&self) -> usize {
            if !self.is_empty() {
                encoding::bytes::encoded_len(1, &self.buf)
            } else {
                0
            }
        }

        fn clear(&mut self) {
            self.clear();
        }
    }
}
