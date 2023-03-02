# OpenGL 


## Creating a Screen

### Drawing to Screen

`glClear(GLbifield mask)` Clear the mask to preset values.

The three masks are `GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, and GL_STENCIL_BUFFER_BIT`

`GL_COLOR_BUFFER_BIT` is the buffer currently enabled for drawing color.

Use `glClearColor(r, g, b, a)` to set the default color to be rendered when `glClear()` is called.


## Commonly used APIs

### Front facing and Back facing

`glEnable(GL_CULL_FACE)` will enable opengl to cull (i.e., stop drawing) the face that is not facing the screen.

By default only render the triangles that are facing the screen, i.e. with counterclockwise vertices.

Use `glFrontFace(CW)` to make clockwise facing to the screen. (CCW for counterclockwise)

Use `glCullFace(GLenum mode)` to specified which facing are culled. Options are `GL_FRONT`, `GL_BACK`, and `GL_FRONT_AND_BACK`.


## GL Data Types

For more details, see [OpenGL Data Types](https://www.khronos.org/opengl/wiki/OpenGL_Type).

- `GLClampf` : a floating-point value clamped to the range [0,1].
