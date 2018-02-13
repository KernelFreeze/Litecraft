module scenes.scene;

import std.experimental.logger;
import std.array : split;

public import draw;

/// Base class for Scene rendering
public abstract class Scene {
    /// Render the scene
    abstract void render3D();

    /// Render 2D objects
    abstract void render2D();

    protected this() {
        auto type = typeid(this).toString.split(".")[$ - 1];

        infof("Loading scene '%s'...", type);
    }
}