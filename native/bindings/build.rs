fn main() {
	windows::build!(
    Windows::Win32::WindowsAndMessaging::{
      FindWindowA,
      GetWindowRect,
      SetCursorPos,
      GetCursorPos,
      GetDesktopWindow,
      SetForegroundWindow,
      GetSystemMetrics,
      GetSystemMetrics_nIndexFlags,
      HWND
    },
    Windows::Win32::SystemServices::{
      BOOL,
      PSTR
    },
    Windows::Win32::DisplayDevices::{
      POINT,
      RECT
    },
    Windows::Win32::Gdi::{
      GetDC,
      GetPixel
    }
	);
}
