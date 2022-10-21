# AVATAR area visualizer

It's a visualizer for [DIKU](https://dikumud.com/) based area files, but only tested on the dialect used by [AVATAR](https://www.outland.org).

![Screenshot of the program's graphical window, in Windows, showing four groups of squares arranged in a grid and connected to each other by lines. In the corner there's a legend showing which colors correspond to which terrain types.](/avatar-mapper.png)

## Usage

either invoke via command line like `areview <areafile>.are`, or drag an `.are` file onto the program icon.

Areview works best on areas that are logically separated in "floors", because by default it splits rooms into groups where they make up/down connections. Rooms are also separated from their group when they are only connected by one-way exits. The resulting groups may still be slightly distorted if rooms don't lay on an evenly spaced 2-D grid. You can force Areview to split rooms into groups with some command line arguments.

```shell
# This will consider room with vnum VVVV to be
# separate from all other rooms
areview AREAFILE.ARE isolate=VVVV
```

```shell
# This will split rooms into two groups at the
# connection between rooms AAAA and BBBB
areview AREAFILE.ARE separate=AAAA,BBBB
```

For example, `forge.are` looks especially gnarly unless you invoke the program with `separate=11490,11489`.

Besides those command line options, you can also massage room positions by clicking and dragging. You can group select rooms to move them all at once.

* Double-click a room to seelect all rooms on the same "floor".
* Ctrl-click to select multiple rooms at once.
* Right-click to undo a drag in progress.

## Releasing

Draft ye a release and click publish. The release assets will come in time.
