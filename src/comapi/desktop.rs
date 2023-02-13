use super::*;
use std::fmt::Debug;
use windows::core::GUID;

use super::raw::*;

type HWND_ = u32;

#[derive(Copy, Clone, PartialEq)]
pub struct Desktop(GUID);

impl Debug for Desktop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Desktop({:?})", self.0)
    }
}

impl Desktop {
    pub(crate) fn empty() -> Desktop {
        Desktop(GUID::default())
    }

    pub fn get_id(&self) -> GUID {
        self.0
    }

    pub fn get_name(&self) -> Result<String> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let desktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        get_idesktop_name(&desktop)
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        set_idesktop_name(&manager, &idesktop, name)
    }

    pub fn get_index(&self) -> Result<u32> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        let index = get_idesktop_number(&manager, &idesktop)?;
        Ok(index)
    }

    pub fn get_wallpaper(&self) -> Result<String> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        get_idesktop_wallpaper(&idesktop)
    }

    pub fn set_wallpaper(&self, path: &str) -> Result<()> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        set_idesktop_wallpaper(&manager, &idesktop, path)
    }

    pub fn switch_to(&self) -> Result<()> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        switch_to_idesktop(&manager, &idesktop)
    }

    pub fn remove(&self, fallback_desktop: &Desktop) -> Result<()> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        let fallback_idesktop = get_idesktop_by_guid(&manager, &fallback_desktop.0)?;
        remove_idesktop(&manager, &idesktop, &fallback_idesktop)
    }

    pub fn has_window(&self, hwnd: HWND_) -> Result<bool> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager_internal = get_ivirtual_desktop_manager_internal(&provider)?;
        let manager = get_ivirtual_desktop_manager(&provider)?;
        let desktop = get_idesktop_by_window(&manager_internal, &manager, hwnd)?;
        let desktop_id = get_idesktop_guid(&desktop)?;
        Ok(desktop_id == self.get_id())
    }

    pub fn move_window(&self, hwnd: HWND_) -> Result<()> {
        com_sta();
        let provider = get_iservice_provider()?;
        let manager = get_ivirtual_desktop_manager_internal(&provider)?;
        let vc = get_iapplication_view_collection(&provider)?;
        let view = get_iapplication_view_for_hwnd(&vc, hwnd)?;
        let idesktop = get_idesktop_by_guid(&manager, &self.get_id())?;
        move_view_to_desktop(&manager, &view, &idesktop)
    }
}

pub fn create_desktop() -> Result<Desktop> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let desktop = create_idesktop(&manager)?;
    let id = get_idesktop_guid(&desktop)?;
    Ok(Desktop(id))
}

pub fn get_current_desktop() -> Result<Desktop> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let desktop = get_current_idesktop(&manager)?;
    let id = get_idesktop_guid(&desktop)?;
    Ok(Desktop(id))
}

pub fn get_desktop_count() -> Result<u32> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let desktops = get_idesktops_array(&manager)?;
    unsafe { desktops.GetCount().map_err(map_win_err) }
}
pub fn get_desktops() -> Result<Vec<Desktop>> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let desktops: Result<Vec<Desktop>> = get_idesktops(&manager)?
        .into_iter()
        .map(|d| -> Result<Desktop> {
            let mut desktop = Desktop::empty();
            unsafe { d.get_id(&mut desktop.0).as_result()? };
            Ok(desktop)
        })
        .collect();
    Ok(desktops?)
}

pub fn get_desktop_by_index(index: u32) -> Result<Desktop> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let desktop = get_idesktop_by_number(&manager, index)?;
    let id = get_idesktop_guid(&desktop)?;
    Ok(Desktop(id))
}

pub fn get_desktop_by_guid(guid: &GUID) -> Result<Desktop> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager = get_ivirtual_desktop_manager_internal(&provider)?;
    let desktop = get_idesktop_by_guid(&manager, &guid)?;
    let id = get_idesktop_guid(&desktop)?;
    Ok(Desktop(id))
}

pub fn get_desktop_by_window(hwnd: HWND_) -> Result<Desktop> {
    com_sta();
    let provider = get_iservice_provider()?;
    let manager_internal = get_ivirtual_desktop_manager_internal(&provider)?;
    let manager = get_ivirtual_desktop_manager(&provider)?;
    let desktop = get_idesktop_by_window(&manager_internal, &manager, hwnd)?;
    let id = get_idesktop_guid(&desktop)?;
    Ok(Desktop(id))
}
