extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate proc_macro_hack;
extern crate TCFSMFTTTCS;

use proc_macro_hack::proc_macro_hack;
use quote::quote;
use TCFSMFTTTCS::TysCountingFSMForTyTtClusterSeq;

// TODO: Switch out from proc_macro_hack and edit documentation when
//"error: procedural macros cannot be expanded to expressions" is resloved.
// note: see issue #54727 <https://github.com/rust-lang/rust/issues/54727> for more information
#[proc_macro_hack]
pub fn unsafe_count_tys_in_ty_ts(input :proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tys_count :usize = proc_macro2::TokenStream::from(input)
        .into_iter()
        .fold(TysCountingFSMForTyTtClusterSeq::<usize>::new(), |ty_counting_fsm, tt| unsafe {
            // proof of safety
            ty_counting_fsm.unsafe_transition(tt)
        })
        .get_count();
    let expanded_tt  = quote!{#tys_count};
    proc_macro::TokenStream::from(expanded_tt)
}