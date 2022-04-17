#include <Graphics/Renderer.hpp>

using namespace acid;

namespace osuPlusPlus {
    class GameRenderer : public Renderer {
        public:
            GameRenderer();

            void Start() override;
            void Update() override;
    };
}