pub const VERTEX_SHADER: &str = r#"
  attribute vec2 a_position;
  attribute vec2 a_texCoord;

  uniform mat3 u_textureProjectionMatrix;
  uniform mat3 u_canvasProjectionMatrix;

  varying vec2 v_texCoord;

  void main() {
    vec2 position = (u_canvasProjectionMatrix * vec3(a_position, 1)).xy;
    gl_Position = vec4(position, 0, 1);
    v_texCoord = (u_textureProjectionMatrix * vec3(a_texCoord, 1)).xy;
  }
"#;

pub const FRAGMENT_SHADER: &str = "
precision mediump float;

uniform float u_duration;
uniform float u_revealDuration;
uniform vec2 u_textureResolution;
uniform sampler2D u_mainImage;
uniform sampler2D u_maskImage;

varying vec2 v_texCoord;

float inverseLerp(float v, float minValue, float maxValue) {
    return (v - minValue) / (maxValue - minValue);
}

float remap(float v, float inMin, float inMax, float outMin, float outMax) {
    float t = inverseLerp(v, inMin, inMax);
    return mix(outMin, outMax, t);
}

void main() {
    float numberOfPieces = 54.0;
    vec3 backgroundColor = vec3(0.012, 0.012, 0.012);
    vec4 imageColor = texture2D(u_mainImage, v_texCoord);
    vec4 maskColor = texture2D(u_maskImage, v_texCoord);

    float greenValue = maskColor.g * 255.0;
    float pieceIndex = floor(greenValue + 0.5) - 1.0;

    float pieceCols = 9.0;
    float pieceRows = 6.0;
    float pieceX = mod(pieceIndex, pieceCols);
    float pieceY = floor(pieceIndex / pieceCols);

    float centerPieceX = mod(31.0, pieceCols); // 4.0
    float centerPieceY = floor(31.0 / pieceCols); // 3.0

    // Use either Manhattan or Euclidean distance
    float distanceFromCenter = abs(pieceX - centerPieceX) + abs(pieceY - centerPieceY);
    // float distanceFromCenter = distance(vec2(pieceX, pieceY), vec2(centerPieceX, centerPieceY));

    float maxDistance = pieceCols + pieceRows;
    float perRingTime = u_revealDuration / maxDistance;

    // Smooth fade per piece
    float pieceRevealTime = distanceFromCenter * perRingTime;
    float fade = smoothstep(pieceRevealTime, pieceRevealTime + perRingTime, u_duration);

    // Edge detection
    vec2 texelSize = vec2(1.0 / u_textureResolution.x, 1.0 / u_textureResolution.y);
    vec2 offsetRight = vec2(texelSize.x, 0.0);
    vec2 offsetLeft = vec2(-texelSize.x, 0.0);
    vec2 offsetUp = vec2(0.0, texelSize.y);
    vec2 offsetDown = vec2(0.0, -texelSize.y);

    float neighborG1 = texture2D(u_maskImage, v_texCoord + offsetRight).g * 255.0;
    float neighborG2 = texture2D(u_maskImage, v_texCoord + offsetLeft).g * 255.0;
    float neighborG3 = texture2D(u_maskImage, v_texCoord + offsetUp).g * 255.0;
    float neighborG4 = texture2D(u_maskImage, v_texCoord + offsetDown).g * 255.0;

    float deltaG1 = abs(neighborG1 - greenValue);
    float deltaG2 = abs(neighborG2 - greenValue);
    float deltaG3 = abs(neighborG3 - greenValue);
    float deltaG4 = abs(neighborG4 - greenValue);

    float edgeThreshold = 0.5;
    float isEdge = float(max(max(deltaG1, deltaG2), max(deltaG3, deltaG4)) > edgeThreshold);

    float isBlue = float(maskColor.b > 0.5);
    float isRed = float(maskColor.r > 0.5);
    float isBlueOrBlend = max(isBlue, isRed * isBlue);

    float opacity = 0.0;
    vec3 color = backgroundColor;

    if (isBlueOrBlend > 0.0) {
        color = mix(backgroundColor, imageColor.rgb, maskColor.b);
    } else {
        opacity = fade;
        color = mix(backgroundColor, imageColor.rgb, opacity);
    }

    if (isEdge > 0.0) {
        color = backgroundColor;
    }

    gl_FragColor = vec4(color, opacity);
}
";
