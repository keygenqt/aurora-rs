#include "aurora-rs/src/cxx/cpp/streamcamera/cxx_streamcamera.h"
#include "aurora-rs/src/cxx/rust/cxx_streamcamera.rs.h"

namespace com
{
  namespace keygenqt
  {
    namespace aurora_rs
    {
      std::unique_ptr<CxxStreamCamera> new_streamcamera()
      {
        return std::make_unique<CxxStreamCamera>();
      }

      CxxStreamCamera::CxxStreamCamera() : m_manager(StreamCameraManager()) {}

      uint64_t CxxStreamCamera::count_cameras() const
      {
        return m_manager->getNumberOfCameras();
      }
    } // namespace aurora_rs
  } // namespace keygenqt
} // namespace com
