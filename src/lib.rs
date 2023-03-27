pub mod   n_leq_8 { pub type Uxx =   u8; }
pub mod  n_leq_16 { pub type Uxx =  u16; }
pub mod  n_leq_32 { pub type Uxx =  u32; }
pub mod  n_leq_64 { pub type Uxx =  u64; }
pub mod n_leq_128 { pub type Uxx = u128; }

pub mod  count_leq_2_15 { pub type Iyy = i16; }
pub mod  count_leq_2_31 { pub type Iyy = i32; }
pub mod  count_leq_2_63 { pub type Iyy = i32; }

pub mod visits_leq_2_32 { pub type Uzz = u32; }
pub mod visits_leq_2_64 { pub type Uzz = u64; }

pub mod uniform_2 {
    pub const P: [i32; 2] = [1, 1];
}

pub mod uniform_3 {
    pub const P: [i32; 3] = [1, 1, 1];
}

pub mod uniform_4 {
    pub const P: [i32; 4] = [1, 1, 1, 1];
}

pub mod uniform_5 {
    pub const P: [i32; 5] = [1, 1, 1, 1, 1];
}

pub mod r_3_3 {
    pub const N: usize = 5;
    pub const S: [usize; 2] = [3, 3];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_8::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_3_4 {
    pub const E: usize = 8*(8-1)/2;
    pub const S: [usize; 2] = [3, 4];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_32::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_3_5 { // todo!()
    pub const N: usize = 14;
    pub const S: [usize; 2] = [3, 5];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_32::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_3_6 {
    pub const N: usize = 17;
    pub const S: [usize; 2] = [3, 6];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_32::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_3_7 {
    pub const N: usize = 22;
    pub const S: [usize; 2] = [3, 7];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_32::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_3_8 {
    pub const N: usize = 27;
    pub const S: [usize; 2] = [3, 8];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_32::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_3_9 { // todo!()
    pub const N: usize = 33;
    pub const S: [usize; 2] = [3, 9];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_64::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_3_10 { // todo!()
    pub const N: usize = 39; // tigher bound in literature?
    pub const S: [usize; 2] = [3, 10];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_64::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_3_11 {
    pub const N: usize = 39; // todo!("check another table");
    pub const S: [usize; 2] = [3, 11];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_64::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_4_4 {
    pub const N: usize = 17;
    pub const S: [usize; 2] = [4, 4];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_64::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_4_5 {
    pub const N: usize = 24;
    pub const S: [usize; 2] = [4, 5];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_32::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_4_6 { // todo!()
    pub const N: usize = 32; // [36--40]
    pub const S: [usize; 2] = [4, 6];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_64::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_4_7 { // todo!()
    pub const N: usize = 40; // [49--58]
    pub const S: [usize; 2] = [4, 7];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_64::*, count_leq_2_31::*, visits_leq_2_32::*};
}

pub mod r_4_8 { // todo!()
    pub const N: usize = 35; // [49--79]
    pub const S: [usize; 2] = [4, 8];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_64::*, count_leq_2_31::*, visits_leq_2_32::*};
}

pub mod r_5_5 { // todo!()
    pub const N: usize = 42; // [43--48]
    pub const S: [usize; 2] = [5, 5];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_64::*, count_leq_2_31::*, visits_leq_2_64::*};
}

pub mod r_5_6 {
    pub const N: usize = 58;
    pub const S: [usize; 2] = [5, 6];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_64::*, count_leq_2_31::*, visits_leq_2_64::*};
}

pub mod r_6_6 {
    pub const N: usize = 102;
    pub const S: [usize; 2] = [6, 6];
    pub use crate::uniform_2::*;
    pub use crate::{n_leq_128::*, count_leq_2_31::*, visits_leq_2_64::*};
}

pub mod r_3_3_3 {
    pub const N: usize = 16;
    pub const S: [usize; 3] = [3, 3, 3];
    pub use crate::uniform_3::*;
    pub use crate::{n_leq_16::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod r_3_3_3_3 {
    pub const N: usize = 42;
    pub const S: [usize; 4] = [3, 3, 3, 3];
    pub use crate::uniform_4::*;
    pub use crate::{n_leq_64::*, count_leq_2_15::*, visits_leq_2_32::*};
}

pub mod colored_graph;
pub mod display;
pub mod action_matrix;
pub mod search_maps;
pub mod learning_loop;
