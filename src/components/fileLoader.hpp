#pragma once

#include <cmrc/cmrc.hpp>

class FileLoader {
    public:
        FileLoader();

        cmrc::file loadResFile(const char* filename);
};