#include <Engine/Engine.hpp>
#include <Graphics/Graphics.hpp>
#include "base.hpp"
#include "main.hpp"
#include "renderer.hpp"

using namespace osuPlusPlus;
using namespace acid;

GAME_STATES gameState;

int main(int argc, char* args[]) {
    auto engine = std::make_unique<Engine>(args[0]);
    engine->SetApp(std::make_unique<Game>());

    int exit_code = engine->Run();
    return exit_code;
}

namespace osuPlusPlus {
    Game::Game() : App("osu++",{OSUPP_VERSION_MAJOR, OSUPP_VERSION_MINOR, OSUPP_VERSION_REVISION}) {

    }
    Game::~Game() {

    }

    void Game::Start() {
        Graphics::Get()->SetRenderer(std::make_unique<GameRenderer>());
    }
    void Game::Update() {

    }
}