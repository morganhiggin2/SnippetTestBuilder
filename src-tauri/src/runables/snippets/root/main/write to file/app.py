#import snippet_module as spm

def init(*args, **kwargs):
    snippet = args[0]
    #snippet.add_input("numbers", "input_numbers_schema.yaml")
    #snippet.add_output("numbers", "output_numbers_schema.yaml")
    snippet.add_input("writable")
    #schema = spm.create_base_schema()

    return snippet;

def run(*args, **kwargs):
    inputs = kwargs['function_inputs']
    params = kwargs['parameter_values']

    writable = str(inputs['writable'])

    f = open('output.txt', 'a')

    f.write(writable)

    return {}
