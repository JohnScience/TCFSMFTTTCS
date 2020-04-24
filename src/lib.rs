#![allow(non_snake_case)]
#![feature(external_doc)]
//#![allow(dead_code)]

#![doc(include = "../doc/struct TCFSMFTTTCS/description")]
#![doc(include = "../doc/ToC")] // Table of Contents
#![doc(include = "../doc/definitions, signs, and properties")]
#![doc(include = "../doc/md-links")]
use proc_macro2::{TokenStream, TokenTree, TokenTree::{Punct, Ident}};
// TODO: replace One trait because it carries the semantics of the multiplicative identity.
// Suggest num_traits to add saturating_succ(), overflowing_succ(), and unwarpped_succ()
// functions for natural numbers.
pub use num_traits::One;

#[doc(include = "../doc/struct TCFSMFTTTCS/description")]
#[doc(include = "../doc/struct TCFSMFTTTCS/md-links")]
#[derive(Copy, Clone)]
pub struct TysCountingFSMForTyTtClusterSeq<T> {
    prev_tt_was_colon :bool,
    count :T,
}

impl <T> TysCountingFSMForTyTtClusterSeq<T> {
    #[doc(include = "../doc/struct TCFSMFTTTCS/impl/description")]
    /// # Examples
    /// 
    /// Basic usage:
    /// ```rust
    /// extern crate proc_macro;
    /// extern crate proc_macro2;
    /// extern crate quote;
    /// extern crate proc_macro_hack;
    /// extern crate TCFSMFTTTCS;
    /// 
    /// use quote::quote;
    /// use TCFSMFTTTCS::TysCountingFSMForTyTtClusterSeq;
    /// 
    /// // TODO: Switch out from proc_macro_hack and edit documentation when
    /// //"error: procedural macros cannot be expanded to expressions" is resloved.
    /// // note: see issue #54727 <https://github.com/rust-lang/rust/issues/54727> for more information
    /// #[proc_macro_hack]
    /// pub unsafe fn unsafe_count_tys_in_ty_ts() {
    ///     let tys_count :usize = proc_macro2::TokenStream::from(input)
    ///         .into_iter()
    ///         .fold(TysCountingFSMForTyTtClusterSeq::<usize>::new(), |ty_counting_fsm, tt| unsafe {
    ///             // proof of safety
    ///             ty_counting_fsm.unsafe_transition(tt)
    ///         })
    ///         .get_count();
    ///     let expanded_tt  = quote!{#tts_count};
    ///     proc_macro::TokenStream::from(expanded_tt)
    /// }
    /// 
    /// ```
    /// 
    /// [:ty TokenTree cluster]: ./index.html#ty-tokentree-cluster
    pub fn get_count(self) -> T {
        self.count
    }
}

impl <T> TysCountingFSMForTyTtClusterSeq<T>
where T :std::default::Default
{
    /// Constructs a new [TysCountingFSMForTyTtClusterSeq]
    /// 
    /// Newly constructed [TysCountingFSMForTyTtClusterSeq] returns [Default]::[default]() with
    /// [TysCountingFSMForTyTtClusterSeq::\<T\>::get_count]()
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```rust
    /// extern crate TCFSMFTTTCS;
    /// use TCFSMFTTTCS::TysCountingFSMForTyTtClusterSeq;
    /// 
    /// let TCFSMFTTTCS = TysCountingFSMForTyTtClusterSeq::<usize>::new();
    /// let initial_value :usize = TCFSMFTTTCS.get_count();
    /// assert_eq!(0, initial_value);
    /// ```
    /// 
    /// [TysCountingFSMForTyTtClusterSeq]: ./struct.TysCountingFSMForTyTtClusterSeq.html
    /// [usize]: https://doc.rust-lang.org/std/primitive.usize.html
    /// [TysCountingFSMForTyTtClusterSeq::<T>::get_count]: ./struct.TysCountingFSMForTyTtClusterSeq.html#method.get_count
    /// [Default]: https://doc.rust-lang.org/std/default/trait.Default.html
    /// [default]: https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default
    pub fn new () -> Self {
        Self {
            prev_tt_was_colon: false,
            count: T::default(),
        }
    }
}

impl <T>TysCountingFSMForTyTtClusterSeq<T>
where T: std::ops::AddAssign + One
{
    /// Returns a mutable reference to the [TysCountingFSMForTyTtClusterSeq] in the transitioned 
    /// (modified) state. Internally, [counts] [:ty TokenTree cluster]s in the given
    /// [TokenTree sequence] assuming it is a [:ty TokenTree sequence] and using 
    /// [Signs of :ty TokenTree clusters` boundaries in :ty TokenTree sequence].
    /// Guarantees to correctly maintain count of encountered [:ty TokenTree cluster]s only for 
    /// traversed paths that are prefixes of [:ty TokenTree sequence].
    /// 
    /// # Arguments
    /// * `tt` - next [TokenTree] on the prefix-path of a [:ty TokenTree cluster sequence]
    /// 
    /// # Safety
    /// 
    /// `tt` must be on the prefix-path of a [:ty TokenTree cluster sequence]. For example,
    /// any prefix of [:ty TokenStream]
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```rust
    /// extern crate proc_macro;
    /// extern crate proc_macro2;
    /// extern crate quote;
    /// extern crate proc_macro_hack;
    /// extern crate TCFSMFTTTCS;
    /// 
    /// use quote::quote;
    /// use TCFSMFTTTCS::TysCountingFSMForTyTtClusterSeq;
    /// 
    /// // TODO: Switch out from proc_macro_hack and edit documentation when
    /// //"error: procedural macros cannot be expanded to expressions" is resloved.
    /// // note: see issue #54727 <https://github.com/rust-lang/rust/issues/54727> for more information
    /// #[proc_macro_hack]
    /// pub unsafe fn unsafe_count_tys_in_ty_ts() {
    ///     let tys_count :usize = proc_macro2::TokenStream::from(input)
    ///         .into_iter()
    ///         .fold(TysCountingFSMForTyTtClusterSeq::<usize>::new(), |ty_counting_fsm, tt| unsafe {
    ///             // proof of safety
    ///             ty_counting_fsm.unsafe_transition(tt)
    ///         })
    ///         .get_count();
    ///     let expanded_tt  = quote!{#tts_count};
    ///     proc_macro::TokenStream::from(expanded_tt)
    /// }
    /// 
    /// ```
    /// 
    /// [counts]: ./struct.TysCountingFSMForTyTtClusterSeq.html#method.get_count
    /// [Signs of :ty TokenTree clusters` boundaries in :ty TokenTree sequence]: index.html#signs-of-ty-tokentree-clusters-boundaries-in-ty-tokentree-sequence
    /// [TokenTree sequence]: ./index.html#ty-tokentree-sequence
    /// [TysCountingFSMForTyTtClusterSeq]: ./struct.TysCountingFSMForTyTtSeq.html
    /// [:ty TokenTree cluster]: ./index.html#ty-tokentree-cluster
    /// [:ty TokenTree cluster sequence]: ./index.html#ty-tokentree-cluster-sequence
    pub unsafe fn unsafe_transition(mut self, tt :TokenTree) -> Self {
        self.prev_tt_was_colon = match tt {
            Punct(punct) => (punct.as_char() == ':'),
            Ident(ident) => {
                if self.prev_tt_was_colon && ident.to_string() == "ty" { self.count += T::one(); }
                false
            }
            _ => false,
        };
        self
    }

    /// Returns a mutable reference to the [TysCountingFSMForTyTtClusterSeq] in the transitioned 
    /// (modified) state. Internally, [counts] [:ty TokenTree cluster]s in the given
    /// [TokenStream] assuming it is a [:ty TokenStream] and using 
    /// [Signs of :ty TokenTree clusters` boundaries in :ty TokenTree sequence].
    /// Guarantees to correctly maintain count of encountered [:ty TokenTree cluster]s only for 
    /// traversed paths that are prefixes of [:ty TokenTree sequence].
    /// 
    /// # Arguments
    /// * `ts` - presumably, not just a regular [TokenStream], but a [:ty TokenStream]
    /// 
    /// # Safety
    /// 
    /// `ts` must be a [:ty TokenStream]
    /// 
    /// # Example
    /// 
    /// ```rust
    /// extern crate proc_macro;
    /// extern crate proc_macro2;
    /// extern crate quote;
    /// extern crate proc_macro_hack;
    /// extern crate TCFSMFTTTCS;
    /// 
    /// use quote::quote;
    /// use TCFSMFTTTCS::TysCountingFSMForTyTtClusterSeq;
    /// 
    /// // TODO: Switch out from proc_macro_hack and edit documentation when
    /// //"error: procedural macros cannot be expanded to expressions" is resloved.
    /// // note: see issue #54727 <https://github.com/rust-lang/rust/issues/54727> for more information
    /// #[proc_macro_hack]
    /// pub unsafe fn unsafe_count_tys_in_ty_ts() {
    ///     let TCFSMFTTTCS = TysCountingFSMForTyTtClusterSeq::<usize>::new();
    ///     let tys_count :usize = unsafe {
    ///         TCFSMFTTTCS.unsafe_into_iter_transition proc_macro2::TokenStream::from(input)
    ///     }
    ///     let tys_count :usize = proc_macro2::TokenStream::from(input)
    ///         .into_iter()
    ///         .fold(TysCountingFSMForTyTtClusterSeq::<usize>::new(), |ty_counting_fsm, tt| unsafe {
    ///             // proof of safety
    ///             ty_counting_fsm.unsafe_transition(tt)
    ///         })
    ///         .get_count();
    ///     let expanded_tt  = quote!{#tts_count};
    ///     proc_macro::TokenStream::from(expanded_tt)
    /// }
    /// 
    /// ```
    /// 
    /// [TokenStream]: https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html
    /// [counts]: ./struct.TysCountingFSMForTyTtClusterSeq.html#method.get_count
    /// [Signs of :ty TokenTree clusters` boundaries in :ty TokenTree sequence]: index.html#signs-of-ty-tokentree-clusters-boundaries-in-ty-tokentree-sequence
    /// [TokenTree sequence]: ./index.html#ty-tokentree-sequence
    /// [TysCountingFSMForTyTtClusterSeq]: ./struct.TysCountingFSMForTyTtSeq.html
    /// [:ty TokenStream]: ./index.html#ty-tokenstream
    /// [:ty TokenTree cluster]: ./index.html#ty-tokentree-cluster
    /// [:ty TokenTree cluster sequence]: ./index.html#ty-tokentree-cluster-sequence
    pub unsafe fn unsafe_into_iter_transition(self, ts :TokenStream) -> Self {
        ts.into_iter()
            .fold(self, |fsm, tt| {
                
                fsm.unsafe_transition(tt)
            })
    }
}