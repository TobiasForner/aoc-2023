from sympy import *


def main():
    x0 = 248315803897794
    y0 = 386127890875011.0
    z0 = 326651351825022.0
    v_x0 = -89.0
    v_y0 = -119.0
    v_z0 = 32.0
    x1 = 332497633176671.0
    y1 = 319768494554521.0
    z1 = 308514709214883.0
    v_x1 = -120.0
    v_y1 = -49.0
    v_z1 = -91.0
    x2 = 362310144548603.0
    y2 = 372801373228571.0
    z2 = 154999941640943.0
    v_x2 = 80.0
    v_y2 = -599.0
    v_z2 = 249.0
    x, y, z, vx, vy, vz, t0, t1, t2 = symbols("x y z vx vy vz t0 t1 t2")
    res = solve(
        [
            x + t0 * (vx - v_x0) - x0,
            x + t1 * (vx - v_x1) - x1,
            x + t2 * (vx - v_x2) - x2,
            y + t0 * (vy - v_y0) - y0,
            y + t1 * (vy - v_y1) - y1,
            y + t2 * (vy - v_y2) - y2,
            z + t0 * (vz - v_z0) - z0,
            z + t1 * (vz - v_z1) - z1,
            z + t2 * (vz - v_z2) - z2,
        ],
        [x, y, z, vx, vy, vz],
        dict=True,
    )
    print(res)

    res = solve(
        [
            (x0 - x) * (vy - v_y0) - (y0 - y) * (vx - v_x0),
            (z0 - z) * (vy - v_y0) - (y0 - y) * (vz - v_z0),
            (x1 - x) * (vy - v_y1) - (y1 - y) * (vx - v_x1),
            (z1 - z) * (vy - v_y1) - (y1 - y) * (vz - v_z1),
            (x2 - x) * (vy - v_y2) - (y2 - y) * (vx - v_x2),
            (z2 - z) * (vy - v_y2) - (y2 - y) * (vz - v_z2),
        ],
        [x, y, z, vx, vy, vz],
        dict=True,
    )
    res = [r for r in res if all([v % 1 == 0 for v in r.values()])]
    print(res)
    res = res[0]
    print(res[x] + res[y] + res[z])


if __name__ == "__main__":
    main()
