use std::sync::mpsc::Receiver;

use crate::*;
use crate::shared::*;

use shaderc::*;


static VSHADER: &str = r"
                    #version 460

                    layout(location = 0) in vec2 position;
                    layout(location = 1) in vec3 color;

                    layout(location = 0) out vec3 fragColor;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                        fragColor = color;
                    }
                ";

static FSHADER: &str = r"
                    #version 460

                    layout(location = 0) in vec3 fragColor;
                    layout(location = 0) out vec4 outColor;

                    void main() {
                        outColor = vec4(fragColor, 1.0);
                    }
                ";

static VSHADER_2: &str = r"
                    #version 460

                    layout(location = 0) in vec2 position;
                    layout(location = 1) in vec3 color;

                    layout(location = 0) out vec3 fragColor;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                        fragColor = color;
                    }
                ";

static FSHADER_2: &str = r"
                    #version 460

                    layout(location = 0) in vec3 fragColor;
                    layout(location = 0) out vec4 outColor;

                    void main() {
                        outColor = vec4(1.0, 0.0, 0.0, 1.0);
                    }
                ";

fn compile_glsl(shared: &SharedData,code: &str, kind: ShaderKind, c: &Compiler, o: &CompileOptions) -> Option<Vec<u32>>{
    return match c.compile_into_spirv(
        code, kind,
        "shader.glsl", "main", Some(o)) {
            Ok(v) => Some(Vec::from(v.as_binary())),
            Err(e) => {
                err!(shared, "Failed to compile shader: {e}");
                return None
            },
        }
}

pub fn main(shared: SharedData, rx: Receiver<AssetRequest>){
    let c;
    let o;

    if let Ok(mut _lock) = shared.asset_init.lock() {
        c = Compiler::new().unwrap_or_else(|e| {
            fatal!(shared,"Failed to create shader compiler: {e}");
        });
        o = CompileOptions::new().unwrap_or_else(|e| {
            fatal!(shared,"Failed to create shader compiler options: {e}");
        });

        info!(shared,"Asset thread initialized");
    } else {
        fatal!(shared, "Failed to aquire asset initialization lock");
    }

    //HACK: dummy backend
    //TODO: implement actual asset pack
    for req in rx {
        let ass: Asset = match req.name.as_str() {
            "test_vs" =>  match compile_glsl(&shared,VSHADER,ShaderKind::Vertex,&c,&o) {
                                Some(d) => Asset::Shader(d),
                                Option::None => Asset::None
                            },
            "test_fs" => match compile_glsl(&shared,FSHADER,ShaderKind::Fragment,&c,&o) {
                                Some(d) => Asset::Shader(d),
                                Option::None => {
                                    Asset::None
                                }
                            },
            "test_vs2" =>  match compile_glsl(&shared,VSHADER_2,ShaderKind::Vertex,&c,&o) {
                                Some(d) => Asset::Shader(d),
                                Option::None => Asset::None
                            },
            "test_fs2" => match compile_glsl(&shared,FSHADER_2,ShaderKind::Fragment,&c,&o) {
                                Some(d) => Asset::Shader(d),
                                Option::None => {
                                    Asset::None
                                }
                            },
            _ => Asset::None
        };

        req.sender.send(ass).unwrap_or_else(|e| {
            err!(shared, "Failed to send asset: {e}")
        })
    }

}