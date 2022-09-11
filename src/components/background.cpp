#include "background.hpp"
#include <iostream>
using namespace SFML;

Background::Background(Framework *framework) : Sprite::Sprite(framework, "res/background.jpg") {}