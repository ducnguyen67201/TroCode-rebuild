from PyInstaller.utils.hooks import collect_all, copy_metadata
from pathlib import Path

project = Path(SPECPATH).parent
native_data, native_binaries, native_imports = collect_all('cua_driver')
agents_data, agents_binaries, agents_imports = collect_all('agents')
a = Analysis([str(project / 'packaging/entry.py')],
    pathex=[str(project / 'src')],
    binaries=native_binaries + agents_binaries,
    datas=native_data + agents_data + copy_metadata("openai-agents", recursive=True) + [
        (str(project / 'src/tro_runtime/schema.json'), 'tro_runtime'),
        (str(project / 'src/tro_runtime/digest.txt'), 'tro_runtime'),
        (str(project / 'src/tro_runtime/resources/read-only.json'), 'tro_runtime/resources'),
    ], hiddenimports=native_imports + agents_imports,
    hookspath=[str(project / 'packaging/hooks')], noarchive=False)
pyz = PYZ(a.pure)
exe = EXE(pyz, a.scripts, [], exclude_binaries=True, name='tro-runtime',
    debug=False, bootloader_ignore_signals=False, strip=False, upx=False, console=True)
coll = COLLECT(exe, a.binaries, a.datas, strip=False, upx=False, name='tro-runtime')
