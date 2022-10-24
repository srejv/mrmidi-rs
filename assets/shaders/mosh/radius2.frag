// km = { uniforms: { tDiffuse: { value: null }, v: { value: 1 / 512 }, r: { value: .35 } }, 
// vertexShader: [
// "varying vec2 vUv;", 
// "void main() {", 
//     "vUv = uv;", 
//     "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", 
// "}"].join("" + ""), 

// uniform sampler2D tDiffuse;
// uniform float v;
// uniform float r; 
// varying vec2 vUv;

void radius_2(inout vec4 out_color, sampler2D channel, vec2 vUv, float r, float v) {
    vec4 sum = vec4( 0.0 );
    float vv = v * abs( r - vUv.y );
    sum += texture2D( channel, vec2( vUv.x, vUv.y - 4.0 * vv ) ) * 0.051;
    sum += texture2D( channel, vec2( vUv.x, vUv.y - 3.0 * vv ) ) * 0.0918;
    sum += texture2D( channel, vec2( vUv.x, vUv.y - 2.0 * vv ) ) * 0.12245;
    sum += texture2D( channel, vec2( vUv.x, vUv.y - 1.0 * vv ) ) * 0.1531; 
    sum += texture2D( channel, vec2( vUv.x, vUv.y ) ) * 0.1633;
    sum += texture2D( channel, vec2( vUv.x, vUv.y + 1.0 * vv ) ) * 0.1531;
    sum += texture2D( channel, vec2( vUv.x, vUv.y + 2.0 * vv ) ) * 0.12245;
    sum += texture2D( channel, vec2( vUv.x, vUv.y + 3.0 * vv ) ) * 0.0918; 
    sum += texture2D( channel, vec2( vUv.x, vUv.y + 4.0 * vv ) ) * 0.051; 
    out_color = sum;, 
}
