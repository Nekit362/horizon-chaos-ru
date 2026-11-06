"""Create the personal Windows bundle, without Minecraft resources or saves."""
from pathlib import Path
import argparse,hashlib,json,zipfile

ROOT=Path(__file__).resolve().parent.parent

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--repository',type=Path,default=ROOT.parent/'HorizonChaos');parser.add_argument('--binary',type=Path,default=ROOT/'target-shared/fast/horizon-client.exe');args=parser.parse_args()
    files={
        'horizon-client.exe':args.binary,
        'wyr-full-content-0.2.0.hmod':ROOT/'dist/combined/wyr-full-content-0.2.0.hmod',
        'Start.cmd':args.repository/'Start.cmd',
        'Start.ps1':args.repository/'Start.ps1',
        'README.md':args.repository/'README.md',
        'VALIDATION.md':args.repository/'VALIDATION.md',
        'licenses/Horizon-MIT.txt':ROOT/'horizon/LICENSE',
        'licenses/Horizon-THIRD-PARTY-NOTICES.md':ROOT/'horizon/THIRD_PARTY_NOTICES.md',
        'licenses/Orbital-Laser-MIT.txt':ROOT/'orbital-laser/LICENSE',
        'licenses/Orbital-Laser-NOTICE.txt':ROOT/'orbital-laser/NOTICE',
    }
    hashes={name:hashlib.sha256(path.read_bytes()).hexdigest() for name,path in files.items()}
    out=ROOT/'dist/combined/horizon-chaos-ru-0.2.0-windows-x64.zip'
    with zipfile.ZipFile(out,'w',zipfile.ZIP_DEFLATED,compresslevel=6) as z:
        for name,path in sorted(files.items()):
            info=zipfile.ZipInfo(name,(2026,11,6,0,0,0));info.compress_type=zipfile.ZIP_DEFLATED;info.external_attr=0o100644<<16;z.writestr(info,path.read_bytes())
        z.writestr('SHA256SUMS.txt',''.join(f'{digest}  {name}\n' for name,digest in sorted(hashes.items())))
    with zipfile.ZipFile(out) as z:
        if z.testzip():raise RuntimeError('ZIP integrity check failed')
    print(json.dumps({'file':str(out),'bytes':out.stat().st_size,'sha256':hashlib.sha256(out.read_bytes()).hexdigest()},indent=2))

if __name__=='__main__':main()
