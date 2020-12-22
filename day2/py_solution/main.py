from collections import namedtuple

class element:
    def __init__(self):
        self.range = namedtuple('range',['low','high'])
        self.password = None
        self.token = None

    def set(self,low,high,pswd,tok):
        self.range.low = low
        self.range.high = high
        self.password = pswd
        self.token = tok
    
    def __repr__(self):
        return f"range:({self.range.low},{self.range.high}),password:{self.password},token:{self.token}"

def parse_input():
    with open("input.txt","r+") as _f:
        #elem_list = []
        for line in _f:
            el = element()
            ranges,_,_ = line[:-1].partition(' ')
            char,_,pswd = line[len(ranges):-1].partition(':')
            low,_,high = ranges.partition('-')
            el.set(int(low),int(high),pswd[1:],char[1:])
            #elem_list.append(el)
            yield el

def check_rules(elem):
    low,high = elem.range.low,elem.range.high
    pwd = elem.password
    tok = elem.token
    cnt = pwd.count(tok)
    return (cnt >= low) and (cnt <= high)
        

if __name__ == "__main__":
    gen = parse_input()
    cnnt = 0
    for g in gen:
        cnnt += check_rules(g)
    print(cnnt)

