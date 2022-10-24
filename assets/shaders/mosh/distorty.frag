// Lm = { uniforms: { tDiffuse: { type: "t", value: null }, amount: { type: "f", value: .5 }, time: { type: "f", value: .5 } }, 
// vertexShader: "" + "\t\t${cm}" + "\t", 

const float TWO_PI = 6.283185307179586;

// uniform sampler2D tDiffuse;
//uniform float amount;
u//niform float time;

// varying vec2 vUv;

vec2 rotate2D(vec2 position, float theta) {
    mat2 m = mat2( cos(theta), -sin(theta), sin(theta), cos(theta) );
    return m * position;
}

void rotation_thing(inout vec4 out_color, sampler2D channel, vec2 uv, float amount) {
    vec2 p = uv;
    //Displace image by its own rg channel
    vec2 sPos = uv;
    vec2 off = texture2D( channel, sPos ).rg - 0.5;

    //rotate
    float ang = iTime * TWO_PI;
    off = rotate2D(off, ang);
    p += off * amount;
    vec4 col = texture2D(channel, p);
    out_color = col;
}
