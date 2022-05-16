import ast


def get_functions(body):
    funcs = []
    for node in body:
        match node:
            case ast.FunctionDef():
                funcs.append(node)
    return funcs


def has_any_statement(body):
    for node in body:
        match node:
            case ast.Expr():
                return True
            case ast.Assign():
                return True
    return False


def has_function_with_name(body, name):
    for node in body:
        match node:
            case ast.FunctionDef():
                if node.name == name:
                    return True
    return False
