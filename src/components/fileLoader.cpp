#include "fileLoader.hpp"
#include <cmrc/cmrc.hpp>

CMRC_DECLARE(resources);

FileLoader::FileLoader() {

}

cmrc::file FileLoader::loadResFile(const char* filename) {
    return cmrc::resources::get_filesystem().open(filename);
}