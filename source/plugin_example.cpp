#include <hex/plugin.hpp>

#include <hex/views/view.hpp>

class ViewExample : public hex::View {
public:
    ViewExample() : hex::View("Example") {}
    ~ViewExample() override {}

    void drawContent() override {
        if (ImGui::Begin("Example")) {
            ImGui::Text("Custom plugin window");
        }
        ImGui::End();
    }
};

IMHEX_PLUGIN_SETUP("C++ Template Plugin", "Plugin Author", "Plugin Description") {

    ContentRegistry::Views::add<ViewExample>();

}


