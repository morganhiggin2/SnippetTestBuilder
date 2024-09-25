import importlib
import copy
import sys
import os

# add runables path to sys modules
sys.path.append(os.getcwd())

def run_snippet(*args, **kwargs):
    snippet_path = kwargs["snippet_path"]
    function_inputs: dict[str, any] = kwargs["function_inputs"]
    input_mappings: dict[str, (int, str)] = kwargs["input_mappings"] 
    parameter_values = kwargs["parameter_values"]

    '''
    run...
    :param module_path: path of the module relative to this file
    :param function_inputs: inputs for the snippet mapped to their input name
    :param input_mappings: mapping of each input name to each output id and name 
    :param parameter_values: parameter values
    '''

    # import snippet from other file
    # reload if it has already been loaded
    py_snippet_runnable = importlib.import_module(snippet_path)
    importlib.reload(py_snippet_runnable)

    #call run function from snippet
    outputs = py_snippet_runnable.run(function_inputs, parameter_values)

    '''
    #check types with type parser
    for output_name, output_value in outputs: 
        #check if output_name exists in function_outputs
        if output_name not in function_outputs.keys():
            #raise some exeption: raise ResourceNotFoundExeption
            raise IncorrectFunctionOutputs(snippet_path, output_name) 

        check_type(output_value, function_outputs[output_name])'''

    mapped_outputs = {} 

    # for each output, map it to an output
    for output_name, output_value in outputs.items():
        if output_name in input_mappings:
            # create deep copy
            mapped_outputs[input_mappings[output_name]] = copy.deepcopy(output_value)

    return mapped_outputs 

def check_type(type, data):
    None

class IncorrectFunctionOutputs(Exception):
    def __init__(self, snippet_name: str, missing_output: str):
        self.message = "incorrect values and output names returned from snippet {snippet_name}, could not find output {missing_output} in snippet outputs"
        super().__init__(self.message)















