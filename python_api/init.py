#each snippet code contains 
    #init 
        #that specifies property names and type (ex. url: url-string, retires: integer)
        #specifies input and output of snippet
        #specifiy dependencies
    #run(params) that specify the params, where the params are what are specified in init
        #includes error function to send error to our backend
        #includes successful to send to our backend

#we treat all snippets as one dependency
#when a snippet is written, we must include dependency in init 
    #this will consult the group dependencies, and call npm install if we do not have it

'''
math_test_1.py
-------------------------

import pandas
import numpy
import rusties

class MyOwnSnippet:
    var retries;
    var start;

    def init(snippet):
        #register retires as int, snippet.{function_name}
        #register start as int 
        
        #register inputs, n as int
        #register outputs, o as int, o2 as int

    def run(input):
        var num = start + n
        while(reties):
            num *= 2
        
        if num < 10:
            #return error
        
        #return succeed
        #return o = num / 2
        #return o = 1
'''

'''
rusties.py
------------------------


'''

#1. read files in directory
    #2. run init in directory to register snippets, noting location
        #--this is a rust registered python module
        #creates blank snippet based on class name, creates obj, injects it as 'snippet' callable into init function
        #this calls built in rust functions, which change the internal rust snippet
#3. run snippets when need to, injecting values into run function 
    #assemble params in format (TBD)
    #ingect input into run function input param
    #call run function from python script with (TBD)
    #wait for result, which is Result enum wrapped in python value
        #to match return value type
        #in rust, cast based on type (if special, like json schema or string-url, do checks)


#rust type -> py type -> snippet 1 -> pytype -> snippet 2 ... -> pytype -> rust type
#applies to inputs, outputs, and params



#---this class serves as wrapper for rust python functions


def create_snippet(name):
    #call rust function to create snippet
    None

#get date_type
    #gets string from rust
    #converts to date