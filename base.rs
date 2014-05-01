pub mod option {
    pub enum Option<T> {
        Some(T),
        None
    }
}

pub mod kinds {
    #[lang="sized"]
    pub trait Sized {}
}

pub mod fail {
    pub fn abort() -> ! { loop {} }

    #[inline]
    #[lang="fail_bounds_check"]
    pub fn fail_bounds_check(_: *u8, _: uint, _: uint, _: uint) -> ! {
        abort()
    }

    #[inline]
    #[lang="fail_"]
    pub fn fail_(_: *u8, _: *u8, _: uint) -> ! {
        abort()
    }
}

pub mod iter {
    use super::option::*;
    
    pub trait Iterator<A> {
        fn next(&mut self) -> Option<A>;
    }
    
    pub struct Range {
        state: int,
        stop: int
    }

    pub fn range(start: int, stop: int) -> Range { Range { state: start, stop: stop } }

    impl Iterator<int> for Range {
        fn next(&mut self) -> Option<int> {
            if self.state < self.stop {
                let result = Some(self.state);
                self.state += 1;
                result
            } else {
                None
            }
        }
    }
}

pub mod prelude {
    pub use super::option::{ Option, Some, None };
    pub use super::kinds::*;
    pub use super::fail::*;
    pub use super::iter::{ Iterator, Range, range };
}
