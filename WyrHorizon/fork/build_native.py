"""Assemble one launcher-native bundle with matching code and content pins."""
from pathlib import Path
import importlib.util,json,re,shutil,sys,zipfile,hashlib
ROOT=Path(__file__).resolve().parent.parent
sys.path.insert(0,str(ROOT/'fork/python-deps'))
import tomli
sys.path.insert(0,str(ROOT/'fork'))
REV='3a817511807aa1056cfcf2598a05180737751cc1'
MIXINS='a2af5c0dd4117d51ecd70066ba7a00b4a2c10dab'
VERSION='0.3.0'

def value(v):
    if isinstance(v,str):return json.dumps(v,ensure_ascii=False)
    if isinstance(v,bool):return str(v).lower()
    if isinstance(v,list):return '['+', '.join(value(x) for x in v)+']'
    if isinstance(v,dict):return '{ '+', '.join(k+' = '+value(x) for k,x in v.items())+' }'
    return str(v)

def main():
    destination=ROOT/'native-launcher/combined/wyr'
    destination.mkdir(parents=True,exist_ok=True)
    legal=destination/'data/licenses';legal.mkdir(parents=True,exist_ok=True)
    for source,name in [(ROOT/'horizon/LICENSE','Horizon-MIT.txt'),(ROOT/'horizon/THIRD_PARTY_NOTICES.md','Horizon-NOTICES.md'),(ROOT/'orbital-laser/LICENSE','Orbital-MIT.txt'),(ROOT/'orbital-laser/NOTICE','Orbital-NOTICE.txt')]:
        shutil.copyfile(source,legal/name)
    sections={k:[] for k in ('module','injection','bridge','state','crate','post_build')}
    extra_content={}
    def merge(data):
        for key in sections:sections[key].extend(data.get(key,[]))
    for prefix,folder in [('upgrader',ROOT.parent/'UpgraderHorizon/mixins/upgrader_hardcore'),('wyr',ROOT/'port/mixins/wyr')]:
        manifest=tomli.loads((folder/'horizon-mixin.toml').read_text(encoding='utf8'))
        for key in sections:
            for item in manifest.get(key,[]):
                item['id']=prefix+'_'+item['id']
                sourcekey='source' if key=='module' else 'helper' if key=='injection' else None
                if sourcekey:
                    old=item[sourcekey];name=prefix+'_'+old
                    source=(folder/old).read_text(encoding='utf8')
                    if prefix=='upgrader' and old=='runtime.rs':
                        source=source.replace('game.native_on(ID)','game.native_on("wyr")').replace('"upgrader:upgrader"','"wyr:upgrader"')
                        # The earlier standalone package used a different item
                        # registry ID. Give an existing save its unified device
                        # once, while retaining its payout and attempt records.
                        source=source.replace('    granted: bool,','    granted: bool,\n    combined_native: bool,')
                        source=source.replace('"granted":p.granted,','"granted":p.granted,"combined_native":p.combined_native,')
                        source=source.replace('Ok(Player{granted:p["granted"].as_bool().unwrap_or(false),','Ok(Player{combined_native:p["combined_native"].as_bool().unwrap_or(false),granted:p["granted"].as_bool().unwrap_or(false),')
                        source=source.replace('        if !player.granted {','        if player.granted && owns(game) {player.combined_native=true;}\n        if !player.granted || !player.combined_native {')
                        source=source.replace('player.granted=true;','player.granted=true;player.combined_native=true;')
                        source=re.sub(r'pub fn mods\(mods:&mut Vec<NativeMod>\) \{.*?\n\}', 'pub fn mods(mods:&mut Vec<NativeMod>) {let _=mods;}',source,flags=re.S)
                    (destination/name).write_text(source,encoding='utf8');item[sourcekey]=name
        merge(manifest)
    import native_orbital
    merge(tomli.loads(native_orbital.assemble(destination)['manifest']))
    if (ROOT/'fork/native_patches.py').exists():
        import native_patches
        result=native_patches.assemble(destination)
        merge(tomli.loads(result['manifest']))
        extra_content.update(result.get('content',{}))
    # The content is read from this same native archive, as in Orbital Laser.
    helper=(ROOT/'orbital-laser/native/mixins/orbital/content_find.rs').read_text(encoding='utf8').replace('orbital','wyr')
    (destination/'combined_content_find.rs').write_text(helper,encoding='utf8')
    original=tomli.loads((ROOT/'orbital-laser/native/mixins/orbital/horizon-mixin.toml').read_text(encoding='utf8'))
    find=next(i for i in original['injection'] if i['id']=='content_find').copy();find.update(id='combined_content_find',helper='combined_content_find.rs')
    sections['injection'].append(find)
    text=f'[mod]\nid = "wyr"\nversion = "{VERSION}"\npriority = 100\n\n[compatibility]\nhorizon_revision = "{REV}"\n'
    ids=[]
    for key,items in sections.items():
        for item in items:
            if 'id' in item:ids.append(item['id'])
            text+='\n[['+key+']]\n'+''.join(k+' = '+value(v)+'\n' for k,v in item.items())
    if len(ids)!=len(set(ids)):raise RuntimeError('Duplicate native manifest IDs')
    (destination/'horizon-mixin.toml').write_text(text,encoding='utf8')
    out=ROOT/'dist/combined/wyr.hnmod'
    with zipfile.ZipFile(out,'w',zipfile.ZIP_DEFLATED,compresslevel=6) as z:
        z.writestr('horizon-native.json',json.dumps(dict(schema=1,id='wyr',version=VERSION,game='26.3',horizon=REV,mixins=MIXINS),indent=2))
        for p in sorted(destination.rglob('*')):
            if p.is_file() and (p.suffix in ('.rs','.toml') or 'data' in p.relative_to(destination).parts):
                z.write(p,'mixins/wyr/'+p.relative_to(destination).as_posix())
        with zipfile.ZipFile(ROOT/'dist/combined/wyr-full-content-0.2.0.hmod') as content:
            for name in content.namelist():
                if name.startswith(('assets/','data/')) or name=='horizon-mod.toml':
                    data=content.read(name)
                    if name=='horizon-mod.toml':data=data.replace(b'version = "0.2.0"',f'version = "{VERSION}"'.encode())
                    z.writestr(name,data)
        for name,data in extra_content.items():z.writestr(name,data)
    print(json.dumps({'file':str(out),'sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'modules':len(sections['module']),'injections':len(sections['injection'])},indent=2))

if __name__=='__main__':main()
