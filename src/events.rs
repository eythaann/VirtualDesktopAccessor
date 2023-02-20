use crate::Desktop;
use crate::DesktopEventThread;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::{cell::RefCell, rc::Rc};
use windows::Win32::Foundation::HWND;

#[derive(Debug, Clone)]
pub enum DesktopEventSender<T> {
    Std(std::sync::mpsc::Sender<T>),

    // #[cfg(feature = "crossbeam-channel")]
    Crossbeam(crossbeam_channel::Sender<T>),
}

impl<T> DesktopEventSender<T> {
    pub fn try_send(&self, event: T) {
        match self {
            DesktopEventSender::Std(sender) => {
                let _ = sender.send(event);
            }

            // #[cfg(feature = "crossbeam-channel")]
            DesktopEventSender::Crossbeam(sender) => {
                let _ = sender.try_send(event);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum DesktopEvent {
    DesktopCreated(Desktop),
    DesktopDestroyed(Desktop),
    DesktopChanged {
        new: Desktop,
        old: Desktop,
    },
    DesktopNameChanged(Desktop, String),
    DesktopWallpaperChanged(Desktop, String),
    DesktopMoved {
        desktop: Desktop,
        old_index: i64,
        new_index: i64,
    },
    WindowChanged(HWND),
}

pub fn create_event_thread<T>(sender: DesktopEventSender<T>) -> DesktopEventThread
where
    T: From<DesktopEvent> + Clone + Send + 'static,
{
    DesktopEventThread::new(sender)
}