#shader vertex
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTex;

out vec3 color;
out vec2 texCoord;

uniform float aSc;

void main()
{
  gl_Position = vec4(aPos.x + aPos.x * aSc, aPos.y + aPos.y * aSc, aPos.z + aPos.z * aSc, 1.0);
  color = aColor;
  texCoord = aTex;
};

#shader fragment
#version 330 core
out vec4 FragColor;

in vec3 color;
in vec2 texCoord;

uniform sampler2D tex0;

void main()
{
  FragColor = texture(tex0, texCoord);
};


