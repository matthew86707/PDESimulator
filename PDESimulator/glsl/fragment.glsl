#version 140

in vec2 pos;
out vec4 color;

uniform ColorData {
    float time;
    float values[GRID_SIZE_X * GRID_SIZE_Y];
};

void main() {
    int xCell = int(((pos.x + 1.0) / 2.0) * GRID_SIZE_X);
    int yCell = int(((pos.y + 1.0) / 2.0) * GRID_SIZE_Y);
    float val = values[(yCell * 150) + xCell];
    color = vec4(val, val, val, 1.0);
}