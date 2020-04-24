use proc_macro_hack::proc_macro_hack;
#[proc_macro_hack]
use unsafe_count_tys_in_ty_ts::unsafe_count_tys_in_ty_ts;

macro_rules! declare_const_with_number_of_types {
    ($const_name:ident[$($ty:ty),*]) => {
        const $const_name :usize = unsafe_count_tys_in_ty_ts!($($ty),*);
    };
}

declare_const_with_number_of_types!(NUM[usize,usize,usize]);
fn main() {
    assert_eq!(3, NUM);
}
