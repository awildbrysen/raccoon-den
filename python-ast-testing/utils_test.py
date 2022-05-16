from utils import get_functions, has_any_statement
import ast


code_with_just_2_functions = """
def sum(a, b):
    return a + b

def subtr(a, b):
    return a - b
"""

code_with_2_functions_and_a_statement = """
def sum(a, b):
    return a + b

def subtr(a, b):
    return a - b

print("hello")
"""

def test_that_only_2_methods_are_present():
    tree = ast.parse(code_with_2_functions_and_a_statement)
    assert len(get_functions(tree.body)) == 2
    assert not has_any_statement(tree.body)
