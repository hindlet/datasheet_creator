import re
import os

### PLACE THIS FILE IN THE DIRECTORY THAT HAS THE FOLDERS OF YOUR UNITS AND RUN IT


DEPTH = 2



def get_all_files(searchDepth: int):
    current_dir = os.listdir(".")
    files = [f for f in current_dir if os.path.isfile(f) and os.path.splitext(f)[1] == ".ron"]
    dirs = [f for f in current_dir if os.path.isdir(f)]
    for _ in range(0, searchDepth):
        for _ in range(0, len(dirs)):
            dir = dirs.pop(0)
            current_dir = os.listdir(dir)
            paths = [dir + "/" + p for p in current_dir]
            files += [f for f in paths if os.path.isfile(f) and os.path.splitext(f)[1] == ".ron"]
            dirs += [f for f in paths if os.path.isdir(f)]
    files.remove('SETTINGS.ron')
    return files


def upgrade_unit(file: str):
    text = open(file).read();
    replace = re.sub(re.escape("RapidFire([1-9]+)"), "RapidFire(Set(1), \"\")", text)
    # print(replace)
    open(file, 'w').write(replace)
    print("Updated file: " + file)



files = get_all_files(DEPTH);
# print(files)
for file in files:
    upgrade_unit(file)
