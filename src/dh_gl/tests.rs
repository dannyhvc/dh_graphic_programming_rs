#[test]
fn test_read_shader_file_into_string() {
    let (v_shader, f_shader) = crate::dh_gl::common::dhgl_read_shader_file_into_string(
        "./src/res/shaders/vertexshader.glsl",
        "./src/res/shaders/fragmentshader.glsl",
    );

    const vertexShadertxt: &str = "#version 330 core\r\nlayout (location = 0) in vec3 aPos;\r\nvoid main() {\r\n    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);\r\n}\r\n";
    const fragmentShadertxt: &str = "#version 330 core\r\nout vec4 FragColor;\r\nvoid main() {\r\n    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);\r\n}\r\n";

    assert_eq!(v_shader, String::from(vertexShadertxt));
    assert_eq!(f_shader, String::from(fragmentShadertxt));
}
