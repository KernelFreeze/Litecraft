module scenes.scene;

private {
    import std.experimental.logger;
    import std.array : split;
    import draw;
    import camera.base;
}

/// Base class for Scene rendering
public abstract class Scene {
    /// Render the scene
    abstract void render3D();

    /// Render 2D objects
    abstract void render2D();

    protected Camera _camera;

    ///Get scene camera
    Camera camera() {
        return _camera;
    }

    ///Get scene camera
    T camera(T)() {
        return cast(T) _camera;
    }

    protected this() {
        auto type = typeid(this).toString.split(".")[$ - 1];

        infof("Loading scene '%s'...", type);
    }
}