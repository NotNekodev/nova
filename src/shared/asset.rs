use std::string::String;

// Make Asset public so it can appear in a public function’s signature.
pub enum Asset {
    ShaderCode(String), //sorry you gotta compile GLSL for now
    None
}


static vshader: &str = r"
                    #version 460

                    layout(location = 0) in vec2 position;
                    layout(location = 1) in vec3 color;

                    layout(location = 0) out vec3 fragColor;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                        fragColor = color;
                    }
                ";

static fshader: &str = r"
                    #version 460

                    layout(location = 0) in vec3 fragColor;
                    layout(location = 0) out vec4 outColor;

                    void main() {
                        outColor = vec4(fragColor, 1.0);
                    }
                ";



pub fn get_asset(name: &str) -> Asset {
    //TODO: make a real backend for this
    match name {
        "vshader" => Asset::ShaderCode(vshader.to_string()),
        "fshader" => Asset::ShaderCode(fshader.to_string()),
        _ => Asset::None
    }
}