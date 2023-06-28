
def find_and_replace_arguments(lines, variable_name, new_arguments):

    section_reached = False
    for i, line in enumerate(lines):
        if "[start:inputs]" in line:
            section_reached = True
        if section_reached:
            if variable_name in line:
                lines[i] = re.sub('\(.+\)', f'({new_arguments})', line)
                return lines

def convert_values(series):
    """
    Converts a series object into a comma separated list.
    """
    return ", ".join([str(float(val)) for val in series.values])

def insert_in_section(lines, lines_to_insert, start_identyfier, end_identifiyer):
    start_line=0
    end_line=0
    for i,line in enumerate(lines):
        if start_identyfier in line:
            start_line=i
        if end_identifiyer in line:
            end_line=i

    return lines[:start_line+1] + lines_to_insert + lines[end_line:]
