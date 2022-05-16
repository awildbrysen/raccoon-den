import ast

def get_functions(body):
    functions = []
    for node in body:
        match node:
            case ast.FunctionDef():
                functions.append(node)
    return functions

def has_any_statement(body):
    for node in body:
        match node:
            case ast.Expr:
                return True
    return False