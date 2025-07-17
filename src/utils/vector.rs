use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::{self};

use crate::syss::error_internal;
use crate::winapi::{free, malloc, realloc};
///C风格动态数组
#[repr(C)]
pub struct CVec<T> {
    data: *mut T,
    last: *mut T,
    end: *mut T,
    _marker: PhantomData<T>,
}

impl<T> CVec<T> {
    pub fn new() -> Self {
        CVec {
            data: ptr::null_mut(),
            last: ptr::null_mut(),
            end: ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        if cap == 0 {
            return Self::new();
        }

        // 处理零大小类型 (ZST)
        let elem_size = size_of::<T>();
        if elem_size == 0 {
            return CVec {
                data: ptr::null_mut(),
                last: ptr::null_mut(),
                end: (usize::MAX as *mut T),
                _marker: PhantomData,
            };
        }

        unsafe {
            let byte_size = cap * elem_size;
            let new_data = malloc(byte_size) as *mut T;

            if new_data.is_null() {
                 error_internal(
                "Cvec\0".as_ptr(),
                line!(),
                "Failed to allocate memory\0".as_ptr(),
            );
            }

            CVec {
                data: new_data,
                last: new_data,
                end: new_data.add(cap),
                _marker: PhantomData,
            }
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        if vec.is_empty() {
            return Self::new();
        }

        // 直接接管 Vec 的内存
        let mut vec = vec;
        let data = vec.as_mut_ptr();
        let len = vec.len();
        let cap = vec.capacity();

        // 防止 Vec 被丢弃时释放内存
        std::mem::forget(vec);

        unsafe {
            CVec {
                data: data,
                last: data.add(len),
                end: data.add(cap),
                _marker: PhantomData,
            }
        }
    }

    pub fn len(&self) -> usize {
        if size_of::<T>() == 0 {
            // ZST 特殊处理
            (self.end as usize).saturating_sub(self.last as usize)
        } else {
            unsafe { self.last.offset_from(self.data) as usize }
        }
    }

    pub fn capacity(&self) -> usize {
        if size_of::<T>() == 0 {
            // ZST 容量无限
            usize::MAX
        } else {
            unsafe { self.end.offset_from(self.data) as usize }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn clear(&mut self) {
        // 析构所有元素（非 ZST 类型）
        if size_of::<T>() != 0 {
            unsafe {
                let mut current = self.data;
                while current != self.last {
                    ptr::drop_in_place(current);
                    current = current.add(1);
                }
            }
        }
        self.last = self.data; // 重置指针
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            unsafe {
                self.last = self.last.sub(1);
                Some(ptr::read(self.last))
            }
        }
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data, self.len()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.len()) }
    }
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
    pub fn push(&mut self, item: T) {
        // 处理零大小类型
        if size_of::<T>() == 0 {
            // 只需更新位置计数器
            self.last = (self.last as usize).wrapping_add(1) as *mut T;
            return;
        }

        if self.last == self.end {
            self.grow();
        }

        unsafe {
            ptr::write(self.last, item);
            self.last = self.last.add(1);
        }
    }

    fn grow(&mut self) {
        let old_cap = self.capacity();
        let new_cap = if old_cap == 0 { 4 } else { old_cap * 2 };

        unsafe {
            let elem_size = size_of::<T>();
            let new_byte_size = new_cap * elem_size;

            let new_data = if old_cap == 0 {
                malloc(new_byte_size) as *mut T
            } else {
                realloc(self.data as _, new_byte_size) as *mut T
            };

            if new_data.is_null() {
                error_internal(
                "Cvec\0".as_ptr(),
                line!(),
                "Failed to allocate memory\0".as_ptr(),
            );
            }

            self.data = new_data;
            self.last = new_data.add(old_cap);
            self.end = new_data.add(new_cap);
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            ptr: self.data,
            end: self.last,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            ptr: self.data,
            end: self.last,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for CVec<T> {
    fn drop(&mut self) {
        if size_of::<T>() == 0 || self.data.is_null() {
            return;
        }

        // 析构所有元素
        unsafe {
            let mut current = self.data;
            while current != self.last {
                ptr::drop_in_place(current);
                current = current.add(1);
            }
        }

        // 释放内存
        let cap = self.capacity();
        if cap > 0 {
            unsafe {
                free(self.data as _);
            }
        }
    }
}

// 不可变迭代器
pub struct Iter<'a, T> {
    ptr: *const T,
    end: *const T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else {
            unsafe {
                let result = &*self.ptr;
                self.ptr = self.ptr.add(1);
                Some(result)
            }
        }
    }
}

// 可变迭代器
pub struct IterMut<'a, T> {
    ptr: *mut T,
    end: *mut T,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else {
            unsafe {
                let result = &mut *self.ptr;
                self.ptr = self.ptr.add(1);
                Some(result)
            }
        }
    }
}