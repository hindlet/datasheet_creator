# Datasheet Creator


This is a small personal project to render custom datacards for warhammer. 


### UI

There is now a GUI written using [egui](https://crates.io/crates/egui). It is fairly basic but includes a card editor and some basic theme customisation.

[Download](https://github.com/hindlet/datasheet_creator/releases/download/v1.0/datasheet_creator.exe)


### App File Layout

The folders used with the app should be layed out as follows. The child folders are where the datacards are stored and can have any name wanted

```
Datasheets/
├─ infantry/
│  ├─ unit.ron
├─ vehicles/
│  ├─ unit.ron

├─ characters/
│  ├─ commander.ron
```


# TODO
[✓] - add keyword adds ability instead
[✓] - Melee weapons have WS instead of BS
[✓] - Stat labels show up in light mode
[✓] - weapon keywords read mode
[] - weapon keywords edit mode
