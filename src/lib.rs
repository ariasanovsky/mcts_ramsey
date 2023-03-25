pub mod   n_leq_8 { pub type Uxx =   u8; }
pub mod  n_leq_16 { pub type Uxx =  u16; }
pub mod  n_leq_32 { pub type Uxx =  u32; }
pub mod  n_leq_64 { pub type Uxx =  u64; }
pub mod n_leq_128 { pub type Uxx = u128; }

pub mod  count_leq_2_31 { pub type Iyy = i32; }
pub mod  count_leq_2_63 { pub type Iyy = i32; }

pub mod visits_leq_2_32 { pub type Uzz = u32; }
pub mod visits_leq_2_64 { pub type Uzz = u64; }

pub mod s_3_3 { pub const S: [usize; 2] = [3, 3]; }
pub mod s_3_4 { pub const S: [usize; 2] = [3, 4]; }
pub mod s_3_5 { pub const S: [usize; 2] = [3, 5]; }
pub mod s_4_4 { pub const S: [usize; 2] = [4, 4]; }
pub mod s_4_5 { pub const S: [usize; 2] = [4, 5]; }
pub mod s_5_5 { pub const S: [usize; 2] = [5, 5]; }

pub mod s_3_3_3 { pub const S: [usize; 3] = [3, 3, 3]; }

pub mod r_3_3 {
    pub const N: usize = 5;
    pub use crate::{n_leq_8::*, count_leq_2_31::*, s_3_3::*, visits_leq_2_32::*};
}

pub mod r_3_4 {
    pub const N: usize = 8;
    pub use crate::{n_leq_32::*, count_leq_2_31::*, s_3_4::*, visits_leq_2_32::*};
}

pub mod r_4_4 {
    pub const N: usize = 17;
    pub use crate::{n_leq_32::*, count_leq_2_31::*, s_4_5::*, visits_leq_2_32::*};
}

pub mod r_3_3_3 {
    pub const N: usize = 16;
    pub use crate::{n_leq_16::*, count_leq_2_31::*, s_3_3_3::*, visits_leq_2_32::*};
}

pub mod colored_graph;
pub mod action_matrix;
pub mod search_maps;
pub mod learning_loop;
