// Usually graphics driver is the biggest thing. Can load the graphics driver straight away and tell the GPU to render the arcboot graphics with vulkan
// More advanced features like RAID features, RGB device lighting, Surround Sound, etc can be kernel drivers loaded when the kernel starts up
// BIOS should have basically no drivers. If firmware big enough can maybe place some extra stuff but assume mostly interfaces very primitively, initialising, multiplexing and closing

struct GraphicsDriver;

impl GraphicsDriver {
    fn run_vulkan_program(&self, graphics_program: &GraphicsProgram) {}
}

// what to do with VRAM -> maybe we are able to load most of the assets, textures, 3d models (list of vertices and options) to VRAM beforehand
// maybe we are able to load it in binary format or somehow organise them in a pseudo hierarchy within VRAM
// then use the bottom parts of VRAM to store temp data. IF not enough space then use RAM
// GPU only gets called with glDoX gets called on the state machine within VRAM
// glSpecify to specify the vertex ordering and stuff
struct GraphicsProgram;

impl GraphicsProgram {
    //technically CPU controls a lot of it
    //from the loop to the shader.use()
    //GPU gets action when shader.use() is called, as the compiled shader should be in VRAM already to be used as a program
    //uniforms need to be passed from RAM to GPU VRAM/registers at each cycle
    //when glDrawElements is called, that is when the action is. Registers work to crunch numbers and output a framebuffer
    //glClear is when CPU tells GPU to clear the framebuffer, drawing it to the screen
    fn render_loop(&self) {
        // assert vulkan_init

        loop {
            // do_something
            // vulkan_draw_elements()
            // vulkan_clear()
        }
    }

    // implement draw_elements() by telling GPU to do 3D level stuff
    fn vulkan_draw_implementation() {
        // gpu_set_reg(reg_num, val)
        // gpu_set_reg(reg_num_2, val2)

        // textures
        // gpu_texture_sample(texture_img, coords)
    }
}

fn make_graphics_program() -> GraphicsProgram {
    // attach CPU listener to specific things like element positions
    // on click, call functions to handle and update state
    // the state updates in RAM and when shader.use() is called on a new state
    // GPU will have access to new state

    GraphicsProgram {}
}

#[test]
fn test_graphics_driver() {
    // create graphics driver manager
    let graphics_driver = GraphicsDriver {};

    // create a vulkan graphics program based on arcboot GUI
    // includes extra logic to listen to mouse events
    let arcboot_gui = make_graphics_program();

    // start making vulkan calls to draw the gui/output to monitor
    graphics_driver.run_vulkan_program(&arcboot_gui);
}
