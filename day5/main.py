example = """47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"""

with open("./puzzle_input.txt", "r") as file:
    puzzle_input = file.read().strip()


def parse_input(in_str: str):
    rules: dict[int, list[int]] = {}
    lines = in_str.splitlines()
    
    while lines[0] != "":
        x, y = lines.pop(0).split("|")
        x = int(x)
        y = int(y)

        if x in rules.keys():
            rules[x].append(y)
        else:
            rules[x] = [y]
    
    lines.pop(0)
    updates = [[int(x) for x in u.split(",")] for u in lines] 
    
    return rules, updates

rules, updates = parse_input(puzzle_input)

def get_rules_for(n: int):
    if n not in rules.keys():
        return None
    
    return rules[n]

def update_is_valid(upd: list[int]) -> bool:
    for entry in upd:
        entry_rules = get_rules_for(entry) 
        if entry_rules is None:
            continue
        
        for rule in entry_rules:
            if rule in upd:
                rule_index = upd.index(rule)
                entry_index = upd.index(entry)
                
                if rule_index < entry_index:
                    return False
    
    return True

def make_update_valid(upd: list[int]) -> bool:
    from functools import cmp_to_key

    def strict_rule_for(x: int, y: int):
        if get_rules_for(x) is None:
            return False
        
        if y in get_rules_for(x):
            return True

        return False

    def multi_compare(a: int, b: int):
        if strict_rule_for(a, b):
            return -1
        elif strict_rule_for(b, a):
            return 1
        else:
            return 0
            
    return sorted(upd, key=cmp_to_key(multi_compare))
        
def get_middle_value_from(update: list[int]):
    return update[(len(update)//2)]

valid_updates: list[list[int]] = [upd for upd in updates if update_is_valid(upd)]
final_count = sum(get_middle_value_from(upd) for upd in valid_updates)
print("Middle sum of updates:", final_count)

invalid_updates: list[list[int]] = [upd for upd in updates if not update_is_valid(upd)]
corrected_updates = [make_update_valid(upd) for upd in invalid_updates]
corrected_count = sum(get_middle_value_from(upd) for upd in corrected_updates)
print("(Corrected) Middle sum of updates:", corrected_count)

 