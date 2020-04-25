use proc_macro_hack::proc_macro_hack;
#[proc_macro_hack]
use unsafe_count_tys_in_ty_ts::unsafe_count_tys_in_ty_ts;
#[proc_macro_hack]
use identify_tts::identify_tts;

macro_rules! declare_const_with_number_of_types {
    ($const_name:ident[$($ty:ty),*]) => {
        const $const_name :usize = unsafe_count_tys_in_ty_ts!($($ty:ty)*);
    };
}

macro_rules! test_tys {
    ($const_name:ident[$($ty:ty),*]) => {
        let $const_name = identify_tts!($($ty:ty)*);
    };
}

//declare_const_with_number_of_types!(NUM[Vec<usize>,usize,usize]);
fn main() {
    //assert_eq!(3, NUM);
    //println!("{}", NUM);
    test_tys!(num[Vec<usize>,usize,usize]);
    print!("{}", num.len());
    println!("{:?}", num);
}
