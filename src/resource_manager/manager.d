/*
 * Copyright 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software
 * and associated documentation files (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge, publish, distribute,
 * sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all copies or
 * substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
 * BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

module resource_manager.manager;

import gl;
import accessors;
import dlib.math;
import std.experimental.logger;
import std.string : format;
import std.array : split;
import std.parallelism : task, taskPool;
import util;
import resource_manager;

private static Texture[string] textures;
private static AnimatedTexture[string] animated_textures;
private static Shader[string] shaders;

private static SafeQueue!Loadable loadQueue;
private static Loadable[] loadedResources;

private static uint pendingAsync;

/// Ensure we free all resources...
shared static ~this() {
    foreach (resource; loadedResources) {
        resource.unload(true);
        resource.isLoaded = false;
        resource.destroy;
    }
}

/// Check if everything is loaded
bool isGameLoaded() {
    return pendingAsync <= 0 && loadQueue.empty;
}

/// Do a pending load tick
void loadResources() {
    if (!loadQueue.empty) {
        Loadable resource = cast(Loadable) loadQueue.pop;

        if (auto asyncResource = cast(AsyncLoadable) resource) {
            if (!asyncResource.isPreLoaded) {
                auto t = task!preLoadResource(asyncResource);
                taskPool.put(t);

                pendingAsync++;

                // Request another resource load as this task is running async...
                loadResources();
                return;
            }

            pendingAsync--;
        }

        auto type = typeid(resource).toString.split(".")[$ - 1];
        infof("Loading %s '%s:%s'...", type, resource.namespace, resource.name);

        if (resource.isLoaded)
            resource.unload();
        resource.load();
        resource.isLoaded = true;
        loadedResources ~= resource;
    }
}

/// Add a resource to load queue
void loadResource(Loadable resource) {
    loadQueue.push(cast(shared(Loadable)) resource);
}

/// Pre-load a resource, you should call loadResource instead...
void preLoadResource(AsyncLoadable resource) {
    auto type = typeid(resource).toString.split(".")[$ - 1];
    infof("Pre-Loading %s '%s:%s'...", type, resource.namespace, resource.name);

    resource.asyncLoad();
    resource.isPreLoaded = true;

    // Add resource to the queue again, but this time will be full loaded...
    resource.loadResource;
}

/// Represents a resource that can be loaded at initialization
public abstract class Loadable {
    @Read @Write private bool _isLoaded;
    @Read @Write private string _name;
    @Read @Write private string _namespace;

    /// Load the resource
    abstract void load();

    /// Unload the resource
    abstract void unload(bool force = false);

    mixin(GenerateFieldAccessors);
}

/// Represents a resource that can be loaded async at initialization
public abstract class AsyncLoadable : Loadable {
    @Read @Write private bool _isPreLoaded;

    /// Load the resource async
    abstract void asyncLoad();

    mixin(GenerateFieldAccessors);
}

/// Load a resource by name
string loadResource(string name, string type, string namespace = "minecraft") {
    import std.file : readText;

    return readText(resourcePath(name, type, namespace));
}

/// Load a binary resource by name
ubyte[] loadBinaryResource(string name, string type, string namespace = "minecraft") {
    import std.file : read;

    return cast(ubyte[]) read(resourcePath(name, type, namespace));
}

/// Get a resource path, look up on Resource Packs first
string resourcePath(string name, string type, string namespace) {
    // TODO: Look-up on resource packs
    return "resources/%s/%s/%s".format(namespace, type, name);
}
