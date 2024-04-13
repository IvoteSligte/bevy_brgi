from os import listdir
from os.path import isfile, join

path = "src/shaders"
files = [f for f in listdir(path) if isfile(join(path, f))]

for shader in files:
    base = shader.split('.')[0]

    print('load_internal_asset!(')
    print('\tapp,')
    print('\t' + base.upper() + '_SHADER_HANDLE,')
    print('\t"shaders/' + shader + '",')
    print("\tShader::from_wgsl")
    print(');')
