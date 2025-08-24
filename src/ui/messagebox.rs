
#[repr(C)]
pub enum DialogType {
    Error,
    Warning,
    Information,
    Question,
}

#[link(name = "libgeomint", kind = "dylib")]
unsafe extern "C" {
   pub fn X0JA_NXMESSAGE_BOX_show(
        title: *const u8,
        msgboxType: DialogType,
        message: *const u8,
        result: &i32,
    ) -> i32;
}

/// utf8版本消息框宏
/// 
/// # 示例
/// ```rust
/// nx_msgbox!("标题11", DialogType::Information, "abc{}中文", 666);
/// ```
/// 
/// # 参数
/// - `$title`: 消息框标题字符串
/// - `$msgbox_type`: 消息框类型，如`DialogType::Information`
/// - `$($arg)*`: 格式化的消息内容，支持`format!`语法
#[macro_export]
macro_rules! nx_msgbox {
    ($title:expr, $msgbox_type:expr, $($arg:tt)*) => {{
       
        let mut message_string = format!($($arg)*);
        message_string.push('\0');
            unsafe {
                unsafe { nxopen_rs::jam::JAM_start_wrapped_call()};
                let mut result = 0;
                let _ =nxopen_rs::messagebox::X0JA_NXMESSAGE_BOX_show(
                    nxopen_rs::s!($title),
                    $msgbox_type,
                    message_string.as_ptr(),
                    &mut result
                );
                result
            }
    }};
    ($($arg:tt)*) => {{
        $crate::nx_msgbox!("Message", nxopen_rs::messagebox::DialogType::Information, $($arg)*)
    }};
}
/// ANSI版本消息框宏
/// 
/// 适用于低版本NX系统，使用ANSI编码处理字符串
/// 
/// # 示例
/// ```rust
/// nx_msgbox_ansi!("标题11", DialogType::Information, "abc{}中文", 666);
/// ```
/// 
/// # 参数
/// - `$title`: 消息框标题字符串
/// - `$msgbox_type`: 消息框类型，如`DialogType::Information`
/// - `$($arg)*`: 格式化的消息内容，支持`format!`语法
#[macro_export]
macro_rules! nx_msgboxa {
    ($title:expr, $msgbox_type:expr, $($arg:tt)*) => {{
       
        let message_string = format!($($arg)*);
        let title_cstr = nxopen_rs::cstr::NXstrA::new($title);
        let message_cstr = nxopen_rs::cstr::NXstrA::new(message_string.as_str());
        
        if !title_cstr.get_ptr().is_null() && !message_cstr.get_ptr().is_null() {
            unsafe {
                unsafe { nxopen_rs::jam::JAM_start_wrapped_call()};
                let mut result = 0;
                let _ =nxopen_rs::messagebox::X0JA_NXMESSAGE_BOX_show(
                    title_cstr.get_ptr(),
                    $msgbox_type,
                    message_cstr.get_ptr(),
                    &mut result
                );
            }
        }
    }};
    ($($arg:tt)*) => {{
        $crate::nx_msgboxa!("Message", nxopen_rs::messagebox::DialogType::Information, $($arg)*)
    }};
}

