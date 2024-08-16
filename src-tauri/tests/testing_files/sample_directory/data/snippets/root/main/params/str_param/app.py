#import snippet_module as spm

def init(*args, **kargs):
    snippet = args[0] 
    snippet.add_parameter("str_input", "SingleLineText")
    snippet.add_output("str")

    return snippet;

def run(inputs): 
    s = inputs["str_input"]

    outputs = {}
    outputs["str"] = s 

    return outputs