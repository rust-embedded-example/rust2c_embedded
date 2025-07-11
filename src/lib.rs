#![no_std]

use core::panic::PanicInfo;

/// Panic handler - 在 no_std 环境中必需的
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// add函数：计算两个32位整数的和
/// 
/// # 参数
/// - `a`: 第一个整数
/// - `b`: 第二个整数
/// 
/// # 返回值
/// 返回a和b的和
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// add_u32函数：计算两个无符号32位整数的和
/// 
/// # 参数
/// - `a`: 第一个无符号整数
/// - `b`: 第二个无符号整数
/// 
/// # 返回值
/// 返回a和b的和
#[unsafe(no_mangle)]
pub extern "C" fn add_u32(a: u32, b: u32) -> u32 {
    a + b
}

/// add_f32函数：计算两个32位浮点数的和
/// 
/// # 参数
/// - `a`: 第一个浮点数
/// - `b`: 第二个浮点数
/// 
/// # 返回值
/// 返回a和b的和
#[unsafe(no_mangle)]
pub extern "C" fn add_f32(a: f32, b: f32) -> f32 {
    a + b
}

/// add_u16函数：计算两个16位无符号整数的和
/// 
/// # 参数
/// - `a`: 第一个16位无符号整数
/// - `b`: 第二个16位无符号整数
/// 
/// # 返回值
/// 返回a和b的和
#[unsafe(no_mangle)]
pub extern "C" fn add_u16(a: u16, b: u16) -> u16 {
    a + b
}

/// add_u8函数：计算两个8位无符号整数的和
/// 
/// # 参数
/// - `a`: 第一个8位无符号整数
/// - `b`: 第二个8位无符号整数
/// 
/// # 返回值
/// 返回a和b的和
#[unsafe(no_mangle)]
pub extern "C" fn add_u8(a: u8, b: u8) -> u8 {
    a + b
} 