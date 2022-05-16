from utils import get_functions, has_any_statement, has_function_with_name
import ast

code_with_only_2_functions = """
def sum(a, b):
    return a + b

def subtr(a, b):
    return a - b
"""

code_with_2_functions_and_a_expression = """
def sum(a, b):
    return a + b

def subtr(a, b):
    return a - b

print("hello")
"""

code_with_2_functions_and_assignments = """
def sum(a, b):
    return a + b

def subtr(a, b):
    return a - b

a = 1
b = ""
"""

code_with_2_functions_and_constants = """
def sum(a, b):
    return a + b

def subtr(a, b):
    return a - b

""
True
"""


def test_that_2_methods_are_present():
    assert len(get_functions(ast.parse(code_with_2_functions_and_a_expression).body)) == 2


def test_that_there_are_no_statements():
    assert not has_any_statement(ast.parse(code_with_only_2_functions).body)
    assert has_any_statement(ast.parse(code_with_2_functions_and_a_expression).body)
    assert has_any_statement(ast.parse(code_with_2_functions_and_assignments).body)
    assert has_any_statement(ast.parse(code_with_2_functions_and_constants).body)


def test_has_function_with_name():
    tree = ast.parse(code_with_only_2_functions)
    assert has_function_with_name(tree.body, "sum")
    assert not has_function_with_name(tree.body, "test")
