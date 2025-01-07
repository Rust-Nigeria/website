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

  uniform sampler2D u_mainImage;
  uniform sampler2D u_maskImage;

  varying vec2 v_texCoord;

  void main() {
    vec3 baseColor = vec3(0.012,0.012,0.012);
    vec4 imageColor = texture2D(u_mainImage, v_texCoord);
    vec4 maskColor = texture2D(u_maskImage, v_texCoord);
    
    float opacity = 1.0 - maskColor.b;

    vec3 bg = baseColor * maskColor.b;
    vec3 img = imageColor.rgb * opacity;

    gl_FragColor = vec4(bg + img, 1.0);
  }
";
