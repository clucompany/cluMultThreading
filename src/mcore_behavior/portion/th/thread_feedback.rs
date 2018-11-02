

#[derive(Debug)]
pub struct EndThreadInfo {
     pub num: usize,
     pub success: usize,
}

impl EndThreadInfo {
     #[inline]
     pub const fn new(num: usize, success: usize) -> Self {
          Self {
               num: num,
               success: success,
          }
     }
}



