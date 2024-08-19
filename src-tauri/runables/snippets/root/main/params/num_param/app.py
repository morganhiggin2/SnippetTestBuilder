#import snippet_module as spm

def init(*args, **kargs):
    snippet = args[0] 
    snippet.add_parameter("num_input", "SingleLineText")
    snippet.add_output("num")

    return snippet;

def run(inputs, params): 
    s = params["num_input"]

    outputs = {}
    outputs["num"] = s + '4'

    return outputs