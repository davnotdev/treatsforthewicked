use super::*;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

const MAX_SPRITES: usize = 512;

#[derive(SingleResource)]
pub struct RendererInitLoadTextures(pub Vec<TextureData>);

#[derive(SingleResource, Default, Clone, Copy)]
pub struct RendererCamera {
    pub position: glm::Vec2,
}

#[derive(Event)]
pub struct RendererDrawSprite(pub Sprite);

#[derive(SingleResource)]
pub struct RendererRes(Renderer);

#[derive(Default, Clone, Copy)]
struct SpriteUniform {
    pub model: glm::Mat4,
    pub color: glm::Vec3,
    pub visible: f32,
}

pub struct Renderer {
    context: Context,

    vbo: VertexBufferId,
    ibo: IndexBufferId,

    scene_ubo: UniformBufferId,
    scene_ubo_guard: UniformBufferTypeGuard<Scene>,

    sprite_ubo: DynamicUniformBufferId,
    sprite_ubo_guard: DynamicUniformBufferTypeGuard<SpriteUniform>,

    programs: Vec<ProgramId>,
    output_attachment: PassLocalAttachment,
    compiled_pass: CompiledPassId,
}

impl Renderer {
    pub fn new(
        display: RawDisplayHandle,
        window: RawWindowHandle,
        width: usize,
        height: usize,
        textures: &[TextureData],
    ) -> Self {
        let mut extensions = Extensions::new();
        extensions
            .native_debug(NativeDebugConfiguration::default())
            .naga_translation()
            .surface_extension(SurfaceConfiguration {
                width,
                height,
                display,
                window,
            })
            .webgpu_init_from_window(WebGpuInitFromWindow {
                adapter: String::from("mepeyewAdapter"),
                device: String::from("mepeyewDevice"),
                canvas_id: Some(String::from("canvas")),
            });

        let mut context = Context::new(extensions, None).unwrap();

        let vs = include_bytes!("shaders/vs.spv");
        let fs = include_bytes!("shaders/fs.spv");

        let vs = context
            .naga_translate_shader_code(
                naga_translation::NagaTranslationStage::Vertex,
                naga_translation::NagaTranslationInput::Spirv,
                vs,
                naga_translation::NagaTranslationExtensionTranslateShaderCodeExt::default(),
            )
            .unwrap();
        let fs = context
            .naga_translate_shader_code(
                naga_translation::NagaTranslationStage::Fragment,
                naga_translation::NagaTranslationInput::Spirv,
                fs,
                naga_translation::NagaTranslationExtensionTranslateShaderCodeExt::default(),
            )
            .unwrap();

        let shader_set = ShaderSet::shaders(&[
            (
                ShaderType::Vertex(VertexBufferInput { args: vec![2, 2] }),
                &vs,
            ),
            (ShaderType::Fragment, &fs),
        ]);

        let sampler = context.get_sampler(None).unwrap();

        let (scene_ubo, scene_ubo_guard) =
            context.new_uniform_buffer(&Scene::default(), None).unwrap();

        let (sprite_ubo, sprite_ubo_guard) = context
            .new_dynamic_uniform_buffer(&[SpriteUniform::default(); MAX_SPRITES], None)
            .unwrap();

        let programs = textures
            .iter()
            .map(|texture| {
                let texture_id = context
                    .new_texture(texture.width, texture.height, TextureFormat::Rgba, None)
                    .unwrap();
                context
                    .upload_texture(texture_id, &texture.data, None)
                    .unwrap();

                context
                    .new_program(
                        &shader_set,
                        &[
                            ShaderUniform {
                                set: 0,
                                binding: 0,
                                ty: ShaderUniformType::UniformBuffer(scene_ubo),
                            },
                            ShaderUniform {
                                set: 1,
                                binding: 0,
                                ty: ShaderUniformType::DynamicUniformBuffer(sprite_ubo),
                            },
                            ShaderUniform {
                                set: 2,
                                binding: 0,
                                ty: ShaderUniformType::Sampler(sampler),
                            },
                            ShaderUniform {
                                set: 2,
                                binding: 1,
                                ty: ShaderUniformType::Texture(texture_id),
                            },
                        ],
                        Some(NewProgramExt {
                            enable_blend: Some(()),
                            blend_color_operation: Some(ShaderBlendOperation::Add),
                            blend_color_src_factor: Some(ShaderBlendFactor::SrcAlpha),
                            blend_color_dst_factor: Some(ShaderBlendFactor::OneMinusSrcAlpha),
                            blend_alpha_operation: Some(ShaderBlendOperation::Add),
                            blend_alpha_src_factor: Some(ShaderBlendFactor::SrcAlpha),
                            blend_alpha_dst_factor: Some(ShaderBlendFactor::OneMinusSrcAlpha),
                            ..Default::default()
                        }),
                    )
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let vbo = context
            .new_vertex_buffer(quad_vertices(), BufferStorageType::Static, None)
            .unwrap();
        let ibo = context
            .new_index_buffer(quad_indices(), BufferStorageType::Static, None)
            .unwrap();

        let mut pass = Pass::new(
            width,
            height,
            Some(NewPassExt {
                depends_on_surface_size: Some(()),
                surface_attachment_load_op: Some(PassInputLoadOpColorType::Clear),
            }),
        );

        let output_attachment = pass.get_surface_local_attachment();
        {
            let pass_step = pass.add_step();
            pass_step
                .add_vertex_buffer(vbo)
                .set_index_buffer(ibo)
                .add_write_color(output_attachment);

            for program in programs.iter().copied() {
                pass_step.add_program(program);
            }
        }

        let compiled_pass = context.compile_pass(&pass, None).unwrap();

        Self {
            context,

            vbo,
            ibo,

            scene_ubo,
            scene_ubo_guard,

            sprite_ubo,
            sprite_ubo_guard,

            programs,
            output_attachment,
            compiled_pass,
        }
    }

    pub fn draw(&mut self, camera: &RendererCamera, sprites: Vec<Sprite>) {
        let mut submit = Submit::new();

        let view = glm::translate(
            &glm::identity(),
            &glm::vec3(-camera.position.x, -camera.position.y, 0.0),
        );

        let scene = Scene { view };
        submit.transfer_into_uniform_buffer(self.scene_ubo_guard, &scene);

        let mut sprite_uniforms = sprites
            .iter()
            .map(|s| {
                let model = glm::identity();
                let model = glm::translate(&model, &glm::vec3(s.position.x, s.position.y, 0.0));
                let model = glm::rotate(&model, s.rotation, &glm::vec3(0.0, 0.0, 1.0));
                let model = glm::scale(&model, &glm::vec3(s.scale.x, s.scale.y, 1.0));

                SpriteUniform {
                    visible: s.visible,
                    color: s.color,
                    model,
                }
            })
            .collect::<Vec<_>>();
        sprite_uniforms.resize(MAX_SPRITES, SpriteUniform::default());
        sprite_uniforms
            .iter()
            .enumerate()
            .for_each(|(idx, obj_data)| {
                submit.transfer_into_dynamic_uniform_buffer(self.sprite_ubo_guard, obj_data, idx);
            });

        let mut pass_submit = PassSubmitData::new(self.compiled_pass);

        {
            let mut step_submit = StepSubmitData::new();

            for (dyn_idx, sprite) in (0..sprite_uniforms.len()).zip(sprites.iter()) {
                step_submit
                    .draw_indexed(self.programs[sprite.texture_index], 0, quad_indices().len())
                    .set_dynamic_uniform_buffer_index(self.sprite_ubo, dyn_idx);
            }

            pass_submit.set_attachment_clear_color(
                self.output_attachment,
                ClearColor {
                    r: 0.027,
                    g: 0.01,
                    b: 0.08,
                    a: 1.0,
                },
            );
            pass_submit.step(step_submit);
        }

        submit.pass(pass_submit);
        self.context.submit(submit, None).unwrap();
    }
}

pub fn graphics_init(galaxy: &Galaxy) {
    let raw_window = galaxy
        .get_resource::<window::RawWindowRes, _>(window::RawWindowRes::single_resource())
        .unwrap();

    let textures = galaxy
        .get_resource::<RendererInitLoadTextures, _>(RendererInitLoadTextures::single_resource())
        .unwrap();

    let renderer = Renderer::new(raw_window.display, raw_window.window, 800, 600, &textures.0);

    galaxy.insert_resource(RendererRes::single_resource(), RendererRes(renderer));
    galaxy.insert_resource(RendererCamera::single_resource(), RendererCamera::default());
}

pub fn graphics_update(galaxy: &Galaxy) {
    let mut renderer = galaxy
        .get_mut_resource::<RendererRes, _>(RendererRes::single_resource())
        .unwrap();

    for ev in galaxy.get_events::<WindowResize>().iter() {
        renderer
            .0
            .context
            .set_surface_size(ev.width, ev.height)
            .unwrap();
    }

    let camera = galaxy
        .get_resource::<RendererCamera, _>(RendererCamera::single_resource())
        .unwrap();

    let sprites = galaxy
        .get_events::<RendererDrawSprite>()
        .iter()
        .map(|RendererDrawSprite(sprite)| sprite)
        .copied()
        .collect::<Vec<_>>();

    renderer.0.draw(&camera, sprites);
}
