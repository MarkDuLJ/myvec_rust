use std::{alloc::{self, Layout}, ptr::NonNull};

pub struct MyVec<T>{
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,//space on heap
}

impl<T> MyVec<T> {
    pub fn new() -> Self{
        Self { 
            ptr: NonNull::dangling(),//dangling pointer, but not null 
            len: 0, 
            capacity: 0 
        }
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(),0,"no zero size types");

        if self.capacity == 0 {
            let layout = Layout::array::<T>(4)
                                    .expect("Allocate space failed");//[T; n]create 4 any kind of items
            let ptr = unsafe {alloc::alloc(layout)} as *mut T;
            let ptr = NonNull::new(ptr).expect("can't allocate memory");
            unsafe {
                ptr.as_ptr().write(item);
            }
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        }else if self.len < self.capacity {
            unsafe {
                self.ptr.as_ptr().add(self.len).write(item);
                self.len += 1;
            }
        }else {
            let new_cap = self.capacity.checked_mul(2).expect("capacity wrapped");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(size % align).expect("Allocate failed");
            let ptr = unsafe {
              let layout = alloc::Layout::from_size_align_unchecked(
                size, 
                align,
                );  
                let  new_size = std::mem::size_of::<T>() * new_cap;
                
            
               let ptr =  alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
               let ptr = NonNull::new( ptr as *mut T).expect("msg");
               ptr.as_ptr().add(self.len).write(item);
               ptr
        };
        self.ptr = ptr;
        self.capacity = new_cap;
        self.len += 1;
        
    }
    }

    pub fn capacity(&self) -> usize{
        self.capacity
    }

    pub fn len(&self) -> usize{
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec:MyVec<usize> = MyVec::new();
        vec.push(5usize);
        vec.push(5);
        vec.push(5);
        vec.push(5);
        vec.push(5);

        assert_eq!(vec.capacity(),8);
        assert_eq!(vec.len(),5);
    }
}
