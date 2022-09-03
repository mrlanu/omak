#shader vertex
#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTex;

out vec2 TexCoords;

uniform mat4 model;
uniform mat4 projection;

void main()
{
    TexCoords = aTex;
    gl_Position = projection * model * vec4(aPos.x, aPos.y, 0.0, 1.0);
}


#shader fragment
#version 330 core
in vec2 TexCoords;
out vec4 color;

uniform sampler2D image;
uniform vec3 spriteColor;

void main()
{    
    color = vec4(spriteColor, 1.0) * texture(image, TexCoords);
}  
