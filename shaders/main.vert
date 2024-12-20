#version 150

in vec2 position;
in vec4 color;
out vec4 fscolor;
uniform mat4 trans;

void main() {
  fscolor = color;
  gl_Position = trans * vec4(position, 0.0, 1.0);
}
