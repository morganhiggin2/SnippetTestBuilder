#import snippet_module as spm

def init(*args, **kargs):
    snippet = args[0]
    snippet.add_parameter("num_input", "SingleLineText")
    snippet.add_output("num")

    return snippet;

def run(*args, **kwargs):
    inputs = kwargs['function_inputs']
    params = kwargs['parameter_values']

    s = params["num_input"]

    outputs = {}
    outputs["num"] = float(s)

    return outputs
