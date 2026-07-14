pub const VERTEX_SHADER: &str = r#"
    attribute vec2 a_position;

    uniform mat3 u_canvasProjectionMatrix;

    void main() {
        vec2 position = (u_canvasProjectionMatrix * vec3(a_position, 1)).xy;
        gl_Position = vec4(position, 0, 1);
    }
"#;

pub const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    uniform vec2 u_canvas_resolution;
    uniform float u_extension_dimension;

    float inverseLerp(float v, float minValue, float maxValue) {
      return (v - minValue) / (maxValue - minValue);
    }

    float remap(float v, float inMin, float inMax, float outMin, float outMax) {
      float t = inverseLerp(v, inMin, inMax);
      return mix(outMin, outMax, t);
    }

    // First arg is the coord, second is the dimensions from center and third is the radius of the corners
    float sdRoundBox( vec2 p, vec2 b, float r )
    {
      vec2 q = abs(p) - b + r;
      return length(max(q,0.0)) + min(max(q.x,q.y),0.0) - r;
    }

    float circleSdf(vec2 pos, float r)
    {
        return length(pos) - r;
    }

    vec4 paintSdf(vec4 color, float sdf)
    {
        vec4 clear = vec4(0.0);
        return mix(color, clear, smoothstep(-1.0, 1.0, sdf)); //Smoothstep for some small anti aliasing
    }

    // This moves the SDF, assuming the origin to be at the bottom left
    //
    // Also, accoding to what I know now, (I am still building intuition for this) to "move" and SDF to position (x,y), you figure out what operation would
    // normally move it from its desired position (when you want to take it to), to the canvas origin (at the bottom left) and then apply that.
    // For instance, if you want to draw a circle at the center of the canvas, you pinpoint the center of that circle (the circle's position) and then figure
    // out what operation would move that circle to the bottom left. That operaiton is what moves the sdf to that center of the canvas (weird I know xD)
    vec2 placeSdf(vec2 desiredPosition, vec2 coord) {
      return desiredPosition - coord;
    }

    // exponential
    float smin( float a, float b, float k )
    {
        k *= 1.0;
        float r = exp2(-a/k) + exp2(-b/k);
        return -k*log2(r);
    }

    void main() {
        vec4 clear = vec4(0.0);
        vec4 buttonColor = vec4(0.0, 1.0, 1.0, 1.0);

        vec4 color = clear;

        float extension = u_extension_dimension;

        vec2 coord = gl_FragCoord.xy;

        // Write a nice utility to draw the SDF in the scene (use pixel values)

        // Background dimensions without the extension
        vec2 buttonBg = u_canvas_resolution - vec2(extension, 0.0);

        // Move it to the middle of the button BG from the left side of the canvas
        vec2 buttonPosition = placeSdf(
            buttonBg * 0.5,
            coord
        );

        float pill = sdRoundBox(
            buttonPosition,
            buttonBg * 0.5,
            u_canvas_resolution.y * 0.5
        );

        // Temporary Move. Move the circle to the rightmost part of the canvas
        vec2 circlePosition = placeSdf(
             ((u_canvas_resolution) - vec2(10.0, 0.0)) - (extension * 0.5),
             coord
        );

        float circle = circleSdf(circlePosition, extension * 0.5);

        // Draw Circle

        float pillAndCircle = smin(pill, circle, 9.0);

        color += paintSdf(buttonColor, pillAndCircle);

        gl_FragColor = color;
    }
"#;
