import importlib

def run():
    py_snippet_runnable = importlib.import_module('runables.snippets.root.main.basic_one_snippet.app', "run")

run()