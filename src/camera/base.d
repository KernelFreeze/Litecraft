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

module camera.base;

import dlib.math;
import accessors;
import gl : time;
import std.math;
import litecraft;

private {
    const float speed = 2.5f;
    const float sensitity = 0.1f;
    const float zoom = 45.0f;
}

/// Camera movement direction
enum CameraMovement {
    forward,
    backward,
    left,
    right
}

/**
    An abstract camera class that processes input and calculates the corresponding Eular Angles,
    Vectors and Matrices for use in OpenGL.
*/
class Camera {
    protected {
        // Camera Attributes
        @Read vec3 _position;
        vec3 _front;
        vec3 _up;
        vec3 _right;
        vec3 _worldUp;

        // Eular Angles
        @Read float _yaw = -90.0f;
        @Read float _pitch = 0.0f;

        // Camera options
        @Read float _movementSpeed;
        @Read float _mouseSensitivity;
        @Read float _zoom;
    }

    /// Create a camera using vectors
    this(vec3 position = vec3(0.0f, 0.0f, 0.0f), vec3 up = vec3(0.0f, 1.0f, 0.0f)) {
        this._position = position;
        this._worldUp = up;

        updateCameraVectors();
    }

    /// Create a camera using float values
    this(float posX, float posY, float posZ, float upX = 0.0f, float upY = 1.0f, float upZ = 0.0f) {
        this._position = vec3(posX, posY, posZ);
        this._worldUp = vec3(upX, upY, upZ);

        updateCameraVectors();
    }

    /// Get view matrix
    mat4 viewMatrix() {
        return lookAtMatrix(_position, _position + _front, _up);
    }

    /// Get camera projection
    mat4 projection() {
        return perspectiveMatrix(zoom, Litecraft.width / Litecraft.height, 0.1f, 100.0f);
    }

    /// Processes input received from any keyboard-like input system
    void processKeyboardInput(CameraMovement direction) {
        immutable auto velocity = _movementSpeed * time;

        switch (direction) {
        case CameraMovement.forward:
            position += _front * velocity;
            break;
        case CameraMovement.backward:
            position -= _front * velocity;
            break;
        case CameraMovement.left:
            position -= _right * velocity;
            break;
        case CameraMovement.right:
            position += _right * velocity;
            break;
        default:
            break;
        }
    }

    /// Processes input received from a mouse input system. Expects the offset value in both the x and y direction.
    void processMouseMovement(float xoffset, float yoffset, bool constrainPitch = true) {
        xoffset *= _mouseSensitivity;
        yoffset *= _mouseSensitivity;

        _yaw += xoffset;
        _pitch += yoffset;

        // Make sure that when pitch is out of bounds, screen doesn't get flipped
        if (constrainPitch) {
            if (_pitch > 89.0f)
                _pitch = 89.0f;
            if (_pitch < -89.0f)
                _pitch = -89.0f;
        }

        // Update Front, Right and Up Vectors using the updated Eular angles
        updateCameraVectors();
    }

    // Calculates the front vector from the Camera's (updated) Eular Angles
    private void updateCameraVectors() {
        // Calculate the new Front vector
        {
            vec3 front;
            front.x = cos(degtorad(_yaw)) * cos(degtorad(_pitch));
            front.y = sin(degtorad(_pitch));
            front.z = sin(degtorad(_yaw)) * cos(degtorad(_pitch));

            front.normalize();
            _front = front;
        }

        // Also re-calculate the Right and Up vector
        _right = cross(_front, _worldUp);
        _right.normalize();

        _up = cross(_right, _front);
        _up.normalize();
    }

    mixin(GenerateFieldAccessors);
}
