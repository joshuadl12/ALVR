<<<<<<< HEAD
use crate::{graphics_info, MAYBE_WINDOW};
use alvr_common::prelude::*;
use std::{fs, sync::Arc};
=======
use crate::{graphics, ClientListAction, MAYBE_WINDOW, SESSION_MANAGER};
use alvr_common::{lazy_static, prelude::*};
// use alvr_gui::Dashboard;
use crate::capi::{AlvrEvent, DRIVER_EVENT_SENDER};
use alvr_session::SessionDesc;
use parking_lot::Mutex;
use std::{
    fs,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::broadcast::{error::RecvError, Sender};
>>>>>>> libalvr

// this thread gets interrupted when SteamVR closes
// todo: handle this in a better way
pub fn ui_thread() -> StrResult {
    const WINDOW_WIDTH: u32 = 800;
    const WINDOW_HEIGHT: u32 = 600;

    let (pos_left, pos_top) = if let Ok((screen_width, screen_height)) = graphics::get_screen_size()
    {
        (
            (screen_width - WINDOW_WIDTH) / 2,
            (screen_height - WINDOW_HEIGHT) / 2,
        )
    } else {
        (0, 0)
    };

    let temp_dir = trace_err!(tempfile::TempDir::new())?;
    let user_data_dir = temp_dir.path();
    trace_err!(fs::File::create(
        temp_dir.path().join("FirstLaunchAfterInstallation")
    ))?;

    let window = Arc::new(trace_err!(alcro::UIBuilder::new()
        .content(alcro::Content::Url("http://127.0.0.1:8082"))
        .user_data_dir(user_data_dir)
        .size(WINDOW_WIDTH as _, WINDOW_HEIGHT as _)
        .custom_args(&[
            "--disk-cache-size=1",
            &format!("--window-position={pos_left},{pos_top}")
        ])
        .run())?);

    *MAYBE_WINDOW.lock() = Some(Arc::clone(&window));

    window.wait_finish();

    // prevent panic on window.close()
    *MAYBE_WINDOW.lock() = None;
    crate::shutdown_runtime();

    unsafe { crate::ShutdownSteamvr() };

    if let Some(sender) = &*DRIVER_EVENT_SENDER.lock() {
        sender.send(AlvrEvent::Shutdown).ok();
    }

    Ok(())
}
