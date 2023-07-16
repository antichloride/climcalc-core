import re
import pandas as pd

def find_and_replace_arguments(lines, variable_name, new_arguments):

    section_reached = False
    for i, line in enumerate(lines):
        if "[start:inputs]" in line:
            section_reached = True
        if section_reached:
            if variable_name in line:
                lines[i] = re.sub('\(.+\)', f'({new_arguments})', line)
                return lines

def convert_values(series=None):
    """
    Converts a series object into a comma separated list.
    """
    if type(series)==type(None):
        values = [0.0, 0.0, 0.0, 0.0]
    else:
        values = series.values

        if len(values) == 3:
            values = [values[0], values[1], 0.0, values[2]]
        if len(values) == 2:
            values = [0.0, values[0], 0.0, values[1]]
        if len(values) == 1:
            values = [0.0, 0.0, 0.0, values[0]]
        if len(values) == 0:
            values = [0.0, 0.0, 0.0, 0.0]
    return ", ".join([str(float(val)) for val in values])

def insert_in_section(lines, lines_to_insert, start_identyfier, end_identifiyer):
    start_line=None
    end_line=None
    for i,line in enumerate(lines):
        if start_identyfier in line:
            start_line=i
        if end_identifiyer in line:
            end_line=i

    if start_line == None or end_line == None:
        error_msg = ""
        if start_line == None:
            error_msg = f"{start_identyfier} was not found."
        if end_line == None:
            error_msg = f"{error_msg} {end_identifiyer} was not found."
        raise Exception(error_msg)

    return lines[:start_line+1] + lines_to_insert + lines[end_line:]

