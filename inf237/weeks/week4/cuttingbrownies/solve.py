# Concept proof for solution. To be coded in Rust...

opt = {}
opt[(1,1, "Harry")] = False
opt[(1,1, "Vicky")] = False

opt[(1,2, "Harry")] = True
opt[(1,2, "Vicky")] = False
opt[(2,1, "Harry")] = False
opt[(2,1, "Vicky")] = True
opt[(2,2, "Harry")] = False
opt[(2,2, "Vicky")] = False


"""for i in range(3,501):
    opt[(1, i, "Harry")] = True
    opt[(1, i, "Vicky")] = False
    opt[(i, 1, "Vicky")] = True
    opt[(i, 1, "Harry")] = False"""

for width in range(1,10):
    for height in range(1,10):
        if (width,height, "Harry") not in opt:
            can_win = False
            for c in range(1, width):
                # If vicky cannot hold cut one and harry can hold cut 2     OR      Harry can hold cut 1and vicky cannot hold cut 2
                harry_can_cut_1 = opt[(c, height, "Harry")]
                harry_can_cut_2 = opt[(width - c, height, "Harry")]

                vicky_can_cut_1 = opt[(c, height, "Vicky")]
                vicky_can_cut_2 = opt[(width - c, height, "Vicky")]

                if ((not vicky_can_cut_1) and harry_can_cut_2) or (harry_can_cut_1 and (not vicky_can_cut_2)):
                    can_win = True
            
            opt[(width, height, "Harry")] = can_win

        if (width,height, "Vicky") not in opt:
            can_win = False
            for c in range(1, height):
                # If Harry cannot hold cut 1 and Vicky can hold cut 2       OR      Vicky can hold cut 1 and Harry cannot hold cut 2
                harry_can_cut_1 = opt[(width, c, "Harry")]
                harry_can_cut_2 = opt[(width, height - c, "Harry")]

                vicky_can_cut_1 = opt[(width, c, "Vicky")]
                vicky_can_cut_2 = opt[(width, height - c, "Vicky")]

                if ((not harry_can_cut_1) and vicky_can_cut_2) or (vicky_can_cut_1 and (not harry_can_cut_2)):
                    can_win = True
                
            opt[(width, height, "Vicky")] = can_win

for (key, value) in opt.items():
    print(key, value)

print(opt[(4, 2, "Harry")])



"""
n = int(input())

for i in range(n):
    w, h, x = input().strip().split(' ')
    w = int(w)
    h = int(h)

    if (w,h) in opt:
        if opt[(w,h)] == x:
            print(f"{x} can win")
        else:
            print(f"{x} cannot win")
"""