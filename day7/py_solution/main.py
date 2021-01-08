from collections import namedtuple

thing = namedtuple("thing",["bag_obj","qty"])

class bag:
    def __init__(self,bagname):
        self.bagname = bagname
        self.contains = []
    def change_contains(self,bag_obj):
        self.contains.append(bag_obj)
    def __eq__(self,other):
        return hash(self) == hash(other)
    def __hash__(self):
        return hash(self.bagname)
    def __repr__(self):
        ##return f"{self.bagname}  {self.contains} contained_by : {self.contained_by}"
        return f"{self.bagname}"

def get_thing(line):
    if 'no' in line:
        return thing(bag_obj=None,qty=0)
    line = line[1:].split(' ')
    #line = line[:-1]
    ##print(line)    
    bag_name = ' '.join(line[1:])
    bag_obj = get_bag(bag_name)
    return thing(bag_obj=bag_obj,qty=int(line[0]))

def get_bag(line):
    parts = line.split(" ")
    bag_name = ' '.join(parts[:-1])
    ##print(bag)
    bag_obj = bag(bag_name)
    return bag_obj

def line_parser(line,seen):
    bag_name,_,things = line.partition("contain")
    bag_obj = get_bag(bag_name[:-1])
    ##print(f"bag is {bag_n}")
    ##bag = ' '.join(bag.split(" ")[:-1])
    ##print(bag,things)
    things = [get_thing(qt_th.replace('\n','')) for qt_th in things[:-1].split(",")]
    for thing in things:
        seen.setdefault(bag_obj,[]).append(thing)

def recursive_sol(elem,all_bags):
    print("->",elem,end=" ")
    if elem in all_bags:
        return None
    all_bags.add(elem)
    fst = get_contains(elem)
    if len(fst) == 0:
        return None
    for e in fst:
        recursive_sol(e,all_bags)

def get_contains(e):
    #print("getting elem",e)
    first = []
    for k,v in seen.items():
        for t in v:
            ##print(t.bag_obj)
            if t.bag_obj == None:
                continue
            if t.bag_obj.bagname == e:
                #print(f"{k} {seen[k]}")
                first.append(k.bagname)
    return first

class calculate:

    def __init__(self,seen):
        self.seen = seen
        self.cntr = 0
        self.this = 0

    def contains_recurse(self,elem):
        print(elem)
        print("*********************************************************")
        if elem.bag_obj == None:
            return 0
        else:
            #val = 0
            self.cntr = 0
            for t in self.seen[elem.bag_obj]:
                self.cntr += t.qty * self.contains_recurse(t) + t.qty
                print(f"{t.bag_obj} -> {self.cntr}")
            else:
                return self.cntr
            #print(self.contains_recurse(t,elem.qty) + qty)

if __name__ == "__main__":
    f = open("test.txt","r+")
    bag_obj = bag("shiny gold")
    elem = thing(bag_obj=bag_obj,qty=1)
    seen = {}
    for l in f:
        line_parser(l,seen)
    c = calculate(seen)
    things = {}
    graph = {}
    visited = set()
    global_qty = 0
    cntr = c.contains_recurse(elem)
    #build_graph(elem)
    print(cntr)
    f.close()
        

