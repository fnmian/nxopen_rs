pub mod load_function;
pub mod winapi;
pub mod cstr;

/// 将字符串转换为以空字符结尾的C风格字符串指针
/// 
/// 用于快速创建兼容C API的字符串指针，自动在输入字符串后添加'\0'终止符
/// 
/// # 示例
/// ```rust
/// let c_str_ptr = s!("hello");
/// // 等价于 "hello\0".as_ptr()
/// ```
#[macro_export]
macro_rules! s {
    ($s:expr) => {{ concat!($s, "\0").as_ptr() }};
}
/// 将字符串转换为以换行符和空字符结尾的C风格字符串指针
/// 
/// 适用于需要自动添加换行的场景，在输入字符串后添加'\n\0'
/// 
/// # 示例
/// ```rust
/// let line_ptr = s_ln!("hello");
/// // 等价于 "hello\n\0".as_ptr()
/// ```
#[macro_export]
macro_rules! s_ln {
    ($s:expr) => {{ concat!($s, "\n\0").as_ptr() }};
}

/// 将C风格字符串指针转换为Rust字符串（String类型）
/// 
/// 安全地处理从C字符串指针到Rust拥有的String的转换，使用from_utf8_lossy处理非UTF-8数据
/// 
/// # 注意
/// 需要传入指针指向有效的以空字符结尾的字符串
/// 
/// # 示例
/// ```rust
/// let c_ptr = s!("test");
/// let rust_str: String = p_ss!(c_ptr);
/// ```
#[macro_export]
macro_rules! p_ss {
    ($ptr:expr) => {
        unsafe{String::from_utf8_lossy(std::slice::from_raw_parts($ptr, crate::winapi::strlen($ptr))).into_owned()}
    };
}
/// 将C风格字符串指针转换为Rust字符串切片（&str）
/// 
/// 直接将C字符串指针转换为Rust字符串切片，不进行内存分配
/// 
/// # 注意
/// - 要求输入指针指向有效的UTF-8编码字符串
/// - 返回的切片生命周期与输入指针的有效性绑定
/// - 不安全操作：未检查UTF-8有效性和指针有效性
/// 
/// # 示例
/// ```rust
/// let c_ptr = s!("test");
/// let rust_str_slice: &str = p_s!(c_ptr);
/// ```
#[macro_export]
macro_rules! p_s {
    ($ptr:expr) => {
       unsafe{std::str::from_utf8_unchecked(std::slice::from_raw_parts ($ptr, nxopen_rs::winapi::strlen($ptr)))}
    };
}
///调试断点
pub fn debugbreak(){
   unsafe {std::arch::asm!("int 3")};
}