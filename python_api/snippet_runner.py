import importlib

def run_snippet(snippet_path, function_inputs: dict[str, any], function_outputs: dict[str, str], argument_inputs: dict[str, any]):
    '''
    run...
    :param module_path: path of the module relative to this file
    :param function_outputs: dictionary with names of the outputs with their corresponding output data type schema 
    '''
    #import snippet from other file
    py_snippet = importlib.import_module(snippet_path)

    #combine function inputs and function arguments
    snippet_inputs = function_outputs | argument_inputs

    #call run function from snippet
    outputs = py_snippet(**snippet_inputs)

    #check types with type parser
    for output_name, output_value in outputs: 
        #check if output_name exists in function_outputs
        if output_name not in function_outputs.keys():
            #raise some exeption: raise ResourceNotFoundExeption
            raise IncorrectFunctionOutputs(snippet_path, output_name) 

        check_type(output_value, function_outputs[output_name])

    return outputs

def check_type(type, data):
    None

class IncorrectFunctionOutputs(Exception):
    def __init__(self, snippet_name: str, missing_output: str):
        self.message = "incorrect values and output names returned from snippet {snippet_name}, could not find output {missing_output} in snippet outputs"
        super().__init__(self.message)
















