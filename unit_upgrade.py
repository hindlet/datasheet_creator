

files_to_upgrade = ["sunsets_fury.ron"]


for file in files_to_upgrade:
    lines = open(file).readlines()
    ranged = False
    melee = False
    core = False
    comp = False

    out = []
    for line in lines: ## read all lines

        ## start of sections
        if line.strip() == "ranged_weapons: [":
            ranged = True
            out.append(line)
            continue
        if line.strip() == "melee_weapons: [":
            melee = True
            out.append(line)
            continue
        if line.strip() == "core_abilities: [":
            core = True
            out.append("    core_abilities: [],\n")
            continue
        if line.strip() == "composition: [":
            comp = True
            out.append("    unit_comp: (\n")
            out.append("        comp: []\n")
            out.append("    ),\n")
            continue
        ## end of sections
        if (ranged or melee or core or comp) and line.strip() == "],":
            if ranged or melee:
                out.append(line)
            ranged, melee, core, comp = False, False, False, False
            continue


        ## ranged and melee repair
        if ranged or melee:
            split_one = line.split("(name: ")
            split_two = split_one[1].split("keywords: ")
            out.append(split_one[0] + "((name: " + split_two[0] + "keywords: []), 1),\n")
            continue

        ## core repair - just remove
        if core:
            continue 

        ## comp repair - just remove
        if comp:
            continue 

        out.append(line)

    f = open(file, "w")
    f.writelines(out)