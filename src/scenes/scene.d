module scenes.scene;

import std.experimental.logger;
import std.array : split;

/// Base class for Scene rendering
public abstract class Scene {
    /// Render the scene
    abstract void render();

    protected this() {
        auto type = typeid(this).toString.split(".")[$ - 1];

        infof("Loading scene %s...", type);
    }
}