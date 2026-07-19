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
    uniform float u_progression;
    uniform vec4 u_base_color;
    uniform vec4 u_hover_color;

    vec3 srgbToLinear(vec3 c) {
        return pow(c, vec3(2.2)); // or the exact piecewise sRGB curve if you want precision
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

    // quadratic polynomial
    float smin( float a, float b, float k )
    {
        k *= 4.0;
        float h = max( k-abs(a-b), 0.0 )/k;
        return min(a,b) - h*h*k*(1.0/4.0);
    }

    void main() {
        vec4 clear = vec4(srgbToLinear(vec3(0.0)).xyz, 0.0);
        vec4 baseColor = vec4(srgbToLinear(vec3(u_base_color.xyz) / 225.0).xyz, u_base_color.a);
        vec4 hoverColor =  vec4(srgbToLinear(vec3(u_hover_color.xyz) / 225.0).xyz, u_hover_color.a);

        vec4 buttonColor =  mix(baseColor, hoverColor, u_progression);

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

        // Place Circle flush with right side of pill
        vec2 posCircleWithPillBorder = buttonBg - vec2(extension * 0.5);

        // Start the circle from within the pill
        vec2 circlePosition = placeSdf(
             posCircleWithPillBorder + vec2(mix(-0.5 * extension, extension, u_progression), 0.0), // Lerp position
             coord
        );

        // Multiplying by some scaling constant to make it slightly smaller and look nicer (preference)
        float circleRadius = ((extension - 5.0) * 0.5) * 0.7;

        float circle = circleSdf(circlePosition, circleRadius);

        // Draw Circle

        float pillAndCircle = smin(pill, circle, extension * 0.05);

        // Leaving these comments here because they are useful for debugging
        // color += paintSdf(buttonColor, pill);
        // color += paintSdf(vec4(1.0, 0.0, 0.0, 1.0), circle);
        color += paintSdf(buttonColor, pillAndCircle);

        gl_FragColor = color;
    }
"#;
