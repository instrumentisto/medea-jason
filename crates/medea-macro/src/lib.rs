#![cfg_attr(any(doc, test), doc = include_str!("../README.md"))]
#![cfg_attr(not(any(doc, test)), doc = env!("CARGO_PKG_NAME"))]
#![deny(nonstandard_style, rustdoc::all, trivial_casts, trivial_numeric_casts)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
    clippy::absolute_paths,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::as_conversions,
    clippy::as_pointer_underscore,
    clippy::as_ptr_cast_mut,
    clippy::assertions_on_result_states,
    clippy::branches_sharing_code,
    clippy::cfg_not_test,
    clippy::clear_with_drain,
    clippy::clone_on_ref_ptr,
    clippy::collection_is_never_read,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::default_union_representation,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_include_without_cfg,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::equatable_if_let,
    clippy::empty_enum_variants_with_brackets,
    clippy::exit,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::infinite_loop,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_over_hash_type,
    clippy::iter_with_drain,
    clippy::large_include_file,
    clippy::large_stack_frames,
    clippy::let_underscore_untyped,
    clippy::literal_string_with_formatting_args,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::map_with_unused_argument_over_ranges,
    clippy::mem_forget,
    clippy::missing_assert_message,
    clippy::missing_asserts_for_indexing,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    clippy::multiple_inherent_impl,
    clippy::multiple_unsafe_ops_per_block,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_collect,
    clippy::needless_pass_by_ref_mut,
    clippy::needless_raw_strings,
    clippy::non_zero_suggestions,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::pathbuf_init_then_push,
    clippy::pedantic,
    clippy::precedence_bits,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::pub_without_shorthand,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::read_zero_byte_vec,
    clippy::redundant_clone,
    clippy::redundant_test_prefix,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::ref_patterns,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::return_and_then,
    clippy::same_name_method,
    clippy::semicolon_inside_block,
    clippy::set_contains_or_insert,
    clippy::shadow_unrelated,
    clippy::significant_drop_in_scrutinee,
    clippy::significant_drop_tightening,
    clippy::single_option_map,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::todo,
    clippy::too_long_first_doc_paragraph,
    clippy::trailing_empty_array,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::uninhabited_references,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unnecessary_struct_initialization,
    clippy::unused_peekable,
    clippy::unused_result_ok,
    clippy::unused_trait_names,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::while_float,
    clippy::wildcard_enum_match_arm,
    ambiguous_negative_literals,
    closure_returning_async_block,
    future_incompatible,
    impl_trait_redundant_captures,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    redundant_lifetimes,
    rust_2018_idioms,
    single_use_lifetimes,
    unit_bindings,
    unnameable_types,
    unreachable_pub,
    unstable_features,
    unused,
    variant_size_differences
)]

mod caused;
mod dart_bridge;
#[cfg(feature = "dart-codegen")]
mod dart_codegen;
mod dispatchable;
mod enum_delegate;
mod watchers;

#[cfg(test)]
use async_trait as _;
#[cfg(test)]
use medea_jason as _;
use proc_macro::TokenStream;

/// Delegates function calls to enum variants field.
/// Variants are expected to have only one field.
///
/// # How to use
///
/// ```rust
/// use medea_macro::enum_delegate;
///
/// #[enum_delegate(pub fn as_str(&self) -> &str)]
/// #[enum_delegate(pub fn push_str(&mut self, arg: &str))]
/// enum MyEnum {
///     Foo(String),
///     Bar(String),
/// }
///
/// let mut foo = MyEnum::Foo(String::from("foo"));
/// foo.push_str("_bar");
/// assert_eq!(foo.as_str(), "foo_bar")
/// ```
///
/// # Extended example
///
/// ```rust
/// use medea_macro::enum_delegate;
///
/// struct SomeState;
/// struct AnotherState;
///
/// struct Context {
///     some_value: i32,
/// }
///
/// struct Peer<S> {
///     context: Context,
///     state: S,
/// }
///
/// impl<T> Peer<T> {
///     pub fn some_value(&self) -> i32 {
///         self.context.some_value
///     }
///
///     pub fn function_with_additional_args(&self, some_arg: i32) -> i32 {
///         some_arg
///     }
///
///     pub fn mutable_function(&mut self) -> i32 {
///         let old_value = self.context.some_value;
///         self.context.some_value = 1000;
///         old_value
///     }
/// }
///
/// #[enum_delegate(pub fn some_value(&self) -> i32)]
/// #[enum_delegate(
///     pub fn function_with_additional_args(&self, some_arg: i32) -> i32
/// )]
/// #[enum_delegate(pub fn mutable_function(&mut self) -> i32)]
/// enum PeerStateMachine {
///     SomeState(Peer<SomeState>),
///     AnotherState(Peer<AnotherState>),
/// }
///
/// let mut peer = PeerStateMachine::SomeState(Peer {
///     context: Context { some_value: 10 },
///     state: SomeState,
/// });
///
/// assert_eq!(peer.some_value(), 10);
///
/// assert_eq!(peer.function_with_additional_args(100), 100);
///
/// assert_eq!(peer.mutable_function(), 10);
/// assert_eq!(peer.some_value(), 1000);
/// ```
#[proc_macro_attribute]
pub fn enum_delegate(args: TokenStream, input: TokenStream) -> TokenStream {
    enum_delegate::derive(&args, input)
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Generates a `*Handler` trait and dispatching function for some event,
/// represented as `enum`.
///
/// # How to use
///
/// ### 1. Declare `enum` for event variants and a `struct` to handle them.
///
/// ```rust
/// use medea_macro::dispatchable;
///
/// #[dispatchable]
/// enum Event {
///     Some { new_bar: i32 },
///     Another,
///     UnnamedVariant(i32, i32),
/// }
///
/// struct Foo {
///     bar: i32,
///     baz: i32,
/// }
/// ```
///
/// ### 2. Implement handler for your `struct`.
///
/// For the given `enum` macro generates a unique trait by adding `Handler`
/// to the end of its name. Each method of trait is created by `snake_case`'ing
/// `enum` variants and adding `on_` prefix.
///
/// `type Output` is a type which will be returned from all functions of
/// `EventHandler` trait.
///
/// ```rust
/// # use medea_macro::dispatchable;
/// #
/// # #[dispatchable]
/// # enum Event {
/// #     Some { new_bar: i32 },
/// #     Another,
/// #     UnnamedVariant(i32, i32),
/// # }
/// #
/// # struct Foo {
/// #     bar: i32,
/// #     baz: i32,
/// # }
/// #
/// impl EventHandler for Foo {
///     type Output = i32;
///
///     fn on_some(&mut self, new_bar: i32) -> Self::Output {
///         self.bar = new_bar;
///         self.bar
///     }
///
///     fn on_another(&mut self) -> Self::Output {
///         self.bar = 2;
///         self.bar
///     }
///
///     fn on_unnamed_variant(&mut self, data: (i32, i32)) -> Self::Output {
///         self.bar = data.0;
///         self.baz = data.1;
///         self.bar
///     }
/// }
/// ```
///
/// ### 3. Dispatch event with handler
///
/// For the given `enum` macro generates `dispatch_with()` method to dispatch
/// `enum` with a given handler.
///
/// ```rust
/// # use medea_macro::dispatchable;
/// #
/// # #[dispatchable]
/// # enum Event {
/// #     Some { new_bar: i32 },
/// #     Another,
/// #     UnnamedVariant(i32, i32),
/// # }
/// #
/// # struct Foo {
/// #     bar: i32,
/// #     baz: i32,
/// # }
/// #
/// # impl EventHandler for Foo {
/// #    type Output = i32;
/// #
/// #    fn on_some(&mut self, new_bar: i32) -> Self::Output {
/// #        self.bar = new_bar;
/// #        self.bar
/// #    }
/// #
/// #    fn on_another(&mut self) -> Self::Output {
/// #        self.bar = 2;
/// #        self.bar
/// #    }
/// #
/// #    fn on_unnamed_variant(&mut self, data: (i32, i32)) -> Self::Output {
/// #        self.bar = data.0;
/// #        self.baz = data.1;
/// #        self.bar
/// #    }
/// # }
/// #
/// #
/// let mut foo = Foo { bar: 0, baz: 0 };
///
/// let bar = Event::Some { new_bar: 1 }.dispatch_with(&mut foo);
/// assert_eq!(foo.bar, 1);
/// assert_eq!(bar, 1);
///
/// let bar = Event::Another.dispatch_with(&mut foo);
/// assert_eq!(foo.bar, 2);
/// assert_eq!(bar, 2);
///
/// let bar = Event::UnnamedVariant(3, 3).dispatch_with(&mut foo);
/// assert_eq!(foo.bar, 3);
/// assert_eq!(foo.baz, 3);
/// assert_eq!(bar, 3);
/// ```
///
/// # Customize `self` type in handler functions (optional)
///
/// By default, all handler functions take `&mut Self`, if this doesn't suit
/// your case, then you can specify the method receiver manually:
/// `#[dispatchable(self: Rc<Self>)]`, `#[dispatchable(self: &Self)]`.
///
/// You can use any type that is a valid `self` receiver, e.g. `self`, `&self`,
/// `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or
/// `self: Pin<P>` (where P is one of the previous, except `Self`).
///
/// ```rust
/// # use std::rc::Rc;
/// use medea_macro::dispatchable;
///
/// #[dispatchable(self: Rc<Self>)]
/// enum Event {
///     Variant,
/// }
///
/// struct Foo;
/// impl EventHandler for Foo {
///     type Output = ();
///
///     fn on_variant(self: Rc<Self>) {}
/// }
///
/// let foo = Rc::new(Foo);
///
/// Event::Variant.dispatch_with(foo);
/// ```
///
/// # Async handlers (optional)
///
/// It's possible to make handler methods `async`. Rust doesn't support `async`
/// trait methods at the moment, that's why [`async_trait`] is used.
///
/// ```rust
/// use async_trait::async_trait;
/// use medea_macro::dispatchable;
///
/// #[dispatchable(async_trait(?Send))]
/// enum Event {
///     Variant,
/// }
///
/// struct Foo;
/// #[async_trait(?Send)]
/// impl EventHandler for Foo {
///     type Output = ();
///
///     async fn on_variant(&mut self) {}
/// }
///
/// let mut foo = Foo;
///
/// Event::Variant.dispatch_with(&mut foo);
/// ```
///
/// [`async_trait`]: https://docs.rs/async-trait
#[proc_macro_attribute]
pub fn dispatchable(args: TokenStream, input: TokenStream) -> TokenStream {
    let enum_item = syn::parse_macro_input!(input as dispatchable::Item);
    let args = syn::parse_macro_input!(args as dispatchable::Args);
    dispatchable::expand(enum_item, &args)
}

/// Generates `ComponentState` implementation on provided `impl`.
///
/// # Usage
///
/// ```rust,ignore
/// use std::rc::Rc;
///
/// use medea_jason::utils::Component;
/// use medea_macro::{watchers, watch};
///
/// struct SenderState {
///     muted: ObservableCell<bool>,
///     enabled: ObservableCell<bool>,
/// }
///
/// struct Sender;
///
/// type SenderComponent = Component<SenderState, Sender>;
///
/// #[watchers]
/// impl SenderComponent {
///     #[watch(self.muted.subscribe())]
///     async fn muted_change_watcher(
///         ctx: Rc<Sender>,
///         state: Rc<SenderState>,
///         new_muted_val: bool
///     ) -> Result<(), ()> {
///         Ok(())
///     }
///
///     #[watch(self.enabled.subscribe())]
///     async fn enabled_change_watcher(
///         ctx: Rc<Sender>,
///         state: Rc<SenderState>,
///         new_enabled_val: bool,
///     ) -> Result<(), ()> {
///         Ok(())
///     }
/// }
/// ```
///
/// ## `SenderComponent` implementation after macro expansion
///
/// ```rust,ignore
/// impl SenderComponent {
///     async fn muted_change_watcher(
///         sender: Rc<Sender>,
///         state: Rc<SenderState>,
///         new_muted_val: bool
///     ) -> Result<(), ()> {
///         Ok(())
///     }
///
///     async fn enabled_change_watcher(
///         sender: Rc<Sender>,
///         state: Rc<SenderState>,
///         new_enabled_val: bool,
///     ) -> Result<(), ()> {
///         Ok(())
///     }
/// }
///
/// impl ComponentState<Sender> for SenderState {
///     fn spawn_watchers(&self, s: &mut WatchersSpawner<SenderState, Sender>) {
///         s.spawn(
///             self.muted.subscribe(),
///             SenderComponent::muted_change_watcher,
///         );
///         s.spawn(
///             self.enabled.subscribe(),
///             SenderComponent::enabled_change_watcher,
///         );
///     }
/// }
/// ```
///
/// __Note__: `ComponentState` implementation is simplified in this example
///           for better readability.
///
/// In reality object and state types will be obtained by casting
/// `SenderComponent` to the `ComponentTypes` trait and getting types from it.
#[proc_macro_attribute]
pub fn watchers(_: TokenStream, input: TokenStream) -> TokenStream {
    watchers::expand(syn::parse_macro_input!(input))
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Generate implementation of `Caused` trait for errors represented as enum.
///
/// # How to use
///
/// ### 1. Declare custom error and enum for error variants.
///
/// The `cause()` method returns error if nested error has its type declared
/// as an argument of the attribute `#[cause(error = path::to::Error)]` or the
/// error type is assumed to be imported as `Error`.
///
/// ```rust
/// use medea_jason::utils::Caused;
///
/// struct MyError;
///
/// #[derive(Caused)]
/// #[cause(error = MyError)]
/// enum FooError {
///     Internal,
///     MyError(MyError),
/// }
///
/// let err = FooError::Internal;
/// assert!(err.cause().is_none());
///
/// let err = FooError::MyError(MyError {});
/// assert!(err.cause().is_some());
/// ```
///
/// If enum variant has attribute `#[cause]` it will call the `cause()`
/// method on nested error.
///
/// ```rust
/// # use medea_jason::utils::Caused;
/// #
/// # struct MyError;
/// #
/// # #[derive(Caused)]
/// # #[cause(error = MyError)]
/// # enum FooError {
/// #     Internal,
/// #     MyError(MyError),
/// # }
/// #
/// #[derive(Caused)]
/// #[cause(error = MyError)]
/// enum BarError {
///     Foo(#[cause] FooError),
/// }
///
/// let err = BarError::Foo(FooError::Internal);
/// assert!(err.cause().is_none());
///
/// let err = BarError::Foo(FooError::MyError(MyError {}));
/// assert!(err.cause().is_some());
/// ```
#[proc_macro_derive(Caused, attributes(cause))]
pub fn derive_caused(input: TokenStream) -> TokenStream {
    syn::parse::<syn::DeriveInput>(input)
        .and_then(|i| {
            let mut s = synstructure::Structure::try_new(&i)?;
            caused::derive(&mut s)
        })
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Generates code for `extern` Dart functions registration and calling.
///
/// # Usage
///
/// ## Macro call
///
/// ```ignore
/// // Code will be generated in the `peer_connection` module, also you can
/// // control visibility of this module with a visibility modifier (`pub`).
/// //
/// // Module name will be used as a prefix for a registration function.
/// #[dart_bridge]
/// mod peer_connection {
///     use dart_sys::Dart_Handle;
///
///     use crate::platform::Error;
///
///     extern "C" {
///         // This documentation will be injected to the generated
///         // extern function caller:
///
///         /// Returns newly created SDP offer of the provided
///         /// `PeerConnection`.
///         fn create_offer(peer: Dart_Handle) -> Result<Dart_Handle, Error>;
///
///         /// Returns a newly created SDP answer of the provided
///         /// `PeerConnection`.
///         fn create_answer(peer: Dart_Handle) -> Result<Dart_Handle, Error>;
///     }
/// }
/// ```
///
/// ## Example of the generated code
///
/// ```ignore
/// mod peer_connection {
///     use dart_sys::Dart_Handle;
///
///     use crate::platform::Error;
///
///     type PeerConnectionCreateOfferFunction =
///         extern "C" fn(peer: Dart_Handle) -> Dart_Handle;
///     type PeerConnectionCreateAnswerFunction =
///         extern "C" fn(peer: Dart_Handle) -> Dart_Handle;
///
///     static PEER_CONNECTION__CREATE_OFFER__FUNCTION:
///         ::sync_unsafe_cell::SyncUnsafeCell<
///             Option<PeerConnectionCreateOfferFunction>,
///         > = ::sync_unsafe_cell::SyncUnsafeCell::new(None);
///
///     static PEER_CONNECTION__CREATE_ANSWER__FUNCTION:
///         ::sync_unsafe_cell::SyncUnsafeCell<
///             Option<PeerConnectionCreateAnswerFunction>,
///         > = ::sync_unsafe_cell::SyncUnsafeCell::new(None);
///
///     ::std::thread_local! {
///         static PEER_CONNECTION__CREATE_OFFER__ERROR: ::std::cell::RefCell<
///             Option<Error>
///         > = ::std::cell::RefCell::new(None);
///     }
///
///     ::std::thread_local! {
///         static PEER_CONNECTION__CREATE_ANSWER__ERROR: ::std::cell::RefCell<
///             Option<Error>
///         > = ::std::cell::RefCell::new(None);
///     }
///
///     #[unsafe(no_mangle)]
///     pub unsafe extern "C" fn register_peer_connection(
///         create_offer: PeerConnectionCreateOfferFunction,
///         create_answer: PeerConnectionCreateAnswerFunction,
///     ) {
///         *::sync_unsafe_cell::SyncUnsafeCell::get(
///             &PEER_CONNECTION__CREATE_OFFER__FUNCTION,
///         ) = Some(create_offer);
///         *::sync_unsafe_cell::SyncUnsafeCell::get(
///             &PEER_CONNECTION__CREATE_ANSWER__FUNCTION,
///         ) = Some(create_answer);
///     }
///
///     /// Error setter for the `create_offer` function.
///     #[unsafe(no_mangle)]
///     unsafe extern "C" fn peer_connection__create_offer__set_error(
///         err: Dart_Handle,
///     ) {
///         PEER_CONNECTION__CREATE_OFFER__ERROR.set(
///             Some(Error::from_handle(err))
///         );
///     }
///
///     /// Error setter for the `create_answer` function.
///     #[unsafe(no_mangle)]
///     unsafe extern "C" fn peer_connection__create_answer__set_error(
///         err: Dart_Handle,
///     ) {
///         PEER_CONNECTION__CREATE_ANSWER__ERROR.set(
///             Some(Error::from_handle(err))
///         );
///     }
///
///     /// Returns newly created SDP offer of the provided `PeerConnection`.
///     unsafe fn create_offer(
///         peer: Dart_Handle,
///     ) -> Result<Dart_Handle, Error> {
///         let res = (
///             *(*PEER_CONNECTION__CREATE_OFFER__FUNCTION.get())
///                 .as_ref()
///                 .unwrap()
///         )(peer);
///         if let Some(e) = PEER_CONNECTION__CREATE_OFFER__ERROR.take() {
///             Err(e)
///         } else {
///             Ok(res)
///         }
///     }
///
///     /// Returns a newly created SDP answer of the provided `PeerConnection`.
///     unsafe fn create_answer(
///         peer: Dart_Handle,
///     ) -> Result<Dart_Handle, Error> {
///         let res = (
///             *(*PEER_CONNECTION__CREATE_ANSWER__FUNCTION.get())
///                 .as_ref()
///                 .unwrap()
///         )(peer);
///         if let Some(e) = PEER_CONNECTION__CREATE_ANSWER__ERROR.take() {
///             Err(e)
///         } else {
///             Ok(res)
///         }
///     }
/// }
/// ```
///
/// ## Generated code usage
///
/// ```ignore
/// struct PeerConnection(Dart_Handle);
///
/// impl PeerConnection {
///     pub async fn create_offer(&self) -> RtcPeerConnectionResult<String> {
///         let fut = unsafe {
///             peer_connection::create_offer(self.handle.get())
///         }.unwrap();
///
///         unsafe { FutureFromDart::execute(fut) }
///             .await
///             .map_err(RtcPeerConnectionError::CreateOfferFailed)
///     }
///
///     pub async fn create_answer(&self) -> RtcPeerConnectionResult<String> {
///         let fut = unsafe {
///             peer_connection::create_offer(self.handle.get())
///         }.unwrap();
///
///         unsafe { FutureFromDart::execute(fut) }
///             .await
///             .map_err(RtcPeerConnectionError::CreateAnswerFailed)
///     }
/// }
///
/// pub enum RtcPeerConnectionError {
///     CreateOfferFailed(platform::Error),
///     CreateAnswerFailed(platform::Error),
/// }
///
/// pub type RtcPeerConnectionResult<T> = Result<T, RtcPeerConnectionError>;
/// ```
///
/// ## Dart side code
///
/// Also, you need to call registration functions on Dart side:
///
/// ```dart
/// dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
///         'register_peer_connection')(
///     createOffer_native,
///     createAnswer_native,
/// );
/// ```
#[proc_macro_attribute]
pub fn dart_bridge(args: TokenStream, input: TokenStream) -> TokenStream {
    dart_bridge::expand(args.into(), input.into())
        .map_or_else(|e| e.to_compile_error().into(), Into::into)
}
