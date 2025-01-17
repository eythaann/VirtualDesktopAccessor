//! New for this version is that the [`IVirtualDesktopManagerInternal`]
//! interfaces take "monitor" arguments, note that the The
//! [`IVirtualDesktopNotification`] interface still doesn't use monitors.
use super::*;
use build_10240 as prev_build;

// These interfaces haven't changed since previous version:
prev_build::IApplicationView!("372E1D3B-38D3-42E4-A15B-8AB2B178F513");
prev_build::IApplicationViewCollection!("1841C6D7-4F9D-42C0-AF41-8747538F10E5");
prev_build::IVirtualDesktopNotificationService!("0cd45e71-d927-4f15-8b0a-8fef525337bf");
prev_build::IVirtualDesktopPinnedApps!("4CE81583-1E4C-4632-A621-07A53543148F");

// But these interfaces have different methods:

reusable_com_interface!(
    MacroOptions {
        temp_macro_name: _IVirtualDesktop,
        iid: "62FDF88B-11CA-4AFB-8BD8-2296DFAE49E2",
    },
    {
        pub unsafe trait IVirtualDesktop: IUnknown {
            pub unsafe fn is_view_visible(
                &self,
                p_view: ComIn<IApplicationView>,
                out_bool: *mut u32,
            ) -> HRESULT;
            pub unsafe fn get_id(&self, out_guid: *mut GUID) -> HRESULT;

            // These methods are new:

            pub unsafe fn get_monitor(
                &self,
                monitor: HMONITOR,
                out_monitor: *mut HMONITOR,
            ) -> HRESULT;
            pub unsafe fn get_name(&self, out_string: *mut HSTRING) -> HRESULT;
        }
    }
);

reusable_com_interface!(
    MacroOptions {
        temp_macro_name: _IVirtualDesktopManagerInternal,
        iid: "094AFE11-44F2-4BA0-976F-29A97E263EE0",
    },
    {
        pub unsafe trait IVirtualDesktopManagerInternal: IUnknown {
            pub unsafe fn get_desktop_count_m(
                &self,
                monitor: HMONITOR,
                out_count: *mut UINT,
            ) -> HRESULT;

            pub unsafe fn move_view_to_desktop(
                &self,
                view: ComIn<IApplicationView>,
                desktop: ComIn<IVirtualDesktop>,
            ) -> HRESULT;

            pub unsafe fn can_move_view_between_desktops(
                &self,
                view: ComIn<IApplicationView>,
                can_move: *mut i32,
            ) -> HRESULT;

            pub unsafe fn get_current_desktop_m(
                &self,
                monitor: HMONITOR,
                out_desktop: *mut Option<IVirtualDesktop>,
            ) -> HRESULT;

            pub unsafe fn get_desktops_m(
                &self,
                monitor: HMONITOR,
                out_desktops: *mut Option<IObjectArray>,
            ) -> HRESULT;

            /// Get next or previous desktop
            ///
            /// Direction values:
            /// 3 = Left direction
            /// 4 = Right direction
            pub unsafe fn get_adjacent_desktop(
                &self,
                in_desktop: ComIn<IVirtualDesktop>,
                direction: UINT,
                out_pp_desktop: *mut Option<IVirtualDesktop>,
            ) -> HRESULT;

            pub unsafe fn switch_desktop_m(
                &self,
                monitor: HMONITOR,
                desktop: ComIn<IVirtualDesktop>,
            ) -> HRESULT;

            pub unsafe fn create_desktop_m(
                &self,
                monitor: HMONITOR,
                out_desktop: *mut Option<IVirtualDesktop>,
            ) -> HRESULT;

            pub unsafe fn remove_desktop(
                &self,
                destroy_desktop: ComIn<IVirtualDesktop>,
                fallback_desktop: ComIn<IVirtualDesktop>,
            ) -> HRESULT;

            pub unsafe fn find_desktop(
                &self,
                guid: *const GUID,
                out_desktop: *mut Option<IVirtualDesktop>,
            ) -> HRESULT;

            // These methods are new:

            pub unsafe fn get_desktop_switch_include_exclude_views(
                &self,
                desktop: ComIn<IVirtualDesktop>,
                out_pp_desktops1: *mut IObjectArray,
                out_pp_desktops2: *mut IObjectArray,
            ) -> HRESULT;

            pub unsafe fn set_name(
                &self,
                desktop: ComIn<IVirtualDesktop>,
                name: HSTRING,
            ) -> HRESULT;

            pub unsafe fn copy_desktop_state(
                &self,
                view0: ComIn<IApplicationView>,
                view1: ComIn<IApplicationView>,
            ) -> HRESULT;

            pub unsafe fn get_desktop_is_per_monitor(&self, out_per_monitor: *mut i32) -> HRESULT;
        }
        impl IVirtualDesktopManagerInternal {
            pub unsafe fn get_desktop_count(&self, out_count: *mut UINT) -> HRESULT {
                self.get_desktop_count_m(0, out_count)
            }
            pub unsafe fn get_current_desktop(
                &self,
                out_desktop: *mut Option<IVirtualDesktop>,
            ) -> HRESULT {
                self.get_current_desktop_m(0, out_desktop)
            }

            pub unsafe fn get_desktops(
                &self,
                out_desktops: *mut Option<IObjectArray>,
            ) -> HRESULT {
                self.get_desktops_m(0, out_desktops)
            }

            pub unsafe fn switch_desktop(
                &self,
                desktop: ComIn<IVirtualDesktop>,
            ) -> HRESULT {
                self.switch_desktop_m(0, desktop)
            }

            pub unsafe fn create_desktop(
                &self,
                out_desktop: *mut Option<IVirtualDesktop>,
            ) -> HRESULT {
                self.create_desktop_m(0, out_desktop)
            }
        }
    }
);

reusable_com_interface!(
    MacroOptions {
        temp_macro_name: _IVirtualDesktopNotification,
        iid: "f3163e11-6b04-433c-a64b-6f82c9094257",
    },
    {
        pub unsafe trait IVirtualDesktopNotification: IUnknown {
            pub unsafe fn virtual_desktop_created(
                &self,
                desktop: ComIn<IVirtualDesktop>,
            ) -> HRESULT;

            pub unsafe fn virtual_desktop_destroy_begin(
                &self,
                desktop_destroyed: ComIn<IVirtualDesktop>,
                desktop_fallback: ComIn<IVirtualDesktop>,
            ) -> HRESULT;

            pub unsafe fn virtual_desktop_destroy_failed(
                &self,
                desktop_destroyed: ComIn<IVirtualDesktop>,
                desktop_fallback: ComIn<IVirtualDesktop>,
            ) -> HRESULT;

            pub unsafe fn virtual_desktop_destroyed(
                &self,
                desktop_destroyed: ComIn<IVirtualDesktop>,
                desktop_fallback: ComIn<IVirtualDesktop>,
            ) -> HRESULT;

            // This method is new:
            pub unsafe fn virtual_desktop_is_per_monitor_changed(&self, i: i64) -> HRESULT;

            pub unsafe fn virtual_desktop_name_changed(
                &self,
                desktop: ComIn<IVirtualDesktop>,
                name: HSTRING,
            ) -> HRESULT;

            pub unsafe fn view_virtual_desktop_changed(
                &self,
                view: ComIn<IApplicationView>,
            ) -> HRESULT;

            pub unsafe fn current_virtual_desktop_changed(
                &self,
                desktop_old: ComIn<IVirtualDesktop>,
                desktop_new: ComIn<IVirtualDesktop>,
            ) -> HRESULT;
        }

        /// Implements an unstable interface that is only valid for a single Windows
        /// version using a more stable trait that works for all Windows versions.
        #[windows::core::implement(IVirtualDesktopNotification)]
        pub struct VirtualDesktopNotificationAdaptor<T>
        where
            T: build_dyn::IVirtualDesktopNotification_Impl,
        {
            pub inner: T,
        }
        impl<T> IVirtualDesktopNotification_Impl for VirtualDesktopNotificationAdaptor<T>
        where
            T: build_dyn::IVirtualDesktopNotification_Impl,
        {
            unsafe fn virtual_desktop_created(&self, desktop: ComIn<IVirtualDesktop>) -> HRESULT {
                self.inner.virtual_desktop_created(desktop.into())
            }

            unsafe fn virtual_desktop_destroy_begin(
                &self,
                desktop_destroyed: ComIn<IVirtualDesktop>,
                desktop_fallback: ComIn<IVirtualDesktop>,
            ) -> HRESULT {
                self.inner.virtual_desktop_destroy_begin(
                    desktop_destroyed.into(),
                    desktop_fallback.into(),
                )
            }

            unsafe fn virtual_desktop_destroy_failed(
                &self,
                desktop_destroyed: ComIn<IVirtualDesktop>,
                desktop_fallback: ComIn<IVirtualDesktop>,
            ) -> HRESULT {
                self.inner.virtual_desktop_destroy_failed(
                    desktop_destroyed.into(),
                    desktop_fallback.into(),
                )
            }

            unsafe fn virtual_desktop_destroyed(
                &self,
                desktop_destroyed: ComIn<IVirtualDesktop>,
                desktop_fallback: ComIn<IVirtualDesktop>,
            ) -> HRESULT {
                self.inner
                    .virtual_desktop_destroyed(desktop_destroyed.into(), desktop_fallback.into())
            }

            unsafe fn virtual_desktop_is_per_monitor_changed(&self, _i: i64) -> HRESULT {
                HRESULT(0)
            }

            unsafe fn virtual_desktop_name_changed(
                &self,
                desktop: ComIn<IVirtualDesktop>,
                name: HSTRING,
            ) -> HRESULT {
                self.inner
                    .virtual_desktop_name_changed(desktop.into(), name)
            }

            unsafe fn view_virtual_desktop_changed(
                &self,
                view: ComIn<IApplicationView>,
            ) -> HRESULT {
                self.inner.view_virtual_desktop_changed(view.into())
            }

            unsafe fn current_virtual_desktop_changed(
                &self,
                desktop_old: ComIn<IVirtualDesktop>,
                desktop_new: ComIn<IVirtualDesktop>,
            ) -> HRESULT {
                self.inner
                    .current_virtual_desktop_changed(desktop_old.into(), desktop_new.into())
            }
        }
    }
);
