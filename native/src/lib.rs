use neon::prelude::*;
use neon::register_module;

use bindings::Windows::Win32::WindowsAndMessaging::*;
use bindings::Windows::Win32::SystemServices::*;
use bindings::Windows::Win32::DisplayDevices::*;
use bindings::Windows::Win32::Gdi::*;

fn screen_size(mut cx: FunctionContext) -> JsResult<JsObject> {
	let width = unsafe { GetSystemMetrics(GetSystemMetrics_nIndexFlags::SM_CXSCREEN) };
	let height = unsafe { GetSystemMetrics(GetSystemMetrics_nIndexFlags::SM_CYSCREEN) };

	let size = Size::new(width, height);

	size.to_js_object(&mut cx)
}

fn window_pos(mut cx: FunctionContext) -> JsResult<JsObject> {
	let title = cx.argument::<JsString>(0)?.value();
	let rect = unsafe { _window_rect(title).unwrap_or_default() };

	let pos = Position::new(rect.top, rect.left, rect.bottom, rect.right);

	pos.to_js_object(&mut cx)
}

fn window_size(mut cx: FunctionContext) -> JsResult<JsObject> {
	let title = cx.argument::<JsString>(0)?.value();
	let rect = unsafe { _window_rect(title).unwrap_or_default() };

	let size = Size::new(rect.right - rect.left, rect.bottom - rect.top);

	size.to_js_object(&mut cx)
}

fn window_focus(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let title = cx.argument::<JsString>(0)?.value();
	let handle = unsafe { _window_get(title).unwrap_or_default() };

	unsafe { SetForegroundWindow(handle) };

	Ok(JsUndefined::new())
}

fn cursor_pos_set(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	let js_object = cx.argument::<JsObject>(0)?;
	let point = Point::from_js_object(&mut cx, &js_object);

	unsafe { SetCursorPos(point.x, point.y); };
	Ok(JsUndefined::new())
}

fn cursor_pos_get(mut cx: FunctionContext) -> JsResult<JsObject> {
	let mut point = POINT::default();
	let success = unsafe {
		let point_ptr: *mut POINT = &mut point;
		GetCursorPos(point_ptr).as_bool()
	};
	if !success {
		panic!();
	}

	let point = Point::new(point.x, point.y);
	point.to_js_object(&mut cx)
}

fn pixel_color_get(mut cx: FunctionContext) -> JsResult<JsString> {
	let js_object = cx.argument::<JsObject>(0)?;
	let point = Point::from_js_object(&mut cx, &js_object);

	let desktop = unsafe { GetDesktopWindow() };
	let dc = unsafe { GetDC(desktop) };
	let color = unsafe {GetPixel(dc, point.x, point.y) };

	Ok(cx.string(format!("{:#x}", color)))
}

fn pixel_color_search(mut cx: FunctionContext) -> JsResult<JsObject> {
	let bounds = match cx.argument_opt(0) {
		Some(arg) => {
			let js_object = arg.downcast::<JsObject>().unwrap();
			Size::from_js_object(&mut cx, &js_object)
		},
		None => {
			let width = unsafe { GetSystemMetrics(GetSystemMetrics_nIndexFlags::SM_CXSCREEN) };
			let height = unsafe { GetSystemMetrics(GetSystemMetrics_nIndexFlags::SM_CYSCREEN) };

			Size::new(width, height)
		}
	};

	let point = Point::new(0, 0);

	point.to_js_object(&mut cx)
}

register_module!(mut m, {
	m.export_function("screenSizeGet", screen_size)?;

	m.export_function("windowPositionGet", window_pos)?;
	m.export_function("windowSizeGet", window_size)?;
	m.export_function("windowFocus", window_focus)?;

	m.export_function("cursorPositionGet", cursor_pos_get)?;
	m.export_function("cursorPositionSet", cursor_pos_set)?;

	m.export_function("pixelColorGet", pixel_color_get)?;
	Ok(())
});

// Internal

unsafe fn _window_get(title: String) -> Option<HWND> {
	let handle = FindWindowA(PSTR::default(), title);
	if handle.is_null() {
		return None;
	}

	Some(handle)
}

unsafe fn _window_rect(title: String) -> Option<RECT> {
	let handle = _window_get(title)?;
	let mut rect = RECT::default();
	let rect_ptr: *mut RECT = &mut rect;

	let success: BOOL = GetWindowRect(handle, rect_ptr);
	if !success.as_bool() {
		return None;
	}

	Some(rect)
}

struct Point {
	x: i32,
	y: i32
}

impl Point {
	fn new(x: i32, y: i32) -> Point {
		Point { x: x, y: y }
	}

	fn from_js_object(cx: &mut FunctionContext, js_object: &JsObject) -> Point {
		let x = js_object.get(cx, "x").unwrap().downcast::<JsNumber>().or_throw(cx).unwrap().value() as i32;
		let y = js_object.get(cx, "y").unwrap().downcast::<JsNumber>().or_throw(cx).unwrap().value() as i32;

		Point::new(x, y)
	}

	fn to_js_object<'a>(self, cx: &mut CallContext<'a, JsObject>) -> JsResult<'a, JsObject> {
		let x = cx.number(self.x);
		let y = cx.number(self.y);

		let js_object = JsObject::new(cx);
		js_object.set(cx, "x", x)?;
		js_object.set(cx, "y", y)?;

		Ok(js_object)
	}
}

struct Position {
	top: i32,
	left: i32,
	bottom: i32,
	right: i32
}

impl Position {
	fn new(top: i32, left: i32, bottom: i32, right: i32) -> Position {
		Position {
			top: top,
			left: left,
			bottom: bottom,
			right: right
		}
	}

	fn to_js_object<'a>(self, cx: &mut CallContext<'a, JsObject>) -> JsResult<'a, JsObject> {
		let top = cx.number(self.top);
		let left = cx.number(self.left);
		let bottom = cx.number(self.bottom);
		let right = cx.number(self.right);

		let js_object = JsObject::new(cx);
		js_object.set(cx, "top", top)?;
		js_object.set(cx, "left", left)?;
		js_object.set(cx, "bottom", bottom)?;
		js_object.set(cx, "right", right)?;

		Ok(js_object)
	}
}

struct Size {
	width: i32,
	height: i32
}

impl Size {
	fn new(width: i32, height: i32) -> Size {
		Size {
			width: width,
			height: height
		}
	}

	fn from_js_object(cx: &mut FunctionContext, js_object: &JsObject) -> Size {
		let width = js_object.get(cx, "width").unwrap().downcast::<JsNumber>().or_throw(cx).unwrap().value() as i32;
		let height = js_object.get(cx, "height").unwrap().downcast::<JsNumber>().or_throw(cx).unwrap().value() as i32;

		Size::new(width, height)
	}

	fn to_js_object<'a>(self, cx: &mut CallContext<'a, JsObject>) -> JsResult<'a, JsObject> {
		let width = cx.number(self.width);
		let height = cx.number(self.height);

		let js_object = JsObject::new(cx);
		js_object.set(cx, "width", width)?;
		js_object.set(cx, "height", height)?;

		Ok(js_object)
	}
}
