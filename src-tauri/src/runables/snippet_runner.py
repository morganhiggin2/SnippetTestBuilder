import importlib
import copy
import sys
import os

# add runables path to sys modules
sys.path.append(os.getcwd())

def run_snippet(*args, **kwargs):
    snippet_path = kwargs["snippet_path"]
    input_mappings: dict[str, (int, str)] = kwargs["input_mappings"]
    result_builder = kwargs["result_builder"]

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

    # exception raised during the snippet run, if there was one
    run_exception = False

    # get sub list of kwargs for function call
    run_kwargs = {k: v for k, v in kwargs.items() if k in ('logger', 'function_inputs', 'parameter_values')}

    #call run function from snippet
    # handle any exeptions

    try:
        outputs = py_snippet_runnable.run(*args, **run_kwargs)
    except Exception as e:

        # log exception
        # return false for success
        result_builder.logger.log_err(str(e))

        run_exception = True

    '''
    #check types with type parser
    for output_name, output_value in outputs:
        #check if output_name exists in function_outputs
        if output_name not in function_outputs.keys():
            #raise some exeption: raise ResourceNotFoundExeption
            raise IncorrectFunctionOutputs(snippet_path, output_name)

        check_type(output_value, function_outputs[output_name])'''

    # If there was no runtime exception
    if run_exception is False:
        mapped_outputs = {}

        # for each output, map it to an output
        for output_name, output_value in outputs.items():
            if output_name in input_mappings:
                # create deep copy
                mapped_outputs[input_mappings[output_name]] = copy.deepcopy(output_value)

        result_builder.set_successful_result(mapped_outputs)
    else:
        result_builder.set_exception_result()

    return result_builder

def check_type(type, data):
    None

class IncorrectFunctionOutputs(Exception):
    def __init__(self, snippet_name: str, missing_output: str):
        self.message = "incorrect values and output names returned from snippet {snippet_name}, could not find output {missing_output} in snippet outputs"
        super().__init__(self.message)
