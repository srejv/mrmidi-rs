// Im = { uniforms: { tDiffuse: { value: null }, tSize: { value: [256, 256] }, center: { value: [.5, .5] }, angle: { value: 1.57 }, scale: { value: 1 } }, 
// vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 

// uniform vec2 center;
// uniform float angle;
// uniform float scale;
// uniform vec2 tSize;
// uniform sampler2D tDiffuse;
// varying vec2 vUv; 

float pattern(vec2 vUv, float tSize, float angle, float scale) {
    float s = sin( angle ), c = cos( angle );
    vec2 tex = vUv * tSize - center;
    vec2 point = vec2( c * tex.x - s * tex.y, s * tex.x + c * tex.y ) * scale;
    return ( sin( point.x ) * sin( point.y ) ) * 4.0;
}

void pattern_sampler(inout vec4 out_color, sampler2D channel, vec2 vUv, vec2 center, float angle, float scale, vec2 tSize) {
    vec4 color = texture2D(channel, vUv);
    float average = ( color.r + color.g + color.b ) / 3.0;
    out_color = vec4( vec3( average * 10.0 - 5.0 + pattern(vUv, tSize, angle, scale) ), color.a );
}
