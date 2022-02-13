#pragma once

extern "C" {
#include "alvr_streamer.h"
}
#include "openvr_driver.h"
#include "tracked_devices.h"

class GenericTracker : public TrackedDevice {
  public:
    virtual vr::EVRInitError Activate(uint32_t object_id) override;
    GenericTracker(uint64_t device_path);
};