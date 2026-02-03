
unsafe extern "C" {
    pub fn get_info_cpu(eax: u32, ebx: *mut u32, ecx: *mut u32, edx: *mut u32);
}

