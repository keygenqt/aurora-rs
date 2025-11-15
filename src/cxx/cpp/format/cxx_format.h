#pragma once
#include "rust/cxx.h"
#include <memory>

namespace com
{
  namespace keygenqt
  {
    namespace aurora_rs
    {
      class CxxFormat
      {

      public:
        CxxFormat();
        rust::String time(rust::String);
      };

      // Create class
      std::unique_ptr<CxxFormat> new_format();
    } // namespace aurora_rs
  } // namespace keygenqt
} // namespace com
