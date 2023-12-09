from operator import ne


def lcm(n1, n2):
    if n1 > n2:
        x = n1
        y = n2
    else:
        x = n2
        y = n1

    rem = x % y

    while rem != 0:
        x = y
        y = rem
        rem = x % y

    lcm = n1 * n2 / y

    return lcm


def main():
    numbers = (
        (0, 12740, 12737),
        (0, 19786, 19783),
        (0, 14365, 14363),
        (0, 19243, 19241),
        (0, 16533, 16531),
        (0, 18159, 18157),
    )

    pos = numbers[0][2]
    step = numbers[0][1]
    for i in range(1, len(numbers)):
        print(step)
        new = numbers[i]
        success = False
        for t in range(new[1]):
            tmp = (pos + t * step) % new[1]
            # print("trying", tmp)

            if tmp == new[2]:
                success = True
        if not success:
            raise RuntimeError("")

        step = lcm(step, new[1])

    print(pos)


main()
