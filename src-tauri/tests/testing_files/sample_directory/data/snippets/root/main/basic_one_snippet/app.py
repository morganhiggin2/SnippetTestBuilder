#import snippet_module as spm

def init(*args, **kargs):
    snippet = args[0] 
    #snippet.add_input("numbers", "input_numbers_schema.yaml")
    #snippet.add_output("numbers", "output_numbers_schema.yaml")
    snippet.add_input("numbers")
    snippet.add_output("numbers")
    #schema = spm.create_base_schema()

    return snippet;

def run(inputs): 
    number_inputs = inputs['numbers'] 
    number_one = number_inputs['number_one']
    number_two = number_inputs['number_two']

    number_final = number_one + number_two

    number_output = {
        "number": number_final
    }

    outputs = {}
    outputs['numbers'] = number_output

    return outputs