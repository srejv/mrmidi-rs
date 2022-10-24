void rgb_shift(inout vec4 color, float amount, float angle, sampler2D channel) {
    vec2 offset = amount * vec2( cos(angle), sin(angle));
    vec4 cr = texture2D(channel, uv + offset);
    vec4 cga = texture2D(channel, uv);
    vec4 cb = texture2D(channel, uv - offset);
    color = vec4(cr.r, cga.g, cb.b, cga.a);
}