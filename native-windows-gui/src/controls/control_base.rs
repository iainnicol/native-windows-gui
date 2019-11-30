use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::{HWND, HMENU};
use super::ControlHandle;
use crate::win32::window::{build_hwnd_control, build_timer, build_notice, build_tray_notif};
use crate::win32::menu::build_hmenu_control;
use crate::SystemError;

const NOTICE: u32 = 1;
const TRAY: u32 = 2;


#[derive(Debug, Clone)]
pub struct ControlBase {
    pub handle: ControlHandle
}

impl ControlBase {

    pub fn build_hwnd() -> HwndBuilder {
        HwndBuilder::default()
    }

    pub fn build_hmenu() -> HmenuBuilder {
        HmenuBuilder::default()
    }

    pub fn build_timer() -> TimerBuilder {
        TimerBuilder::default()
    }

    pub fn build_notice() -> OtherBuilder {
        OtherBuilder {
            parent: None,
            ty: NOTICE
        }
    }

    pub fn build_tray_notification() -> OtherBuilder {
        OtherBuilder {
            parent: None,
            ty: TRAY
        }
    }
}

#[derive(Default)]
pub struct HwndBuilder {
    class_name: String,
    text: Option<String>,
    size: Option<(i32, i32)>,
    pos: Option<(i32, i32)>,
    forced_flags: DWORD,
    flags: Option<DWORD>,
    ex_flags: Option<DWORD>,
    parent: Option<HWND>
}

impl HwndBuilder {

    pub fn class_name<'a>(mut self, name: &'a str) -> HwndBuilder {
        self.class_name = name.to_string();
        self
    }

    pub fn text<'a>(mut self, text: &'a str) -> HwndBuilder {
        self.text = Some(text.to_string());
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> HwndBuilder {
        self.size = Some(size);
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> HwndBuilder {
        self.pos = Some(pos);
        self
    }

    pub fn flags(mut self, flags: u32) -> HwndBuilder {
        self.flags = Some(flags as DWORD);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> HwndBuilder {
        self.ex_flags = Some(flags as DWORD);
        self
    }

    pub fn forced_flags(mut self, flags: u32) -> HwndBuilder {
        self.forced_flags = flags as DWORD;
        self
    }

    pub fn parent(mut self, parent: Option<ControlHandle>) -> HwndBuilder {
        match parent {
            Some(p) => { self.parent = p.hwnd(); }
            None => { self.parent = None; }
        }
        self
    }

    pub fn build(self) -> Result<ControlHandle, SystemError> {
        let handle = unsafe { build_hwnd_control(
            &self.class_name,
            self.text.as_ref().map(|v| v as &str),
            self.size,
            self.pos,
            self.flags,
            self.ex_flags,
            self.forced_flags,
            self.parent
        )? };

        Ok(handle)
    }
}


#[derive(Default)]
pub struct HmenuBuilder {
    text: Option<String>,
    item: bool,
    separator: bool,
    popup: bool,
    parent_menu: Option<HMENU>,
    parent_window: Option<HWND>,
}

impl HmenuBuilder {

    /// Set the text of the Menu
    pub fn text<'a>(mut self, text: &'a str) -> HmenuBuilder {
        self.text = Some(text.to_string());
        self
    }

    /// Set if the menu should be an item or a menu
    pub fn item(mut self, i: bool) -> HmenuBuilder {
        self.item = i;
        self
    }

    /// Set if the menu item should be a separator
    pub fn separator(mut self, i: bool) -> HmenuBuilder {
        self.separator = i;
        self
    }

    /// Set if the menu item should be a separator
    pub fn popup(mut self, i: bool) -> HmenuBuilder {
        self.popup = i;
        self
    }

    /// Set the parent of the menu. Can be a window or another menu.
    pub fn parent(mut self, parent: ControlHandle) -> HmenuBuilder {
        match parent {
            ControlHandle::Hwnd(hwnd) => { self.parent_window = Some(hwnd); }
            ControlHandle::Menu(_parent, menu) => { self.parent_menu = Some(menu); }
            ControlHandle::PopMenu(_hwnd, menu) => { self.parent_menu = Some(menu); },
            _ => {}
        }

        self
    }

    pub fn build(self) -> Result<ControlHandle, SystemError> {
        let handle = unsafe { build_hmenu_control(
            self.text,
            self.item,
            self.separator,
            self.popup,
            self.parent_menu,
            self.parent_window
        )? };

        Ok(handle)
    }

}


#[derive(Default)]
pub struct TimerBuilder {
    parent: Option<HWND>,
    interval: u32,
    stopped: bool
}

impl TimerBuilder {

    pub fn stopped(mut self, v: bool) -> TimerBuilder {
        self.stopped = v;
        self
    }

    pub fn interval(mut self, i: u32) -> TimerBuilder {
        self.interval = i;
        self
    }

    pub fn parent(mut self, parent: &ControlBase) -> TimerBuilder {
        self.parent = parent.handle.hwnd();
        self
    }

    pub fn build(self) -> Result<ControlBase, SystemError> {
        let handle = unsafe { build_timer(
            self.parent.expect("Internal error. Timer without window parent"),
            self.interval,
            self.stopped
        ) };
        Ok(ControlBase { handle })
    }

}


#[derive(Default)]
pub struct OtherBuilder {
    parent: Option<HWND>,
    ty: u32
}

impl OtherBuilder {

    pub fn parent(mut self, parent: HWND) -> OtherBuilder {
        self.parent = Some(parent);
        self
    }

    pub fn build(self) -> Result<ControlHandle, SystemError> {
        let handle = self.parent.expect("Internal error. Notice without window parent");
        let base = match self.ty {
            NOTICE => build_notice(handle),
            TRAY => build_tray_notif(handle),
            _ => unreachable!()
        };

        Ok(base)
    }

}
