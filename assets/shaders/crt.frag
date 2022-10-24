#version 100
precision lowp float;
varying vec4 color;
varying vec2 uv;


uniform float iTime;

uniform sampler2D Texture;

uniform sampler2D iChannel0;
uniform sampler2D iChannel1;

uniform sampler2D iMidi;
uniform sampler2D iAudio;

uniform vec3 iResolution;



// https://www.shadertoy.com/view/XtlSD7
vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}
void DrawVignette( inout vec3 color, vec2 uv )
{    
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}
void DrawScanline( inout vec3 color, vec2 uv )
{
    // float iTime = 0.1;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );    
    color *= scanline * grille * 1.2;
}

vec4 dot_matrix_sampler(sampler2D channel, vec2 vUv, float dots, float size, float blur) {
    float dotSize = 1.0/dots; 
    vec2 samplePos = vUv - mod(vUv, dotSize) + 0.5 * dotSize; 
    float distanceFromSamplePoint = distance(samplePos, vUv);  
    vec4 col = texture2D(channel, samplePos);
    return mix(col, vec4(0.0), smoothstep(dotSize * size, dotSize *(size + blur), distanceFromSamplePoint));
}

//get float luma from color
float luma(vec3 color) { 
    return dot(color, vec3(0.299, 0.587, 0.114)); 
} 
 
//boost contrast 
vec3 boostContrast(vec3 col, float amount){  
    return  (col - 0.5) / (1.0 - amount) + 0.5; 
}

void two_color_filter(inout vec4 out_color, vec3 colLight, vec3 colDark) {
    vec3 col =  out_color.rgb; // texture2D(tDiffuse, vUv).rgb; 
    //col += brightness;
    //col = boostContrast(col,contrast); 
    col = clamp(col, 0.0, 1.0); 
    col = mix(colDark, colLight, luma(col)); 
    out_color = vec4(col, out_color.a);
}

const float TWO_PI = 6.283185307179586;

void weird_dist(inout vec4 out_color, sampler2D channel, vec2 vUv, float strength, float size, float speed) {
    vec2 p = -1.0 + 2.0 * vUv;
    float pos = iTime * TWO_PI + length(p * size);
    out_color = texture2D(channel, vUv + strength * vec2(cos(pos), sin(pos)));
} 


void main() {
    
    vec2 crtUV = CRTCurveUV(uv);
    
    // vec3 res = texture2D(Texture, uv).rgb * color.rgb;
    vec2 mehuv = vec2(1.0-crtUV.x, 1.0-crtUV.y);
    // vec4 channel0 = texture2D(iChannel0, mehuv);

    vec4 channel0 = dot_matrix_sampler(iChannel0, mehuv, 40.0, 1.0, 0.5);

    vec4 channel1 = texture2D(iChannel1, mehuv).rgba;
    two_color_filter(channel1, vec3(0.2, 0.8, 0.3), vec3(0.1,0.3,0.1));
    vec4 res = mix(channel0, channel1, sin(iTime) * 0.5 + 0.5) * color.rgba;
 	
    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    {
        res = vec4(0.0, 0.0, 0.0, 1.0);
    } 
    vec3 res2 = res.rgb;
    DrawVignette(res2, crtUV);
    DrawScanline(res2, uv);

    vec3 midi_input = texture2D(iMidi, uv).rgb;
    vec3 audio_input = texture2D(iAudio, uv).rgb;
    // res = vec3(res.x * midi_input.x, res.y * midi_input.y, res.z * midi_input.z);

    gl_FragColor = vec4(res2.rgb, 1.0);
}