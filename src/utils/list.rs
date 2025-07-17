use std::marker::PhantomData;
use std::mem;
use std::ptr;

use crate::syss::error_internal;
use crate::winapi::free;
use crate::winapi::malloc;

/// C风格链表
#[repr(C)]
pub struct List<T>
where
    T: Sized,
{
    head: *mut ListNode<T>,
    size: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        unsafe {
            let ptr = malloc(mem::size_of::<ListNode<T>>()) as *mut ListNode<T>;
            if ptr.is_null() {
                error_internal(
                "List\0".as_ptr(),
                line!(),
                "Failed to allocate memory\0".as_ptr(),
            );
            }
            ptr.write(ListNode {
                next: ptr,
                prev: ptr,
                data: mem::MaybeUninit::uninit().assume_init(),
            });
            
            List { head: ptr, size: 0 }
        }
    }

    pub fn clear(&mut self) {
        if self.head.is_null() {
            return;
        }
        
        unsafe {
            let mut current = (*self.head).next;
            
            while current != self.head {
                let next = (*current).next;
                ptr::drop_in_place(&mut (*current).data);
                free(current as *mut _);
                current = next;
            }
            
            (*self.head).next = self.head;
            (*self.head).prev = self.head;
            self.size = 0;
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.insert_before(self.head, value);
    }

    pub fn push_front(&mut self, value: T) {
        self.insert_after(self.head, value);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        
        unsafe {
            let node = (*self.head).prev;
            let data = ptr::read(&(*node).data);
            self.remove(node);
            Some(data)
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        
        unsafe {
            let node = (*self.head).next;
            let data = ptr::read(&(*node).data);
            self.remove(node);
            Some(data)
        }
    }

    fn insert_before(&mut self, node: *mut ListNode<T>, value: T) {
        unsafe {
            let new_node = malloc(mem::size_of::<ListNode<T>>()) as *mut ListNode<T>;
            if new_node.is_null() {
                 error_internal(
                "List\0".as_ptr(),
                line!(),
                "Failed to allocate memory\0".as_ptr(),
            );
            }
            
            *new_node = ListNode {
                next: node,
                prev: (*node).prev,
                data: value,
            };
            
            (*(*node).prev).next = new_node;
            (*node).prev = new_node;
            self.size += 1;
        }
    }

    fn insert_after(&mut self, node: *mut ListNode<T>, value: T) {
        unsafe {
            let new_node = malloc(mem::size_of::<ListNode<T>>()) as *mut ListNode<T>;
            if new_node.is_null() {
                 error_internal(
                "List\0".as_ptr(),
                line!(),
                "Failed to allocate memory\0".as_ptr(),
            );
            }
            
            *new_node = ListNode {
                next: (*node).next,
                prev: node,
                data: value,
            };
            
            (*(*node).next).prev = new_node;
            (*node).next = new_node;
            self.size += 1;
        }
    }

    fn remove(&mut self, node: *mut ListNode<T>) {
        if node == self.head {
            return;
        }
        
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
            ptr::drop_in_place(&mut (*node).data);
            free(node as *mut _);
        }
        
        self.size -= 1;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                current: (*self.head).next,
                sentinel: self.head,
                _marker: PhantomData,
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                current: (*self.head).next,
                sentinel: self.head,
                _marker: PhantomData,
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        if self.head.is_null() {
            return;
        }
        
        self.clear();
        unsafe { free(self.head as *mut _) };
        self.head = ptr::null_mut();
        self.size = 0;
    }
}

unsafe impl<T: Send> Send for List<T> {}
unsafe impl<T: Sync> Sync for List<T> {}

#[repr(C)]
pub(crate) struct ListNode<T>
where
    T: Sized,
{
    next: *mut ListNode<T>,
    prev: *mut ListNode<T>,
    data: T,
}

pub struct Iter<'a, T> {
    current: *const ListNode<T>,
    sentinel: *const ListNode<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.sentinel {
            None
        } else {
            let data = unsafe { &(*self.current).data };
            self.current = unsafe { (*self.current).next };
            Some(data)
        }
    }
}

pub struct IterMut<'a, T> {
    current: *mut ListNode<T>,
    sentinel: *mut ListNode<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.sentinel {
            None
        } else {
            let data = unsafe { &mut (*self.current).data };
            self.current = unsafe { (*self.current).next };
            Some(data)
        }
    }
}