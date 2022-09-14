#include "sfml.hpp"

namespace SFML {

Framework::Framework() {
    window = new Window;
}

void Framework::shutdown() {
    window->close();
}

}