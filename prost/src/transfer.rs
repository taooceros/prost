//! Owned and borrowed asynchronous transfer APIs for Protocol Buffers messages.
//!
//! The borrowed async-ref API mirrors prost's synchronous encoding shape for
//! callers that already own a destination, while [`EncodeEngine`] keeps the
//! owned future-shaped boundary for engines that need to retain message and
//! destination state across `Poll::Pending`.

use alloc::vec::Vec;
use bytes::{BufMut, BytesMut};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::encoding::varint::{encode_varint, encoded_len_varint};
use crate::{EncodeError, Message};

/// Options that control how a message is encoded by an [`EncodeEngine`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EncodeOptions {
    /// Encode the message with a protobuf length delimiter.
    pub length_delimited: bool,
}

/// Owned destination storage for an asynchronous encode operation.
///
/// Engines receive the destination by value and borrow its [`BufMut`] view only
/// while writing bytes. Engines that need stronger guarantees can define their
/// own destination type without changing generated message structs.
pub trait EncodeDst {
    /// Borrowed mutable protobuf byte buffer exposed by this destination.
    type BufMut<'a>: BufMut
    where
        Self: 'a;

    /// Returns the byte buffer used for this encode operation.
    fn as_buf_mut(&mut self) -> Self::BufMut<'_>;
}

impl EncodeDst for Vec<u8> {
    type BufMut<'a> = &'a mut Vec<u8>;

    #[inline]
    fn as_buf_mut(&mut self) -> Self::BufMut<'_> {
        self
    }
}

impl EncodeDst for BytesMut {
    type BufMut<'a> = &'a mut BytesMut;

    #[inline]
    fn as_buf_mut(&mut self) -> Self::BufMut<'_> {
        self
    }
}

/// Borrowed payload bytes offered to an [`AsyncEncodeTarget`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncodePayload<'a> {
    bytes: &'a [u8],
}

impl<'a> EncodePayload<'a> {
    /// Returns the payload bytes.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the payload byte length.
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns true when the payload is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl<'a> From<&'a [u8]> for EncodePayload<'a> {
    #[inline]
    fn from(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

/// Borrowed destination used by generated asynchronous Prost encoding.
///
/// A target exposes a short-lived [`BufMut`] view for protobuf structure writes
/// and a separate payload-copy hook for `string` / `bytes` field contents. The
/// default CPU target copies payloads immediately; experimental targets can
/// return [`Poll::Pending`] while an external copy engine initializes the
/// destination bytes.
pub trait AsyncEncodeTarget {
    /// Error produced while starting or completing an asynchronous encode.
    type Error: Send + Sync + 'static;

    /// Borrowed protobuf byte buffer for CPU-written keys, lengths, and scalars.
    type BufMut<'a>: BufMut + Send
    where
        Self: 'a;

    /// Per-payload state kept by poll-based generated encoders while a target
    /// copy is pending.
    type PayloadState: Default + Send;

    /// Converts a Prost capacity error into this target's error type.
    fn encode_error(error: EncodeError) -> Self::Error;

    /// Returns the byte buffer used for CPU-written protobuf structure.
    fn buf_mut(&mut self) -> Self::BufMut<'_>;

    /// Copies a `string` or `bytes` payload into the destination.
    fn poll_write_payload(
        &mut self,
        payload: EncodePayload<'_>,
        state: Pin<&mut Self::PayloadState>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;

    /// Future-shaped adapter around [`AsyncEncodeTarget::poll_write_payload`].
    fn write_payload<'a>(&'a mut self, payload: EncodePayload<'a>) -> WritePayload<'a, Self>
    where
        Self: Sized,
    {
        WritePayload::new(self, payload)
    }
}

/// Cursor for one generated-message poll encoder frame.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct PollEncodeFrame {
    field: usize,
    index: usize,
    phase: u8,
}

/// Universal generated-message state for poll-based async encoding.
pub struct PollEncodeState<S: AsyncEncodeTarget> {
    root: PollEncodeFrame,
    nested: Vec<PollEncodeFrame>,
    depth: usize,
    payload: S::PayloadState,
}

impl<S: AsyncEncodeTarget> Default for PollEncodeState<S> {
    fn default() -> Self {
        Self {
            root: PollEncodeFrame::default(),
            nested: Vec::new(),
            depth: 0,
            payload: S::PayloadState::default(),
        }
    }
}

impl<S: AsyncEncodeTarget> PollEncodeState<S> {
    #[inline]
    fn frame(&self) -> &PollEncodeFrame {
        if self.depth == 0 {
            &self.root
        } else {
            &self.nested[self.depth - 1]
        }
    }

    #[inline]
    fn frame_mut(&mut self) -> &mut PollEncodeFrame {
        if self.depth == 0 {
            &mut self.root
        } else {
            &mut self.nested[self.depth - 1]
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        *self.frame_mut() = PollEncodeFrame::default();
        if self.depth == 0 {
            self.nested.clear();
        }
    }

    #[inline]
    pub fn field(&self) -> usize {
        self.frame().field
    }

    #[inline]
    pub fn advance_field(&mut self) {
        let frame = self.frame_mut();
        frame.field += 1;
        frame.index = 0;
        frame.phase = 0;
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.frame().index
    }

    #[inline]
    pub fn set_index(&mut self, index: usize) {
        self.frame_mut().index = index;
    }

    #[inline]
    pub fn phase(&self) -> u8 {
        self.frame().phase
    }

    #[inline]
    pub fn set_phase(&mut self, phase: u8) {
        self.frame_mut().phase = phase;
    }

    #[inline]
    pub fn enter_nested(&mut self) {
        let nested_index = self.depth;
        if self.nested.len() == nested_index {
            self.nested.push(PollEncodeFrame::default());
        }
        self.depth += 1;
    }

    #[inline]
    pub fn leave_nested_pending(&mut self) {
        debug_assert!(self.depth > 0);
        self.depth -= 1;
    }

    #[inline]
    pub fn leave_nested_ready(&mut self) {
        debug_assert!(self.depth > 0);
        let nested_index = self.depth - 1;
        self.nested.truncate(nested_index);
        self.depth -= 1;
    }

    #[inline]
    pub fn payload_pin_mut(&mut self) -> Pin<&mut S::PayloadState> {
        // SAFETY: `PollEncodeState` is driven from a pinned encode future in
        // the async paths that can return `Pending`. CPU paths complete
        // synchronously and do not rely on pinning.
        unsafe { Pin::new_unchecked(&mut self.payload) }
    }
}
/// Future-shaped adapter for a single payload copy.
pub struct WritePayload<'a, T: AsyncEncodeTarget> {
    target: &'a mut T,
    payload: EncodePayload<'a>,
    state: T::PayloadState,
}

impl<'a, T: AsyncEncodeTarget> WritePayload<'a, T> {
    #[inline]
    fn new(target: &'a mut T, payload: EncodePayload<'a>) -> Self {
        Self {
            target,
            payload,
            state: T::PayloadState::default(),
        }
    }
}

impl<T> Future for WritePayload<'_, T>
where
    T: AsyncEncodeTarget + Send,
{
    type Output = Result<(), T::Error>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: the future is pinned before polling, and the payload state is
        // only accessed in place until it is dropped.
        let this = unsafe { self.get_unchecked_mut() };
        let state = unsafe { Pin::new_unchecked(&mut this.state) };
        this.target.poll_write_payload(this.payload, state, cx)
    }
}

/// Future-shaped adapter around [`Message::poll_encode_raw`].
pub struct EncodeRawAsync<'a, M, T>
where
    M: Message,
    T: AsyncEncodeTarget,
{
    message: &'a M,
    target: &'a mut T,
    state: PollEncodeState<T>,
}

impl<'a, M, T> EncodeRawAsync<'a, M, T>
where
    M: Message,
    T: AsyncEncodeTarget,
{
    #[inline]
    pub(crate) fn new(message: &'a M, target: &'a mut T) -> Self {
        Self {
            message,
            target,
            state: PollEncodeState::default(),
        }
    }
}

impl<M, T> Future for EncodeRawAsync<'_, M, T>
where
    M: Message,
    T: AsyncEncodeTarget + Send,
{
    type Output = Result<(), T::Error>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: once this future is pinned, its poll state is accessed in
        // place until completion or drop.
        let this = unsafe { self.get_unchecked_mut() };
        this.message
            .poll_encode_raw(this.target, &mut this.state, cx)
    }
}

/// CPU-backed [`AsyncEncodeTarget`] adapter for an existing [`BufMut`].
#[derive(Debug)]
pub struct BufMutEncodeTarget<'a, B> {
    buf: &'a mut B,
}

impl<'a, B> BufMutEncodeTarget<'a, B> {
    /// Wraps a mutable protobuf byte buffer.
    #[inline]
    pub fn new(buf: &'a mut B) -> Self {
        Self { buf }
    }
}

impl<B> AsyncEncodeTarget for BufMutEncodeTarget<'_, B>
where
    B: BufMut + Send,
{
    type Error = EncodeError;

    type BufMut<'a>
        = &'a mut B
    where
        Self: 'a;

    type PayloadState = ();

    #[inline]
    fn encode_error(error: EncodeError) -> Self::Error {
        error
    }

    #[inline]
    fn buf_mut(&mut self) -> Self::BufMut<'_> {
        self.buf
    }

    #[inline]
    fn poll_write_payload(
        &mut self,
        payload: EncodePayload<'_>,
        _state: Pin<&mut Self::PayloadState>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.buf.put_slice(payload.as_bytes());
        Poll::Ready(Ok(()))
    }
}

/// Future-shaped borrowed async-ref encode operation.
pub struct EncodeMessageAsyncRef<'a, M, T>
where
    M: Message,
    T: AsyncEncodeTarget,
{
    message: &'a M,
    target: &'a mut T,
    options: EncodeOptions,
    state: PollEncodeState<T>,
    prepared: bool,
}

impl<'a, M, T> EncodeMessageAsyncRef<'a, M, T>
where
    M: Message,
    T: AsyncEncodeTarget,
{
    #[inline]
    fn new(message: &'a M, target: &'a mut T, options: EncodeOptions) -> Self {
        Self {
            message,
            target,
            options,
            state: PollEncodeState::default(),
            prepared: false,
        }
    }
}

impl<M, T> Future for EncodeMessageAsyncRef<'_, M, T>
where
    M: Message,
    T: AsyncEncodeTarget + Send,
{
    type Output = Result<(), T::Error>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: once this future is pinned, its poll state is accessed in
        // place until completion or drop.
        let this = unsafe { self.get_unchecked_mut() };

        if !this.prepared {
            let len = this.message.encoded_len();
            let required = if this.options.length_delimited {
                len + encoded_len_varint(len as u64)
            } else {
                len
            };
            let remaining = {
                let buf = this.target.buf_mut();
                buf.remaining_mut()
            };
            if required > remaining {
                return Poll::Ready(Err(T::encode_error(EncodeError::new(required, remaining))));
            }

            if this.options.length_delimited {
                let mut buf = this.target.buf_mut();
                encode_varint(len as u64, &mut buf);
            }
            this.prepared = true;
        }

        this.message
            .poll_encode_raw(this.target, &mut this.state, cx)
    }
}

/// Encodes `message` into a borrowed asynchronous target.
///
/// This helper preserves Prost's existing capacity check and length-delimited
/// framing behavior, then delegates the message body to
/// [`Message::poll_encode_raw`].
#[inline]
pub fn encode_message_async_ref<'a, M, T>(
    message: &'a M,
    target: &'a mut T,
    options: EncodeOptions,
) -> EncodeMessageAsyncRef<'a, M, T>
where
    M: Message + Sized,
    T: AsyncEncodeTarget + Send + 'a,
{
    EncodeMessageAsyncRef::new(message, target, options)
}

/// Extension trait for borrowed asynchronous encoding of prost messages.
pub trait AsyncEncodeRefExt: Message + Sized {
    /// Encodes this message into a borrowed asynchronous target.
    fn encode_async_ref<'a, T>(&'a self, target: &'a mut T) -> EncodeMessageAsyncRef<'a, Self, T>
    where
        T: AsyncEncodeTarget + Send + 'a;

    /// Encodes this message with a protobuf length delimiter into a borrowed
    /// asynchronous target.
    fn encode_length_delimited_async_ref<'a, T>(
        &'a self,
        target: &'a mut T,
    ) -> EncodeMessageAsyncRef<'a, Self, T>
    where
        T: AsyncEncodeTarget + Send + 'a;
}

impl<M> AsyncEncodeRefExt for M
where
    M: Message + Sized,
{
    #[inline]
    fn encode_async_ref<'a, T>(&'a self, target: &'a mut T) -> EncodeMessageAsyncRef<'a, Self, T>
    where
        T: AsyncEncodeTarget + Send + 'a,
    {
        encode_message_async_ref(self, target, EncodeOptions::default())
    }

    #[inline]
    fn encode_length_delimited_async_ref<'a, T>(
        &'a self,
        target: &'a mut T,
    ) -> EncodeMessageAsyncRef<'a, Self, T>
    where
        T: AsyncEncodeTarget + Send + 'a,
    {
        encode_message_async_ref(
            self,
            target,
            EncodeOptions {
                length_delimited: true,
            },
        )
    }
}

/// Engine that owns message transfer from a prost message into a destination.
///
/// The trait is intentionally separate from [`Message`], so generated structs
/// and existing synchronous call sites do not grow hardware-specific methods or
/// required async hooks.
pub trait EncodeEngine: Clone + Send + Sync + 'static {
    /// Error produced while starting or completing an encode operation.
    type Error: From<EncodeError> + Send + Sync + 'static;

    /// Owned future returned for one encode operation.
    type Encode<M, B>: Future<Output = Result<B, Self::Error>> + Send + 'static
    where
        M: Message + Send + 'static,
        B: EncodeDst + Send + 'static;

    /// Starts encoding `message` into owned destination `dst`.
    fn encode<M, B>(
        &self,
        message: M,
        dst: B,
        options: EncodeOptions,
    ) -> Result<Self::Encode<M, B>, Self::Error>
    where
        M: Message + Send + 'static,
        B: EncodeDst + Send + 'static;
}

/// CPU-backed encode engine using prost's existing synchronous [`Message`] API.
#[derive(Clone, Copy, Debug, Default)]
pub struct CpuEngine;

/// Ready future returned by [`CpuEngine`].
#[derive(Debug)]
pub struct CpuEncode<B> {
    result: Option<Result<B, EncodeError>>,
}

impl<B> CpuEncode<B> {
    #[inline]
    fn new(result: Result<B, EncodeError>) -> Self {
        Self {
            result: Some(result),
        }
    }

    /// Consumes this ready future and returns the encoded destination.
    ///
    /// This is useful for adapters that must preserve a synchronous ready fast
    /// path while still routing encoding through [`EncodeEngine`].
    #[inline]
    pub fn into_result(mut self) -> Result<B, EncodeError> {
        self.result
            .take()
            .expect("CpuEncode polled or consumed after completion")
    }
}

impl<B> Unpin for CpuEncode<B> {}

impl<B> Future for CpuEncode<B> {
    type Output = Result<B, EncodeError>;

    #[inline]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(
            self.get_mut()
                .result
                .take()
                .expect("CpuEncode polled after completion"),
        )
    }
}

impl EncodeEngine for CpuEngine {
    type Error = EncodeError;

    type Encode<M, B>
        = CpuEncode<B>
    where
        M: Message + Send + 'static,
        B: EncodeDst + Send + 'static;

    #[inline]
    fn encode<M, B>(
        &self,
        message: M,
        mut dst: B,
        options: EncodeOptions,
    ) -> Result<Self::Encode<M, B>, Self::Error>
    where
        M: Message + Send + 'static,
        B: EncodeDst + Send + 'static,
    {
        let result = {
            let mut buf = dst.as_buf_mut();
            if options.length_delimited {
                message.encode_length_delimited(&mut buf)
            } else {
                message.encode(&mut buf)
            }
        };

        Ok(CpuEncode::new(result.map(|()| dst)))
    }
}

/// Extension trait for owned asynchronous encoding of prost messages.
pub trait AsyncEncodeExt: Message + Sized + Send + 'static {
    /// Encodes this message with `engine` into owned destination `dst`.
    fn encode_async<E, B>(
        self,
        engine: E,
        dst: B,
        options: EncodeOptions,
    ) -> Result<E::Encode<Self, B>, E::Error>
    where
        E: EncodeEngine,
        B: EncodeDst + Send + 'static;
}

impl<M> AsyncEncodeExt for M
where
    M: Message + Sized + Send + 'static,
{
    #[inline]
    fn encode_async<E, B>(
        self,
        engine: E,
        dst: B,
        options: EncodeOptions,
    ) -> Result<E::Encode<Self, B>, E::Error>
    where
        E: EncodeEngine,
        B: EncodeDst + Send + 'static,
    {
        engine.encode(self, dst, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::{encode_key, key_len, DecodeContext, WireType};
    use crate::encoding::{encode_varint, encoded_len_varint};
    use crate::DecodeError;
    use bytes::Buf;
    use std::pin::pin;
    use std::task::{Context, Poll};

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct TestMessage {
        value: u32,
    }

    impl Message for TestMessage {
        fn encode_raw(&self, buf: &mut impl BufMut)
        where
            Self: Sized,
        {
            encode_key(1, WireType::Varint, buf);
            encode_varint(u64::from(self.value), buf);
        }

        fn merge_field(
            &mut self,
            _tag: u32,
            _wire_type: WireType,
            _buf: &mut impl Buf,
            _ctx: DecodeContext,
        ) -> Result<(), DecodeError>
        where
            Self: Sized,
        {
            unimplemented!("transfer tests only exercise encoding")
        }

        fn encoded_len(&self) -> usize {
            key_len(1) + encoded_len_varint(u64::from(self.value))
        }

        fn clear(&mut self) {
            self.value = 0;
        }
    }

    #[test]
    fn cpu_async_encode_matches_sync_encode() {
        let message = TestMessage { value: 150 };
        let mut expected = Vec::new();
        message.encode(&mut expected).expect("sync encode succeeds");

        let future = message
            .clone()
            .encode_async(CpuEngine, Vec::new(), EncodeOptions::default())
            .expect("cpu encode starts");
        let future = assert_send_static(future);
        let actual = poll_ready(future).expect("cpu encode succeeds");

        assert_eq!(actual, expected);
    }

    #[test]
    fn cpu_async_length_delimited_encode_matches_sync_encode() {
        let message = TestMessage { value: 65_535 };
        let mut expected = Vec::new();
        message
            .encode_length_delimited(&mut expected)
            .expect("sync length-delimited encode succeeds");

        let future = message
            .encode_async(
                CpuEngine,
                BytesMut::new(),
                EncodeOptions {
                    length_delimited: true,
                },
            )
            .expect("cpu encode starts");
        let actual = poll_ready(future).expect("cpu encode succeeds");

        assert_eq!(&actual[..], &expected[..]);
    }

    #[test]
    fn borrowed_async_ref_encode_matches_sync_encode() {
        let message = TestMessage { value: 150 };
        let mut expected = Vec::new();
        message.encode(&mut expected).expect("sync encode succeeds");

        let mut actual = Vec::new();
        let mut target = BufMutEncodeTarget::new(&mut actual);
        poll_ready(message.encode_async_ref(&mut target)).expect("borrowed async encode succeeds");

        assert_eq!(actual, expected);
    }

    #[test]
    fn borrowed_async_ref_length_delimited_encode_matches_sync_encode() {
        let message = TestMessage { value: 65_535 };
        let mut expected = Vec::new();
        message
            .encode_length_delimited(&mut expected)
            .expect("sync length-delimited encode succeeds");

        let mut actual = Vec::new();
        let mut target = BufMutEncodeTarget::new(&mut actual);
        poll_ready(message.encode_length_delimited_async_ref(&mut target))
            .expect("borrowed async length-delimited encode succeeds");

        assert_eq!(actual, expected);
    }

    fn assert_send_static<T: Send + 'static>(value: T) -> T {
        value
    }

    fn poll_ready<F: Future>(future: F) -> F::Output {
        let mut future = pin!(future);
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        match future.as_mut().poll(&mut cx) {
            Poll::Ready(output) => output,
            Poll::Pending => panic!("CpuEngine returned a pending future"),
        }
    }
}
