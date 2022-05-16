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
