from os import listdir
from os.path import isfile, join
from random_uuid import random_uuid

path = "src/shaders"
files = [f for f in listdir(path) if isfile(join(path, f))]

for shader in files:
    base = shader.split('.')[0]

    print("const " + base.upper() +
          "_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(" + str(random_uuid()) + ");")
