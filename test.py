from meta_pytris import State

state = State("state_name", 0, 29, 0)
playfield = bytearray(256)

tile_map = {
        0xef: '.',
        0x7b: 'b',
        0x7c: 'c',
        0x7d: 'd',
        }

def print_playfield():
    for y in range(20):
        for x in range(10):
            yx = y * 10 + x
            print(tile_map[playfield[yx]], end='')
        print('')

while True:
    if state.dead:
        break
    state.step()
    if (updated := state.playfield()) != playfield:
        playfield[:] = updated
        print_playfield()
