"""Render native ring geometry with the production Rust driver's angles, not a game screenshot."""
import csv
import math
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/"preview"
OUT.mkdir(exist_ok=True)
FONT=Path(r"C:\Windows\Fonts\segoeui.ttf")
FONTS={size:ImageFont.truetype(str(FONT),size) for size in [11,14,16,19,22]}
TAU=math.tau
ODDS=.22

def render(angle,won,moving):
    image=Image.new("RGB",(520,320),"#17232e");draw=ImageDraw.Draw(image)
    draw.text((16,10),"АПГРЕЙДЕР · ХАРДКОР",font=FONTS[19],fill="#ffc36b")
    title="КОЛЕСО КРУТИТСЯ…" if moving else ("ВЫИГРЫШ!" if won else "СТАВКА ПРОИГРАНА")
    draw.text((16,42),title,font=FONTS[16],fill="#ffdc99")
    center=(145,177);scale=1.15
    def ring(inner,outer,start,sweep,color):
        steps=max(1,math.ceil(abs(sweep)/TAU*160))
        def point(radius,a):return (center[0]+radius*scale*math.cos(a),center[1]+radius*scale*math.sin(a))
        for i in range(steps):
            a=start+sweep*i/steps;b=start+sweep*(i+1)/steps
            draw.polygon([point(inner,a),point(outer,a),point(outer,b),point(inner,b)],fill=color)
    ring(0,80,0,TAU,"#e5b866");ring(0,76.5,0,TAU,"#101820")
    for i in range(16):ring(47,73,-math.pi/2+angle+TAU*i/16,TAU/16,"#9f3948" if i%2==0 else "#6c293d")
    ring(47,73,-math.pi/2+angle,TAU*ODDS,"#36c397")
    ring(44.5,47,0,TAU,"#e5b866")
    draw.polygon([(center[0]-8,center[1]-99),(center[0]+8,center[1]-99),(center[0],center[1]-76)],fill="#ffd77e")
    text="22.00000%";bbox=draw.textbbox((0,0),text,font=FONTS[16]);width=bbox[2]-bbox[0]
    draw.text((center[0]-width/2,center[1]-11),text,font=FONTS[16],fill="white")
    for y,text,color,size in [(104,"Зелёный сектор — выигрыш","#65e6ba",16),
                              (132,"Красный сектор — проигрыш","#ff8395",16),
                              (179,"Ставка: 10 железных слитков","#b9cbd7",14),
                              (202,"Выигрыш: 1 алмаз","#ffffff",14),
                              (236,"Результат определяет сервер","#b9cbd7",14)]:
        draw.text((267,y),text,font=FONTS[size],fill=color)
    draw.text((16,294),"Предпросмотр геометрии и анимации; это не кадр из игры.",font=FONTS[11],fill="#99acbb")
    return image

rows=list(csv.reader((OUT/"angles.csv").read_text().splitlines()))
win=[row for row in rows if row[0]=="1"]
frames=[render(float(r[3]),True,r[4]=="1") for r in win]
frames[0].save(OUT/"roulette-win.gif",save_all=True,append_images=frames[1:],duration=80,loop=0,optimize=True)
left=render(float(win[-1][3]),True,False)
lose=[row for row in rows if row[0]=="0"]
right=render(float(lose[-1][3]),False,False)
comparison=Image.new("RGB",(1040,320));comparison.paste(left,(0,0));comparison.paste(right,(520,0))
comparison.save(OUT/"roulette-outcomes.png")
print(OUT/"roulette-win.gif")
print(OUT/"roulette-outcomes.png")
