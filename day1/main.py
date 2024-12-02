left_list: list[int] = []
right_list: list[int] = []

with open("./puzzle_input.txt", "r") as file_input:
    line = file_input.readline().strip()
    while line is None or line != "":
        num_input = line.split(" ")
        left_list.append(int(num_input[0]))
        right_list.append(int(num_input[-1]))
        line = file_input.readline().strip()


def list_is_empty(li: list):
    return len(li) <= 0


def iter_total_distance(l_list: list[int], r_list: list[int]):
    total_distance = 0

    if len(l_list) != len(r_list):
        raise ValueError("List lengths do not match")

    l_list = sorted(l_list)
    r_list = sorted(r_list)

    for _ in range(len(l_list)):
        left, right = (l_list.pop(0), r_list.pop(0))
        total_distance += abs(left - right)

    return total_distance


iter_ans = iter_total_distance(left_list.copy(), right_list.copy())
print(iter_ans)


def similarity_score(l_list: list[int], r_list: list[int]):
    scores: dict[int, int] = {}

    def score(n: int):
        if n in scores.keys():
            return scores[n]

        scores[n] = n * r_list.count(n)
        return scores[n]

    similarity_score = 0
    for num in l_list:
        similarity_score += score(num)

    return similarity_score


s_score = similarity_score(left_list.copy(), right_list.copy())
print(s_score)
