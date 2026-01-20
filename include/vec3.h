#pragma once

/// Simple 3D vector for particle simulation
struct Vec3 {
    float x;
    float y;
    float z;

    Vec3() : x(0.0f), y(0.0f), z(0.0f) {}
    Vec3(float x, float y, float z) : x(x), y(y), z(z) {}

    Vec3 add(const Vec3& other) const {
        return Vec3(x + other.x, y + other.y, z + other.z);
    }

    Vec3 scale(float factor) const {
        return Vec3(x * factor, y * factor, z * factor);
    }
};
