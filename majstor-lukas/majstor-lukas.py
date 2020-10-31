import sys

data = []
for line in sys.stdin:
    data.append(line.split()[0])

moves = int(data[0])
sven_moves = list(data[1])

score = 0
max_score = 0
enemies = int(data[2])

beats = {
    "P": "R",
    "S": "P",
    "R": "S"
}

def getScore(friend, foe):
    if friend == foe:
        return 1
    if(beats[friend] == foe):
        return 2
    return 0

enemy_move_list = []

for i in range(0, enemies):
    enemy_moves = list(data[3+i])
    enemy_move_list.append(enemy_moves)
    for j in range(0,moves):
        score = score + getScore(sven_moves[j], enemy_moves[j])

for move in range(0, moves):
    player_moves = ["R", "P", "S"]
    highest = 0
    for player_move in player_moves:
        in_score = 0
        for enemy in range(0, enemies):
            in_score = in_score + getScore(player_move, enemy_move_list[enemy][move])
        if (in_score > highest):
            highest = in_score
    max_score = max_score + highest            


print(score)
print(max_score)