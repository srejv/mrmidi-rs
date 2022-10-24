// ym = { uniforms: { tDiffuse: { type: "t", value: null }, resolution: { type: "v2" }, scale: { type: "f", value: 0 }, noiseScale: { type: "f", value: .1 }, centerX: { type: "f", value: .5 } }, 
// vertexShader: "" + "\t\t${cm}" + "\t", 

// uniform sampler2D tDiffuse; 
// uniform vec2 resolution; 
// varying vec2 vUv; 
// uniform float scale; 
// uniform float noiseScale; 
// uniform float centerX; 
 
float luma(vec3 color) { 
    return dot(color, vec3(0.299, 0.587, 0.114)); 
} 
 
void noise_mix(inout vec4 out_color, sampler2D channel, vec2 vUv, vec2 resolution, float scale, float noiseScale, float centerX) { 
    
    vec2 center = vec2( 0.5 ); 
    center.x = centerX; 
    vec2 uv = vUv; 
    
    //float noiseScale = 0.1; 
    float radius = 0.5; 
    vec2 d = uv - center; 
    float r = length( d * vec2( 1., resolution.y / resolution.x ) ) * scale; 
    float a = atan(d.y,d.x) + noiseScale*(radius-r)/radius; 
    vec2 uvt = center+r*vec2(cos(a),sin(a)); 
    
    vec2 uv2 = vUv; 
    float c = ( .75 + .25 * sin( uvt.x * 1000. ) ); 
    vec4 color = texture2D( channel, uv2 ); 
    float l = luma( color.rgb ); 
    float f = smoothstep( .5 * c, c, l ); 
    f = smoothstep( 0., .5, f ); 
    
    vec3 col = vec3(f); 
    
    out_color = vec4( col,.0); 
}
