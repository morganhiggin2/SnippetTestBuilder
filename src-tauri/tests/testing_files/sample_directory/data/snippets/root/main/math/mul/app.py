#import snippet_module as spm

def init(*args, **kwargs):
    snippet = args[0]
    snippet.add_input("a")
    snippet.add_input("b")
    snippet.add_output("c")

    return snippet;

def run(inputs):
    a = inputs["a"]
    b = inputs["b"]

    outputs = {}
    outputs["c"] = a * b

    return outputs
