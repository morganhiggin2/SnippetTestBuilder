#import snippet_module as spm

def init(*args, **kargs):
    snippet = args[0] 
    snippet.add_input("index")
    snippet.add_input("str")
    snippet.add_output("new_str")
    snippet.add_output("original_str")

    return snippet;

def run(inputs): 
    i = inputs["index"]
    s = inputs["str"]

    # check if it is out of bounds
    if i < 0 or i >= len(s):
        raise Exception("index to remove is out of bounds")
    
    final_s = ""

    # remove appropriate part of the string by constructing a new string
    if i == 0:
        final_s = s[1:]
    elif i == len(s) - 1:
        final_s = s[:-1]
    else:
        final_s = s[:i] + s[i+1:]

    outputs = {}
    outputs["new_str"] = final_s 
    outputs["original_str"] = s

    return outputs