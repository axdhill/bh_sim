#version 100
precision lowp float;

varying vec2 uv;         // Texture coordinates
uniform sampler2D Texture; // The texture/screen you're rendering
uniform float time;      // Time can be used to animate the effect

void main() {
    vec4 color = texture2D(Texture, uv); // Get the current pixel color
    float alpha = color.a;               // Extract the alpha channel

    // Shift the color based on the alpha channel
    vec3 shifted_color = vec3(
        color.r * (1.0 - alpha),
        color.g * alpha,
        color.b * (1.0 - alpha)
    );

    // Optionally, you could add some time-based effect to make it dynamic
    shifted_color.r += sin(time * alpha) * 0.2;

    gl_FragColor = vec4(shifted_color, color.a); // Output the new color
}
