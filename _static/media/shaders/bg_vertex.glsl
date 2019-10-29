precision mediump float;

attribute vec2 a_vertex;
varying vec2 v_uv;

void main() {

    //scale it to go from 0->1 to 0->2, then subtract to make it -1->1
    vec2 pos = (a_vertex * 2.0) - 1.0;
    gl_Position = vec4(pos,1,1);
    v_uv = a_vertex;
}