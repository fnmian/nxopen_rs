#[link(name = "kernel32", kind = "dylib")]
unsafe extern "C" {
    pub fn LoadLibraryA(dllname: *const u8) -> usize;
    pub fn GetProcAddress(dll: usize, fn_name: *const u8) -> usize;
    pub fn FreeLibrary(dll: usize) -> i32;
    pub fn GetLastError() -> u32;
    pub fn GetCurrentProcess() -> usize;
    pub fn GetCurrentThread() -> usize;
    pub fn GetCurrentThreadId() -> u32;
    pub fn GetModuleHandleA(dllname: *const u8) -> usize;
    pub fn lstrlen(str: *const u8) -> usize;
    pub fn HeapAlloc(heap: usize, flags: u32, size: usize) -> usize;
    pub fn HeapFree(heap: usize, flags: u32, ptr:usize) -> i32;
    pub fn GetProcessHeap() -> usize;
    pub fn WideCharToMultiByte(
        CodePage: u32,
        dwFlags: u32,
        lpWideCharStr: *const u8,
        cchWideChar: i32,
        lpMultiByteStr: *const u8,
        cbMultiByte: i32,
        lpDefaultChar: *const u8,
        lpUsedDefaultChar: *const bool,
    ) -> i32;
}

unsafe extern "C" {
    pub fn malloc(size: usize) -> *mut u8;
    pub fn free(p: *const u8);
    pub fn realloc(ptr:*mut u8,size: usize) -> *mut u8;
    pub fn strlen(cs: *const u8) -> usize;

}