def init(*args, **kwargs):
    snippet = args[0]

    return snippet

def run(inputs, params):
    import sys 

    with open('../more_output.txt', 'w') as f:
        f.write(str(sys.path));

    return {}