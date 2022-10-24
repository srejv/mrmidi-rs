//Nm = { uniforms: { tDiffuse: { value: null }, side: { value: 1 } }, 
// vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 

// uniform sampler2D tDiffuse;
// uniform int side; 
// varying vec2 vUv;
void mirror(inout vec4 out_color, sampler2D channel, vec2 vUv, int side) { 
    vec2 p = vUv;
    if (side == 0) 
    {
        if (p.x > 0.5) p.x = 1.0 - p.x;
    } 
    else if (side == 1) 
    {
        if (p.x < 0.5) p.x = 1.0 - p.x;
    } 
    else if (side == 2) 
    {
        if (p.y < 0.5) p.y = 1.0 - p.y;
    } 
    else if (side == 3) 
    {
        if (p.y > 0.5) p.y = 1.0 - p.y;
    } 
    out_color = texture2D(channel, p);
}

void mirror_left(inout vec4 out_color, sampler2D channel, vec2 vUv) { 
    vec2 p = vUv;
    if (p.x > 0.5) p.x = 1.0 - p.x;
    out_color = texture2D(channel, p);
}

void mirror_right(inout vec4 out_color, sampler2D channel, vec2 vUv) { 
    vec2 p = vUv;
    if (p.x < 0.5) p.x = 1.0 - p.x;
    out_color = texture2D(channel, p);
}

void mirror_top(inout vec4 out_color, sampler2D channel, vec2 vUv) { 
    vec2 p = vUv;
    if (p.y < 0.5) p.y = 1.0 - p.y;
    out_color = texture2D(channel, p);
}

void mirror_bottom(inout vec4 out_color, sampler2D channel, vec2 vUv) { 
    vec2 p = vUv;
    if (p.y > 0.5) p.y = 1.0 - p.y;
    out_color = texture2D(channel, p);
}
