def init(*args, **kargs):
    snippet = args[0]
    snippet.add_input("a")
    snippet.add_input("b")
    snippet.add_output("c")

    return snippet;

def run(*args, **kwargs):
    inputs = kwargs['function_inputs']
    params = kwargs['parameter_values']

    a = inputs["a"]
    b = inputs["b"]

    outputs = {}
    outputs["c"] = a + b

    return outputs
