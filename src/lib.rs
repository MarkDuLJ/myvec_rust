use std::ptr::NonNull;

pub struct MyVec<T>{
    ptr: NonNull<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = Vec::new();
        vec.push(5usize);
        vec.push(5);
        vec.push(5);
        vec.push(5);
        vec.push(5);

        assert_eq!(vec.capacity(),8);
        assert_eq!(vec.len(),5);
    }
}
