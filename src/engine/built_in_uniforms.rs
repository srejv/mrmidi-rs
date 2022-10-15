use macroquad::prelude::*;

use crate::engine::internal_uniform::InternalUniform;
/*
Shadertoy Inputs
vec3	iResolution	image/buffer	The viewport resolution (z is pixel aspect ratio, usually 1.0)
float	iTime	image/sound/buffer	Current time in seconds
float	iTimeDelta	image/buffer	Time it takes to render a frame, in seconds
int	iFrame	image/buffer	Current frame
float	iFrameRate	image/buffer	Number of frames rendered per second
float	iChannelTime[4]	image/buffer	Time for channel (if video or sound), in seconds
vec3	iChannelResolution[4]	image/buffer/sound	Input texture resolution for each channel
vec4	iMouse	image/buffer	xy = current pixel coords (if LMB is down). zw = click pixel
sampler2D	iChannel{i}	image/buffer/sound	Sampler for input textures i
vec4	iDate	image/buffer/sound	Year, month, day, time in seconds in .xyzw
float	iSampleRate	image/buffer/sound	The sound sample rate (typically 44100)
*/
pub struct BuiltInUniforms {
    pub i_resolution: InternalUniform,
    pub i_date: InternalUniform,

    pub i_sample_rate: InternalUniform,

    pub i_time: InternalUniform,
    pub i_time_delta: InternalUniform,

    pub i_frame: InternalUniform,
    pub i_frame_rate: InternalUniform,

    pub i_mouse: InternalUniform,

    pub i_channel: [Texture2D; 4],
    pub i_channel_resolution: [InternalUniform; 4],
    pub i_channel_time: [InternalUniform; 4],
}
impl BuiltInUniforms {
    pub fn new() -> Self {
        Self {
            i_resolution: InternalUniform::Float3(0.0, 0.0, 0.0),
            i_time: InternalUniform::Float1(0.0),
            i_time_delta: InternalUniform::Float1(0.0),
            i_frame: InternalUniform::Int(0),
            i_frame_rate: InternalUniform::Float1(0.0),
            i_channel_time: [
                InternalUniform::Float1(0.0),
                InternalUniform::Float1(0.0),
                InternalUniform::Float1(0.0),
                InternalUniform::Float1(0.0),
            ],
            i_channel_resolution: [
                InternalUniform::Float3(0.0, 0.0, 0.0),
                InternalUniform::Float3(0.0, 0.0, 0.0),
                InternalUniform::Float3(0.0, 0.0, 0.0),
                InternalUniform::Float3(0.0, 0.0, 0.0),
            ],
            i_mouse: InternalUniform::Float4(0.0, 0.0, 0.0, 0.0),
            i_date: InternalUniform::Float4(0.0, 0.0, 0.0, 0.0),
            i_sample_rate: InternalUniform::Float1(0.0),
            i_channel: [
                Texture2D::empty(),
                Texture2D::empty(),
                Texture2D::empty(),
                Texture2D::empty(),
            ],
        }
    }

    pub fn update_time(&mut self, time: f32, delta_time: f32) {
        self.i_time = InternalUniform::Float1(time);
        self.i_time_delta = InternalUniform::Float1(delta_time);
    }

    pub fn update_frame(&mut self, frame: i32, frame_rate: f32) {
        self.i_frame = InternalUniform::Int(frame);
        self.i_frame_rate = InternalUniform::Float1(frame_rate);
    }

    pub fn update_mouse(&mut self, x: f32, y: f32, pressed: f32, held: f32) {
        self.i_mouse = InternalUniform::Float4(x, y, pressed, held);
    }

    pub fn update_date(&mut self, year: f32, month: f32, day: f32, second: f32) {
        self.i_date = InternalUniform::Float4(year, month, day, second);
    }
}
