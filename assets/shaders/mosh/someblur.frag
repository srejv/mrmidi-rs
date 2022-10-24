// gm = { uniforms: { tDiffuse: { type: "t", value: null }, amount: { type: "f", value: .5 }, size: { type: "f", value: 4 }, darkness: { type: "f", value: .1 }, resolution: { type: "v2" } }, 
// vertexShader: "" + "\t${cm}" + "\t", 

// uniform sampler2D tDiffuse; 
// uniform float size; 
// uniform float amount; 
// uniform vec2 resolution; 
// uniform float darkness;  
 
// varying vec2 vUv; 
 
void some_blur(inout vec4 out_color, sampler2D channel, vec2 vUv, vec2 resolution, float size, float amount, float darkness) { 
    float h = size / resolution.x; 
    float v = size / resolution.y;  
    
    vec4 sum = vec4( 0.0 ); 
    
    //H Blur 
    sum += (texture2D( channel, vec2( vUv.x - 4.0 * h, vUv.y ) )- darkness) * 0.051 ; 
    sum += (texture2D( channel, vec2( vUv.x - 3.0 * h, vUv.y ) )- darkness) * 0.0918; 
    sum += (texture2D( channel, vec2( vUv.x - 2.0 * h, vUv.y ) )- darkness) * 0.12245; 
    sum += (texture2D( channel, vec2( vUv.x - 1.0 * h, vUv.y ) )- darkness) * 0.1531;  
    sum += (texture2D( channel, vec2( vUv.x, vUv.y ) )- darkness) * 0.1633; 
    sum += (texture2D( channel, vec2( vUv.x + 1.0 * h, vUv.y ) )- darkness) * 0.1531; 
    sum += (texture2D( channel, vec2( vUv.x + 2.0 * h, vUv.y ) )- darkness) * 0.12245; 
    sum += (texture2D( channel, vec2( vUv.x + 3.0 * h, vUv.y ) )- darkness) * 0.0918;  
    sum += (texture2D( channel, vec2( vUv.x + 4.0 * h, vUv.y ) )- darkness) * 0.051;  
    
    //V Blur 
    sum += (texture2D( channel, vec2( vUv.x, vUv.y - 4.0 * v ) )- darkness) * 0.051; 
    sum += (texture2D( channel, vec2( vUv.x, vUv.y - 3.0 * v ) )- darkness) * 0.0918; 
    sum += (texture2D( channel, vec2( vUv.x, vUv.y - 2.0 * v ) )- darkness) * 0.12245; 
    sum += (texture2D( channel, vec2( vUv.x, vUv.y - 1.0 * v ) )- darkness) * 0.1531;  
    sum += (texture2D( channel, vec2( vUv.x, vUv.y ) )- darkness) * 0.1633; 
    sum += (texture2D( channel, vec2( vUv.x, vUv.y + 1.0 * v ) )- darkness) * 0.1531; 
    sum += (texture2D( channel, vec2( vUv.x, vUv.y + 2.0 * v ) )- darkness) * 0.12245; 
    sum += (texture2D( channel, vec2( vUv.x, vUv.y + 3.0 * v ) )- darkness) * 0.0918; 
    sum += (texture2D( channel, vec2( vUv.x, vUv.y + 4.0 * v ) )- darkness) * 0.051; 
    
    //get original pixel color 
    vec4 base = texture2D( tDiffuse, vUv ); 
    
    //Additive Blend 
    out_color = base + max(sum,0.0) * amount; 
} 
