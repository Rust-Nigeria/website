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
  vec3 backgroundColor = vec3(0.012, 0.012, 0.012); // Background color
  vec4 imageColor = texture2D(u_mainImage, v_texCoord);
  vec4 maskColor = texture2D(u_maskImage, v_texCoord);

  float perPieceTime = u_revealDuration / numberOfPieces;
  float currentAnimPieceVal = u_duration / perPieceTime;

  float currentAnimPieceIndex = floor(currentAnimPieceVal) + 1.0;
  float pieceProgress = fract(currentAnimPieceVal);

  // Map maskColor.g to piece index
  float greenValue = maskColor.g * 255.0; // Convert normalized [0, 1] to [0, 255]
  float currentPieceIndex = floor(greenValue + 0.5) - 1.0; // Round to nearest index

  vec2 texelSize = vec2(1.0 / u_textureResolution.x, 1.0 / u_textureResolution.y);

  // Offsets for neighboring pixels
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

  float edgeThreshold = 0.5; // Tolerance for blending
  float isEdge = float(max(max(deltaG1, deltaG2), max(deltaG3, deltaG4)) > edgeThreshold);

  // Visibility logic
  float pieceVisibilityMultiplier = step(currentPieceIndex, currentAnimPieceIndex);

  float opacity = 0.0;
  vec3 color = backgroundColor;

  // Detect blue or blue-red blends in the mask
  float isBlue = float(maskColor.b > 0.5); // Convert bool to float
  float isRed = float(maskColor.r > 0.5);  // Convert bool to float
  float isBlueOrBlend = max(isBlue, isRed * isBlue); // Blue or red-blue blend

  if (isBlueOrBlend > 0.0) {
      // Blend background and image color based on the blue component
      color = mix(backgroundColor, imageColor.rgb, maskColor.b);
  } else if (pieceVisibilityMultiplier == 1.0) {
      if (currentPieceIndex == currentAnimPieceIndex) {
          opacity = pieceProgress; // Smooth fade-in for the current piece
      } else {
          opacity = 1.0; // Fully revealed pieces
      }

      color = mix(backgroundColor, imageColor.rgb, opacity); // Blend smoothly
  }

  // Apply edge blending
  if (isEdge > 0.0) {
      color = backgroundColor;
  }

  gl_FragColor = vec4(color, opacity);
}
";
