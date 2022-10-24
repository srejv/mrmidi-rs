// Om = { uniforms: { tDiffuse: { value: null }, offset: { value: 1 }, darkness: { value: 1 } }, 
// vertexShader: ["varying vec2 vUv;", "void main() {", "vUv = uv;", "gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );", "}"].join("" + ""), 

// uniform float offset;
//uniform float darkness;
// uniform sampler2D tDiffuse;
// varying vec2 vUv;

void vignette(inout vec4 out_color.  vec2 vUv, float offset, float darkness) {
    vec2 uv = ( vUv - vec2( 0.5 ) ) * vec2( offset );
    out_color = vec4( mix( out_color.rgb, vec3( 1.0 - darkness ), dot( uv, uv ) ), out_color.a );
} 
