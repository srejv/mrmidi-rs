// Dm = { uniforms: { tDiffuse: { value: null }, amount: { value: .005 }, angle: { value: 0 } }, 
// vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 
/*
uniform sampler2D tDiffuse;
uniform float amount;
uniform float angle;
varying vec2 vUv;
void main() {
    vec2 offset = amount * vec2( cos(angle), sin(angle));
    vec4 cr = texture2D(tDiffuse, vUv + offset);
    vec4 cga = texture2D(tDiffuse, vUv);
    vec4 cb = texture2D(tDiffuse, vUv - offset);
    gl_FragColor = vec4(cr.r, cga.g, cb.b, cga.a);
}
*/
/*
uniform sampler2D iChannel0;
uniform float amount;
uniform float angle;
varying vec2 uv;
*/
/*
void radialDistort(inout vec4 color, float amount, float angle, sampler2D channel) {
    vec2 offset = amount * vec2( cos(angle), sin(angle));
    vec4 cr = texture2D(channel, uv + offset);
    vec4 cga = texture2D(channel, uv);
    vec4 cb = texture2D(channel, uv - offset);
    color = vec4(cr.r, cga.g, cb.b, cga.a);
}
*/
void rgb_shift(inout vec4 color, float amount, float angle, sampler2D channel) {
    vec2 offset = amount * vec2( cos(angle), sin(angle));
    vec4 cr = texture2D(channel, uv + offset);
    vec4 cga = texture2D(channel, uv);
    vec4 cb = texture2D(channel, uv - offset);
    color = vec4(cr.r, cga.g, cb.b, cga.a);
}