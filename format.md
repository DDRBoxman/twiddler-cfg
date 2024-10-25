### Format Wiki (v6 not released yet)
https://www.mytwiddler.com/doc/doku.php?id=twiddler_config_format


### v5 Format
https://www.mytwiddler.com/doc/static/twiddler3_config_format_v5_rev3.pdf

### v7 Format

00-03: 0
0004: 7 // version
05-06: config flags // bitfield
    repeat delay enable
    bluetooth???
    haptic
    direct
    sticky_num
    sticky_alt
    sticky_ctrl
    sticky_shift
    left_mouse_pos // FOL or FOR
0008: num chords
0A-0B: idle time
0C: mouse sensitivity
0D: key repeat delay 
0080: Chords

### v6 Format reversing

00-03: 0
0004: 6 // version
0005: config flags // bitfield
    repeat delay enable
    haptic
    left_mouse_pos // FOL or FOR
    direct
    sticky_num
    sticky_alt
    sticky_ctrl
    sticky_shift
0006: number of chords
08-09: idle time (3C 00) 1 minute (10 0E) 10 minute
0010: mouse sensitivity 0x01 -> 254, 0xFE -> 1
0011: key repeat delay 0x01-0xFA (10ms - 2500ms)
0030: Chords
    8 bytes per chord

    00: keys
    01: keys
    02: keys


    03: 
    04:
    05:
    06:
    07:

_ 0M a
00 00 01 00 02 00 04 00

_ 0M A
00 00 01 00 02 20 04 00

_ 0M A
00 00 02 00 02 20 04 00

0 0M A
00 00 0A 00 02 20 04 00

0 0M a
00 00 0A 00 02 00 04 00

_ 0M b
02 00 05 00


Z 
20 1D

0 
00 27

9
00 27

1
1E


ALL a
FF FF 0F 00 02 00 04 00

_ OM left mouse
00 00 02 00 03 01 00 00

_ 0M middle mouse
00 00 02 00 03 04 00 00



More mouse 
00 00 02 00 07 00 00 00

@0x70
03 01 00 00 
05 05 00 00 
03 00 00 00 
00 00 00 00



command

01 System
02 keyboard
03 mouse
05 delay
07 list of commands

keyboard
hid keyfields
https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2


Mouse (bitfield)
01 left
02 right
04 middle

delay
duration in ms / 10
10k ms
E8 03 00

little endian

List of commands
07 00 00 00
07 00 10 00




0b00001010
0b00001100



t0
00 00 08
t1

r0
00 00 04
l0
00 00 01
m0
00 00 02

r1
02 00 00
m1
04 00 00
l1
08 00 00

2r
20 00 00
2m
40 00 00
2l
80 00 00

3r
00 02 00
3m
00 04 00
3l
00 08 00

4r
00 20 00
4m
00 40 00
4l
00 80 00

t10r
01 00 04
t20r
10 00 04
t30r
00 01 04
t40r
00 10 04
t00r
00 00 0C (08)