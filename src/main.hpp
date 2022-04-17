#include <Engine/App.hpp>

using namespace acid;

namespace osuPlusPlus {
    class Game : public App {
        public:
	        Game();
	        ~Game();

	        void Start() override;
	        void Update() override;
    };
}