#!/usr/bin/env python3
"""Génère l'atlas de sprites vectoriels du forgeron (et du lit) : blacksmith.svg.

Le personnage est un petit "rig" : des pièces (tête, barbe, torse, membres, marteau) dessinées une
fois, puis posées différemment à chaque image avec des angles et des positions. Changer le style
(couleurs, proportions) se fait ici, puis on relance `build.sh` pour régénérer le SVG et le PNG.

Toutes les coordonnées sont en pixels logiques d'une cellule de 64 x 80. Le point d'ancrage d'un
personnage debout est entre ses pieds, en (32, 58). Le lit et le dormeur sont ancrés au centre de
la tuile, en (32, 40). Le rendu final est fait à 2x pour rester net quand on zoome.
"""
import math
import sys

CELL_W, CELL_H = 64, 80
COLS, ROWS = 6, 6
FEET = (32, 58)
BED_CENTER = (32, 40)

# ---------------------------------------------------------------- palette
OUT = "#2b1a12"
SKIN, SKIN_SH = "#e6a47e", "#c4805b"
HAIR, HAIR_SH, HAIR_HI = "#e9751f", "#b3480e", "#f9a444"
VEST, VEST_SH = "#565850", "#3e403a"
APRON, APRON_SH, APRON_HI = "#87582f", "#6a4220", "#a06d3c"
BELT, BUCKLE = "#2e2723", "#cfd3d9"
PANTS, PANTS_SH = "#363c4d", "#272c3a"
BOOT, BOOT_SH = "#6f4729", "#51311b"
WRAP = "#7c4f2a"
WOOD, WOOD_DK = "#8d5b2e", "#5d3a1c"
METAL, METAL_DK, METAL_HI = "#8d929e", "#545966", "#c3c8d2"
BLANKET, BLANKET_SH, BLANKET_HI = "#b9473f", "#8f332d", "#d46a5f"
SHEET, PILLOW = "#e9e5d4", "#f4f1e6"


def f(v):
    return f"{v:.2f}".rstrip("0").rstrip(".")


def pts(points):
    return " ".join(f"{f(x)},{f(y)}" for x, y in points)


# ---------------------------------------------------------------- primitives
def limb(points, w, fill, outline=1.4):
    """Un membre : une polyligne épaisse aux bouts ronds, avec un contour sombre."""
    d = pts(points)
    base = f'fill="none" stroke-linecap="round" stroke-linejoin="round"'
    return (
        f'<polyline points="{d}" {base} stroke="{OUT}" stroke-width="{f(w + 2 * outline)}"/>'
        f'<polyline points="{d}" {base} stroke="{fill}" stroke-width="{f(w)}"/>'
    )


def shape(d, fill, outline=1.3, extra=""):
    return f'<path d="{d}" fill="{fill}" stroke="{OUT}" stroke-width="{f(outline)}" stroke-linejoin="round" {extra}/>'


def ell(cx, cy, rx, ry, fill, outline=1.2):
    stroke = f'stroke="{OUT}" stroke-width="{f(outline)}"' if outline else ""
    return f'<ellipse cx="{f(cx)}" cy="{f(cy)}" rx="{f(rx)}" ry="{f(ry)}" fill="{fill}" {stroke}/>'


def rect(x, y, w, h, fill, outline=1.2, r=0.0):
    stroke = f'stroke="{OUT}" stroke-width="{f(outline)}" stroke-linejoin="round"' if outline else ""
    return f'<rect x="{f(x)}" y="{f(y)}" width="{f(w)}" height="{f(h)}" rx="{f(r)}" fill="{fill}" {stroke}/>'


def line(a, b, color, w, cap="round"):
    return (
        f'<line x1="{f(a[0])}" y1="{f(a[1])}" x2="{f(b[0])}" y2="{f(b[1])}" '
        f'stroke="{color}" stroke-width="{f(w)}" stroke-linecap="{cap}"/>'
    )


def ik(a, b, l1, l2, bend):
    """Genou ou coude d'un membre à deux segments qui va de `a` à `b`. `bend` = +1 ou -1 : le côté."""
    dx, dy = b[0] - a[0], b[1] - a[1]
    d = max(math.hypot(dx, dy), 1e-6)
    d = min(d, l1 + l2 - 0.01)
    ang = math.atan2(dy, dx)
    cos_a = max(-1.0, min(1.0, (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)))
    a1 = ang + bend * math.acos(cos_a)
    return (a[0] + l1 * math.cos(a1), a[1] + l1 * math.sin(a1))


def arm_points(shoulder, hand, bend, l1=6.7, l2=6.7):
    return [shoulder, ik(shoulder, hand, l1, l2, bend), hand]


def leg_points(hip, foot, bend, l1=6.5, l2=6.5):
    return [hip, ik(hip, foot, l1, l2, bend), foot]


def shift(points, dy):
    return [(x, y + dy) for x, y in points]


# ---------------------------------------------------------------- pièces communes
def boot(x, y, facing="front", flip=False):
    """Botte dont le dessous est en y + 4. `facing` : front (de face), side (de profil), back."""
    if facing == "side":
        d = f"M{f(x-3.5)} {f(y-3)} L{f(x+2.5)} {f(y-3)} L{f(x+2.7)} {f(y+1)} Q{f(x+2.7)} {f(y+4)} {f(x-0.5)} {f(y+4)} L{f(x-6.5)} {f(y+4)} Q{f(x-7.6)} {f(y+4)} {f(x-7.4)} {f(y+2.2)} Q{f(x-6)} {f(y)} {f(x-3.5)} {f(y)} Z"
        return shape(d, BOOT) + line((x - 6.5, y + 3), (x - 0.5, y + 3), BOOT_SH, 1.0)
    d = f"M{f(x-4)} {f(y-3)} L{f(x+4)} {f(y-3)} L{f(x+4.6)} {f(y+2)} Q{f(x+4.6)} {f(y+4)} {f(x+2.5)} {f(y+4)} L{f(x-2.5)} {f(y+4)} Q{f(x-4.6)} {f(y+4)} {f(x-4.6)} {f(y+2)} Z"
    return shape(d, BOOT) + line((x - 3.5, y + 2.8), (x + 3.5, y + 2.8), BOOT_SH, 1.0)


def hand(x, y, r=3.1):
    return ell(x, y, r, r, SKIN, 1.2)


def wrist(a, b):
    """Bandeau de cuir au poignet : un court segment épais près de la main."""
    t0, t1 = 0.62, 0.88
    p = (a[0] + (b[0] - a[0]) * t0, a[1] + (b[1] - a[1]) * t0)
    q = (a[0] + (b[0] - a[0]) * t1, a[1] + (b[1] - a[1]) * t1)
    return limb([p, q], 5.6, WRAP, 1.1)


def arm(points, back=False):
    """Un bras entier : peau, bandeau au poignet, main."""
    s = limb(points, 5.6, SKIN, 1.3)
    s += wrist(points[1], points[2])
    s += hand(*points[2])
    return s


def hammer(hand_pos, tip, handle_len=None):
    """Marteau tenu en `hand_pos`, dont la tête est dans la direction de `tip`."""
    dx, dy = tip[0] - hand_pos[0], tip[1] - hand_pos[1]
    ang = math.atan2(dy, dx)
    length = handle_len or math.hypot(dx, dy)
    ex, ey = hand_pos[0] + length * math.cos(ang), hand_pos[1] + length * math.sin(ang)
    deg = math.degrees(ang)
    g = limb([(hand_pos[0] - 1.5 * math.cos(ang), hand_pos[1] - 1.5 * math.sin(ang)), (ex, ey)], 2.6, WOOD, 1.0)
    head = (
        f'<g transform="translate({f(ex)} {f(ey)}) rotate({f(deg)})">'
        + rect(-1.5, -4.2, 6.2, 8.4, METAL, 1.1, 0.8)
        + rect(-1.5, -4.2, 6.2, 2.6, METAL_HI, 0, 0.8)
        + "</g>"
    )
    return g + head


# ---------------------------------------------------------------- tête
def head_front(cx, cy, look_down=0.0):
    """Tête vue de face : cheveux, visage, grosse barbe rousse. (cx, cy) = centre du visage."""
    s = ""
    # cheveux derrière les oreilles
    s += shape(
        f"M{f(cx-9.4)} {f(cy+1)} C{f(cx-11)} {f(cy-7)} {f(cx-6)} {f(cy-11.5)} {f(cx)} {f(cy-11.5)} "
        f"C{f(cx+6)} {f(cy-11.5)} {f(cx+11)} {f(cy-7)} {f(cx+9.4)} {f(cy+1)} Z",
        HAIR,
    )
    # oreilles
    s += ell(cx - 8.4, cy + 0.5, 1.6, 2.2, SKIN, 1.0) + ell(cx + 8.4, cy + 0.5, 1.6, 2.2, SKIN, 1.0)
    # visage
    s += ell(cx, cy, 8.0, 7.6, SKIN, 1.3)
    # grosse barbe : couvre les joues et descend en pointe sur la poitrine
    beard = (
        f"M{f(cx-8.3)} {f(cy-1)} C{f(cx-10)} {f(cy+9)} {f(cx-6)} {f(cy+15)} {f(cx)} {f(cy+17.5)} "
        f"C{f(cx+6)} {f(cy+15)} {f(cx+10)} {f(cy+9)} {f(cx+8.3)} {f(cy-1)} "
        f"C{f(cx+6.6)} {f(cy+3.2)} {f(cx+4)} {f(cy+4.6)} {f(cx)} {f(cy+4.6)} "
        f"C{f(cx-4)} {f(cy+4.6)} {f(cx-6.6)} {f(cy+3.2)} {f(cx-8.3)} {f(cy-1)} Z"
    )
    s += shape(beard, HAIR, 1.3)
    s += line((cx, cy + 7), (cx, cy + 14.5), HAIR_SH, 1.0)
    s += line((cx - 4, cy + 6), (cx - 4.5, cy + 12.5), HAIR_SH, 0.9)
    s += line((cx + 4, cy + 6), (cx + 4.5, cy + 12.5), HAIR_SH, 0.9)
    s += line((cx - 3, cy + 7), (cx - 2.2, cy + 10), HAIR_HI, 0.8)
    # moustache
    s += shape(
        f"M{f(cx)} {f(cy+3.4)} C{f(cx-3)} {f(cy+1.2)} {f(cx-6.5)} {f(cy+2.4)} {f(cx-6.8)} {f(cy+5)} "
        f"C{f(cx-4)} {f(cy+5.4)} {f(cx-1.5)} {f(cy+5.6)} {f(cx)} {f(cy+4.6)} "
        f"C{f(cx+1.5)} {f(cy+5.6)} {f(cx+4)} {f(cy+5.4)} {f(cx+6.8)} {f(cy+5)} "
        f"C{f(cx+6.5)} {f(cy+2.4)} {f(cx+3)} {f(cy+1.2)} {f(cx)} {f(cy+3.4)} Z",
        HAIR, 1.0,
    )
    # nez, yeux, sourcils
    ey = cy - 1.0 + look_down
    s += ell(cx, cy + 2.0, 2.0, 1.7, SKIN_SH, 0)
    s += ell(cx - 3.6, ey, 1.15, 1.35, OUT, 0) + ell(cx + 3.6, ey, 1.15, 1.35, OUT, 0)
    s += ell(cx - 3.3, ey - 0.5, 0.4, 0.4, "#ffffff", 0) + ell(cx + 3.9, ey - 0.5, 0.4, 0.4, "#ffffff", 0)
    s += line((cx - 6, ey - 2.4), (cx - 1.6, ey - 1.6), HAIR_SH, 1.5) + line((cx + 6, ey - 2.4), (cx + 1.6, ey - 1.6), HAIR_SH, 1.5)
    # mèche de cheveux sur le front
    s += shape(
        f"M{f(cx-8)} {f(cy-3)} C{f(cx-6)} {f(cy-9)} {f(cx-1)} {f(cy-8.4)} {f(cx)} {f(cy-5.6)} "
        f"C{f(cx+1)} {f(cy-8.4)} {f(cx+6)} {f(cy-9)} {f(cx+8)} {f(cy-3)} "
        f"C{f(cx+9.4)} {f(cy-10)} {f(cx+5)} {f(cy-12.6)} {f(cx)} {f(cy-12.6)} "
        f"C{f(cx-5)} {f(cy-12.6)} {f(cx-9.4)} {f(cy-10)} {f(cx-8)} {f(cy-3)} Z",
        HAIR, 1.2,
    )
    s += line((cx - 4, cy - 10), (cx - 1, cy - 11), HAIR_HI, 1.0)
    return s


def head_back(cx, cy):
    """Tête vue de dos : cheveux roux, la barbe dépasse sur les côtés."""
    s = ""
    s += shape(
        f"M{f(cx-8)} {f(cy+3)} C{f(cx-10)} {f(cy+9)} {f(cx-8)} {f(cy+12)} {f(cx-6)} {f(cy+13)} L{f(cx+6)} {f(cy+13)} "
        f"C{f(cx+8)} {f(cy+12)} {f(cx+10)} {f(cy+9)} {f(cx+8)} {f(cy+3)} Z",
        HAIR_SH, 1.2,
    )
    s += shape(
        f"M{f(cx-9.4)} {f(cy+4)} C{f(cx-11.4)} {f(cy-7)} {f(cx-6)} {f(cy-11.8)} {f(cx)} {f(cy-11.8)} "
        f"C{f(cx+6)} {f(cy-11.8)} {f(cx+11.4)} {f(cy-7)} {f(cx+9.4)} {f(cy+4)} "
        f"C{f(cx+8)} {f(cy+9)} {f(cx+4)} {f(cy+11)} {f(cx)} {f(cy+11)} "
        f"C{f(cx-4)} {f(cy+11)} {f(cx-8)} {f(cy+9)} {f(cx-9.4)} {f(cy+4)} Z",
        HAIR, 1.3,
    )
    s += line((cx - 4, cy - 6), (cx - 5, cy + 4), HAIR_SH, 1.0)
    s += line((cx + 1, cy - 8), (cx + 1, cy + 6), HAIR_SH, 1.0)
    s += line((cx + 5, cy - 6), (cx + 6, cy + 3), HAIR_SH, 1.0)
    s += line((cx - 2.5, cy - 9), (cx + 3, cy - 9.4), HAIR_HI, 1.0)
    return s


def head_side(cx, cy, look_down=0.0):
    """Tête de profil, tournée vers la gauche."""
    s = ""
    # cheveux derrière
    s += shape(
        f"M{f(cx+8)} {f(cy+6)} C{f(cx+12)} {f(cy-4)} {f(cx+7)} {f(cy-11.5)} {f(cx-1)} {f(cy-11.5)} "
        f"C{f(cx-7)} {f(cy-11.5)} {f(cx-9.5)} {f(cy-7)} {f(cx-8.6)} {f(cy-2)} L{f(cx+2)} {f(cy+8)} Z",
        HAIR, 1.3,
    )
    # visage de profil, nez qui pointe vers la gauche
    s += shape(
        f"M{f(cx-8.4)} {f(cy-3)} C{f(cx-8.8)} {f(cy-8)} {f(cx-3)} {f(cy-8.6)} {f(cx+2)} {f(cy-8)} "
        f"C{f(cx+7.5)} {f(cy-6)} {f(cx+8)} {f(cy+1)} {f(cx+6)} {f(cy+5)} L{f(cx-6)} {f(cy+5)} "
        f"L{f(cx-8.8)} {f(cy+0.6)} L{f(cx-11)} {f(cy+1.6)} C{f(cx-11.5)} {f(cy+0.4)} {f(cx-10)} {f(cy-1.5)} {f(cx-8.4)} {f(cy-3)} Z",
        SKIN, 1.3,
    )
    # barbe, vers l'avant et vers le bas
    s += shape(
        f"M{f(cx-8)} {f(cy+1)} C{f(cx-12)} {f(cy+9)} {f(cx-7)} {f(cy+16)} {f(cx-2)} {f(cy+17.5)} "
        f"C{f(cx+4)} {f(cy+15)} {f(cx+8.5)} {f(cy+9)} {f(cx+7.5)} {f(cy-0.5)} "
        f"C{f(cx+3)} {f(cy+4)} {f(cx-3)} {f(cy+3.4)} {f(cx-8)} {f(cy+1)} Z",
        HAIR, 1.3,
    )
    s += line((cx - 3, cy + 6), (cx - 3.5, cy + 14), HAIR_SH, 0.9)
    s += line((cx + 2, cy + 5), (cx + 2.5, cy + 12.5), HAIR_SH, 0.9)
    # moustache et œil
    s += shape(
        f"M{f(cx-9)} {f(cy+1.2)} C{f(cx-7)} {f(cy+0.4)} {f(cx-3)} {f(cy+1.6)} {f(cx-1.5)} {f(cy+3.4)} "
        f"C{f(cx-5)} {f(cy+4.6)} {f(cx-8.6)} {f(cy+4.2)} {f(cx-9.6)} {f(cy+2.6)} Z",
        HAIR, 1.0,
    )
    ey = cy - 1.8 + look_down
    s += ell(cx - 5.2, ey, 1.05, 1.3, OUT, 0) + ell(cx - 4.9, ey - 0.5, 0.35, 0.35, "#ffffff", 0)
    s += line((cx - 8.2, ey - 2.6), (cx - 3.4, ey - 1.8), HAIR_SH, 1.5)
    # oreille, et cheveux sur le dessus
    s += ell(cx + 1.8, cy - 0.5, 1.6, 2.2, SKIN, 1.0)
    s += shape(
        f"M{f(cx-8.4)} {f(cy-3)} C{f(cx-7)} {f(cy-10)} {f(cx-1)} {f(cy-9)} {f(cx+1)} {f(cy-5)} "
        f"C{f(cx+3)} {f(cy-8)} {f(cx+8)} {f(cy-8)} {f(cx+9)} {f(cy-2)} "
        f"C{f(cx+9)} {f(cy-11)} {f(cx+3)} {f(cy-12.8)} {f(cx-2)} {f(cy-12.4)} "
        f"C{f(cx-7.5)} {f(cy-12)} {f(cx-10.5)} {f(cy-8)} {f(cx-8.4)} {f(cy-3)} Z",
        HAIR, 1.2,
    )
    s += line((cx - 4, cy - 10.4), (cx, cy - 11.3), HAIR_HI, 1.0)
    return s


# ---------------------------------------------------------------- poses debout
def torso_front(dy, apron=True):
    s = shape(
        f"M20 {f(31+dy)} Q20 {f(27.5+dy)} 24 {f(27.5+dy)} L40 {f(27.5+dy)} Q44 {f(27.5+dy)} 44 {f(31+dy)} "
        f"L43 {f(41+dy)} L21 {f(41+dy)} Z",
        VEST,
    )
    s += line((22.5, 31 + dy), (22, 40 + dy), VEST_SH, 1.2) + line((41.5, 31 + dy), (42, 40 + dy), VEST_SH, 1.2)
    if apron:
        s += shape(
            f"M25 {f(30.5+dy)} L39 {f(30.5+dy)} L41 {f(47.5+dy)} Q32 {f(50+dy)} 23 {f(47.5+dy)} Z", APRON
        )
        s += line((26.5, 33 + dy), (25.5, 46 + dy), APRON_HI, 1.0)
        s += line((37.5, 33 + dy), (38.5, 46 + dy), APRON_SH, 1.2)
        # bretelles du tablier
        s += line((26.5, 30.5 + dy), (24.2, 27.8 + dy), BELT, 1.6) + line((37.5, 30.5 + dy), (39.8, 27.8 + dy), BELT, 1.6)
    s += rect(21, 38 + dy, 22, 3.6, BELT, 1.1, 0.6)
    s += rect(30, 38.2 + dy, 4.6, 3.2, BUCKLE, 0.9, 0.5)
    return s


def torso_back(dy):
    s = shape(
        f"M20 {f(31+dy)} Q20 {f(27.5+dy)} 24 {f(27.5+dy)} L40 {f(27.5+dy)} Q44 {f(27.5+dy)} 44 {f(31+dy)} "
        f"L43 {f(42+dy)} L21 {f(42+dy)} Z",
        VEST,
    )
    s += line((32, 29 + dy), (32, 40 + dy), VEST_SH, 1.2)
    # sangles du tablier croisées dans le dos
    s += line((25, 28 + dy), (39, 39 + dy), BELT, 1.6) + line((39, 28 + dy), (25, 39 + dy), BELT, 1.6)
    s += rect(21, 38.4 + dy, 22, 3.6, BELT, 1.1, 0.6)
    return s


def torso_side(dy):
    s = shape(
        f"M24 {f(30.5+dy)} Q24 {f(27.5+dy)} 27.5 {f(27.5+dy)} L36 {f(27.5+dy)} Q40.5 {f(27.5+dy)} 41 {f(31+dy)} "
        f"L40 {f(41+dy)} L25 {f(41+dy)} Z",
        VEST,
    )
    # le tablier, vu de profil : une bande sur le devant
    s += shape(f"M23.2 {f(30+dy)} L27.8 {f(30+dy)} L27.8 {f(47+dy)} Q25.4 {f(48.4+dy)} 22.6 {f(47+dy)} Z", APRON)
    s += line((24.8, 33 + dy), (24.8, 45 + dy), APRON_HI, 0.9)
    s += rect(24, 38 + dy, 17, 3.4, BELT, 1.1, 0.6)
    return s


def pose_front_or_back(back, bob, lfoot, rfoot, larm_hand, rarm_hand, look_down=0.0, hammer_belt=False):
    """Personnage debout, de face ou de dos. Les pieds et les mains sont donnés en coordonnées absolues."""
    s = ""
    hl, hr = (27.5, 41.0 + bob), (36.5, 41.0 + bob)
    legs = [(leg_points(hl, lfoot, -1), lfoot), (leg_points(hr, rfoot, 1), rfoot)]
    for pts_, foot in legs:
        s += limb(pts_, 7.6, PANTS, 1.4)
        s += boot(foot[0], foot[1] + 1.6)
    s += torso_back(bob) if back else torso_front(bob)
    if back and hammer_belt:
        # le marteau pendu à la ceinture, côté droit
        s += limb([(40.5, 40 + bob), (41.5, 49 + bob)], 2.2, WOOD, 0.9)
        s += rect(38.8, 38.4 + bob, 5.4, 3.6, METAL, 0.9, 0.6)
    s += (head_back if back else head_front)(32, 19.6 + bob) if back else head_front(32, 19.6 + bob, look_down)
    # bras par-dessus le torse
    sl, sr = (21.0, 31.5 + bob), (43.0, 31.5 + bob)
    s += arm(arm_points(sl, larm_hand, -1))
    s += arm(arm_points(sr, rarm_hand, 1))
    return s


def idle_front_or_back(back):
    return pose_front_or_back(back, 0, (27.5, 53.2), (36.5, 53.2), (19.6, 43.4), (44.4, 43.4), hammer_belt=True)


def walk_front_or_back(back, i, n=6):
    p = i / n
    s_ = math.sin(2 * math.pi * p)
    lift_l, lift_r = max(0.0, s_) * 3.6, max(0.0, -s_) * 3.6
    bob = -abs(s_) * 1.3
    lfoot = (27.5 - 0.5 * s_, 53.2 - lift_l)
    rfoot = (36.5 + 0.5 * s_, 53.2 - lift_r)
    larm = (19.7, 43.4 + 2.6 * s_)
    rarm = (44.3, 43.4 - 2.6 * s_)
    return pose_front_or_back(back, bob, lfoot, rfoot, larm, rarm, hammer_belt=True)


def pose_side(bob, near_foot, far_foot, near_hand, far_hand):
    """Personnage de profil, tourné vers la gauche. Le côté proche est le bras et la jambe de devant."""
    s = ""
    hip = (32.0, 41.0 + bob)
    # jambe lointaine (plus sombre), derrière
    far = leg_points(hip, far_foot, -1)
    s += limb(far, 7.0, PANTS_SH, 1.3)
    s += boot(far_foot[0] - 0.5, far_foot[1] + 1.6, "side")
    # bras lointain, derrière le torse
    shoulder = (32.0, 31.5 + bob)
    s += arm(arm_points(shoulder, far_hand, 1)).replace(SKIN, "#c98a66")
    s += torso_side(bob)
    near = leg_points(hip, near_foot, -1)
    s += limb(near, 7.4, PANTS, 1.4)
    s += boot(near_foot[0] - 0.5, near_foot[1] + 1.6, "side")
    s += head_side(31.2, 19.6 + bob)
    s += arm(arm_points(shoulder, near_hand, 1))
    return s


def idle_side():
    return pose_side(0, (32.0, 53.2), (32.0, 53.2), (31.0, 43.0), (33.0, 43.0))


def walk_side(i, n=6):
    p = i / n
    a = 2 * math.pi * p
    s_, c_ = math.sin(a), math.cos(a)
    stride = 6.2
    near = (32.0 + stride * s_, 53.2 - max(0.0, c_) * 3.2)
    far = (32.0 - stride * s_, 53.2 - max(0.0, -c_) * 3.2)
    bob = -abs(c_) * 1.2
    hand_near = (32.0 - 5.2 * s_, 43.0 - 0.7 * abs(s_))
    hand_far = (32.0 + 5.2 * s_, 43.0 - 0.7 * abs(s_))
    return pose_side(bob, near, far, hand_near, hand_far)


# ---------------------------------------------------------------- forge
def forge_pose(step):
    """4 images : marteau levé, armé, coup sur l'enclume, retour. L'enclume (et le métal chauffé) sont
    dessinés par le jeu, juste sous le point (32, 71)."""
    # (bob, main du marteau, bout du marteau)
    frames = [
        (0.0, (46.5, 25.0), (50.5, 12.5)),
        (-0.6, (47.5, 20.0), (46.0, 7.5)),
        (1.4, (40.5, 49.0), (34.0, 68.5)),
        (0.4, (44.5, 36.0), (47.0, 24.0)),
    ]
    bob, rh, tip = frames[step]
    s = ""
    hl, hr = (27.5, 41.0 + bob), (36.5, 41.0 + bob)
    for hip, foot in ((hl, (26.8, 53.2)), (hr, (37.2, 53.2))):
        s += limb(leg_points(hip, foot, -1 if hip is hl else 1), 7.6, PANTS, 1.4)
        s += boot(foot[0], foot[1] + 1.6)
    s += torso_front(bob)
    s += head_front(32, 19.8 + bob, look_down=0.5 if step == 2 else 0.0)
    # bras gauche : tient la pièce avec des tenailles, vers l'enclume
    sl = (21.0, 31.5 + bob)
    lh = (24.6, 47.0 + bob)
    s += limb([(lh[0], lh[1] + 1), (27.4, 71.0)], 1.8, METAL_DK, 0.8) + limb([(lh[0] + 1.4, lh[1] + 1), (29.4, 71.0)], 1.8, METAL, 0.8)
    s += arm(arm_points(sl, lh, -1))
    # bras droit et marteau
    sr = (43.0, 31.5 + bob)
    s += hammer(rh, tip)
    s += arm(arm_points(sr, rh, 1))
    return s


# ---------------------------------------------------------------- lit et dormeur
def bed():
    cx, cy = BED_CENTER
    x0, y0, w, h = cx - 15, cy - 13, 30, 24
    s = ""
    # pieds et côté avant du lit (donne l'effet de vue inclinée)
    s += rect(x0 + 0.5, y0 + h - 4, w - 1, 7.5, WOOD_DK, 1.3, 1.2)
    # cadre et matelas
    s += rect(x0, y0, w, h, WOOD, 1.4, 1.8)
    s += rect(x0 + 2, y0 + 2, w - 4, h - 4, SHEET, 1.0, 1.0)
    # oreiller, à gauche
    s += rect(x0 + 3, y0 + 4, 9, h - 8, PILLOW, 1.0, 2.0)
    s += line((x0 + 5, y0 + 7), (x0 + 5, y0 + h - 7), SHEET, 0.9)
    # couverture, sur la droite
    s += shape(
        f"M{f(x0+13.5)} {f(y0+2.8)} L{f(x0+w-2.8)} {f(y0+2.8)} L{f(x0+w-2.8)} {f(y0+h-2.8)} L{f(x0+13.5)} {f(y0+h-2.8)} Z",
        BLANKET, 1.1,
    )
    s += line((x0 + 15, y0 + 5), (x0 + 15, y0 + h - 5), BLANKET_HI, 1.0)
    s += line((x0 + 21, y0 + 5), (x0 + 21, y0 + h - 5), BLANKET_SH, 0.9)
    return s


def sleeper(step):
    """Forgeron couché, la tête sur l'oreiller (à gauche), sous la couverture. 2 images : il respire."""
    cx, cy = BED_CENTER
    rise = 0.0 if step == 0 else 1.1
    s = ""
    # épaule et bras, sous la couverture
    s += ell(cx - 5.0, cy - 0.5, 4.8, 5.2, SKIN, 1.2)
    # couverture qui monte jusqu'aux épaules, avec la bosse du corps
    s += shape(
        f"M{f(cx-5.6)} {f(cy-8.2-rise)} C{f(cx+2)} {f(cy-9.4-rise)} {f(cx+8)} {f(cy-8.6-rise*0.6)} {f(cx+12.2)} {f(cy-8)} "
        f"L{f(cx+12.2)} {f(cy+9.2)} L{f(cx-5.6)} {f(cy+9.2)} C{f(cx-7.4)} {f(cy+4)} {f(cx-7.4)} {f(cy-3)} {f(cx-5.6)} {f(cy-8.2-rise)} Z",
        BLANKET, 1.2,
    )
    s += line((cx - 1.2, cy - 6.5 - rise), (cx - 1.2, cy + 7.6), BLANKET_HI, 1.0)
    s += line((cx + 5.2, cy - 6.8 - rise * 0.6), (cx + 5.2, cy + 7.6), BLANKET_SH, 0.9)
    # tête, tournée vers le haut (vue de face, couchée sur le côté), cheveux et barbe étalés
    hx, hy = cx - 9.4, cy - 1.6
    s += shape(
        f"M{f(hx-7.5)} {f(hy+1)} C{f(hx-9)} {f(hy-8)} {f(hx-2)} {f(hy-9.6)} {f(hx+3)} {f(hy-8)} "
        f"C{f(hx+8)} {f(hy-5)} {f(hx+8)} {f(hy+4)} {f(hx+3)} {f(hy+8)} L{f(hx-5)} {f(hy+7)} Z",
        HAIR, 1.3,
    )
    s += ell(hx, hy, 6.6, 6.4, SKIN, 1.3)
    # barbe : s'étale sur la couverture
    s += shape(
        f"M{f(hx-5.6)} {f(hy+1.6)} C{f(hx-6.4)} {f(hy+7)} {f(hx-1)} {f(hy+9.6)} {f(hx+5)} {f(hy+9.6)} "
        f"C{f(hx+11)} {f(hy+9.4)} {f(hx+15)} {f(hy+6)} {f(hx+14.4)} {f(hy+2.4)} "
        f"C{f(hx+10)} {f(hy+3.4)} {f(hx+4)} {f(hy+3.4)} {f(hx+2)} {f(hy+2.6)} "
        f"C{f(hx-1.5)} {f(hy+3.8)} {f(hx-4)} {f(hy+3.2)} {f(hx-5.6)} {f(hy+1.6)} Z",
        HAIR, 1.2,
    )
    s += line((hx + 4, hy + 5), (hx + 11, hy + 5), HAIR_SH, 0.9)
    s += line((hx + 2, hy + 7.4), (hx + 9, hy + 7.6), HAIR_SH, 0.9)
    # yeux fermés, nez
    s += f'<path d="M{f(hx-3.4)} {f(hy-1.4)} q1.4 1.2 2.8 0" fill="none" stroke="{OUT}" stroke-width="1.1" stroke-linecap="round"/>'
    s += f'<path d="M{f(hx+0.8)} {f(hy-1.4)} q1.4 1.2 2.8 0" fill="none" stroke="{OUT}" stroke-width="1.1" stroke-linecap="round"/>'
    s += ell(hx + 0.6, hy + 1.6, 1.5, 1.2, SKIN_SH, 0)
    s += line((hx - 5.4, hy - 3.0), (hx - 1.6, hy - 2.4), HAIR_SH, 1.3) + line((hx + 0.2, hy - 2.4), (hx + 4.2, hy - 3.0), HAIR_SH, 1.3)
    return s


# ---------------------------------------------------------------- atlas
def cell(col, row, inner):
    return f'<g transform="translate({col * CELL_W} {row * CELL_H})">{inner}</g>'


def build():
    cells = []
    for i in range(6):
        cells.append(cell(i, 0, walk_front_or_back(False, i)))
        cells.append(cell(i, 1, walk_front_or_back(True, i)))
        cells.append(cell(i, 2, walk_side(i)))
    cells.append(cell(0, 3, idle_front_or_back(False)))
    cells.append(cell(1, 3, idle_front_or_back(True)))
    cells.append(cell(2, 3, idle_side()))
    for i in range(4):
        cells.append(cell(i, 4, forge_pose(i)))
    cells.append(cell(0, 5, bed()))
    cells.append(cell(1, 5, sleeper(0)))
    cells.append(cell(2, 5, sleeper(1)))
    w, h = COLS * CELL_W, ROWS * CELL_H
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">\n'
        + "\n".join(cells)
        + "\n</svg>\n"
    )


if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "blacksmith.svg"
    with open(out, "w", encoding="utf-8") as fh:
        fh.write(build())
    print(f"écrit {out}")
