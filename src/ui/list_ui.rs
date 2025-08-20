
#[link(name = "libsyss", kind = "dylib")]
unsafe extern "C" {
   #[link_name = "?listUIprintf@@YAXPEBDZZ"]
    pub fn list_uiprintf(format: *const u8, ...);
}


/// NX 环境下的打印宏，类似标准库的 println!
/// 
/// 用于在 NX 系统中输出格式化字符串，自动在末尾添加换行符
/// 支持标准 format! 语法的所有格式化方式
/// 
/// # 用法示例
/// ```rust
/// let i = 666;
/// nx_println!("测试，地址{:p}", &i);
/// ```
/// 
/// # 特点
/// - 自动在输出内容末尾添加换行符
/// - 支持所有标准格式化标记（如 {:p} 用于指针地址）
/// - 内部处理字符串到 NX 兼容的 C 字符串转换
#[macro_export]
macro_rules! nx_println {
($($arg:tt)*) => {{
    let mut formatted_string = format!($($arg)*);
    formatted_string.push('\n');
    formatted_string.push('\0');
    unsafe {
       nxopen_rs::list_ui::list_uiprintf(formatted_string.as_ptr());
        }
    }};
}