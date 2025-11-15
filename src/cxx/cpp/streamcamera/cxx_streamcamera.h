#pragma once
#include "rust/cxx.h"
#include <memory>

#include <streamcamera.h>

namespace com
{
  namespace keygenqt
  {
    namespace aurora_rs
    {
      class CxxStreamCamera
      {

      public:
        CxxStreamCamera();
        uint64_t count_cameras() const;

      private:
        Aurora::StreamCamera::CameraManager *m_manager;
      };

      // Create class
      std::unique_ptr<CxxStreamCamera> new_streamcamera();
    } // namespace aurora_rs
  } // namespace keygenqt
} // namespace com
