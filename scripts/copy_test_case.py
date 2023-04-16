import sys
import pandas as pd
import numpy as np
import re


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


inputs = pd.read_excel(sys.argv[1], sheet_name="Eingabe Ist")
results = pd.read_excel(sys.argv[1], sheet_name="Rechenwerk")

#
# Buildings
#

# Inputs

with open("src/buildings/tests/buildings_test_case.rs", "r") as f:
    lines = f.readlines()

os = -2

# EW bzw. MA bzw. SuS (in 1.000)
lines = find_and_replace_arguments(lines, "n_inhabitants__k__", convert_values(inputs.iloc[5+os, 1:5]))
# Anzahl Gebäude
lines = find_and_replace_arguments(lines, "n_buildings", convert_values(inputs.iloc[6+os, 1:5]))
# qm Geschossfläche/Gebäude
lines = find_and_replace_arguments(lines, "floor_A_building__m2", convert_values(inputs.iloc[7+os, 1:5]))

# Heizbedarf (in kWh/qm/a)
lines = find_and_replace_arguments(lines, "heat_dmd__k__W_h_per_m2_a", convert_values(inputs.iloc[9+os, 1:5]))
# WW-Bedarf je EW bzw. MA bzw. SuS (in kWh/a)
lines = find_and_replace_arguments(lines, "hot_water_dmd__k__W_h_per_m2_a", convert_values(inputs.iloc[10+os, 1:5]))
# Strombedarf Gebäude außer WP/ePkw je EW bzw. MA bzw. SuS (in kWh/a)
lines = find_and_replace_arguments(lines, "elec_dmd_capita__k_W_h_per_a", convert_values(inputs.iloc[18+os, 1:5]))

# Fläche mit Ölheizung ohne Brennwert/Niedertemperatur (in 1.000 qm)
lines = find_and_replace_arguments(lines, "A_heat_oil__k__m2", convert_values(inputs.iloc[12+os, 1:5]))
# Fläche mit Ölheizung mit Brennwert/Niedertemperatur (in 1.000 qm)
lines = find_and_replace_arguments(lines, "A_heat_oil_condensing__k__m2", convert_values(inputs.iloc[13+os, 1:5]))
# Fläche mit Gasheizung (in 1.000 qm)
lines = find_and_replace_arguments(lines, "A_heat_gas__k__m2", convert_values(inputs.iloc[14+os, 1:5]))
# Fläche mit Wärmepumpen-Heizung (in 1.000 qm)
lines = find_and_replace_arguments(lines, "A_heat_heat_pump__k__m2", convert_values(inputs.iloc[15+os, 1:5]))
# Fläche mit anderer Wärmequelle (in 1.000 qm)
lines = find_and_replace_arguments(
    lines,
    "A_heat_other__k__m2",
    convert_values(
        inputs.iloc[7+os, 1:5] * inputs.iloc[6+os, 1:5] * 1e-3
        - inputs.iloc[12+os, 1:5]
        - inputs.iloc[13+os, 1:5]
        - inputs.iloc[14+os, 1:5]
        - inputs.iloc[15+os, 1:5]
    ),
)

with open("src/buildings/tests/buildings_test_case.rs", "w") as f:
    for line in lines:
        f.write(line)

# Declare Variables
with open("src/buildings/tests/compare_with_excel.rs") as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "[start:declare_variables]" in line:
        section_start = i
    if "[end:declare_variables]" in line:
        section_end = i

content = ""

lines = lines[:section_start+1] + content + lines[section_end:]

with open("src/buildings/tests/compare_with_excel.rs", "w") as f:
    for line in lines:
        f.write(line)

# Assert Inputs/Measures
with open("src/buildings/tests/compare_with_excel.rs") as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "[start:assert_measures]" in line:
        section_start = i
    if "[end:assert_measures]" in line:
        section_end = i

assert_lines = ["\n"]

for variable, i, param_type in [
    ["n_inhabitants__k__", 1, "inputs"],
    ["n_buildings", 5, "inputs"],
    ["floor_A_building__m2", 10, "inputs"],
    ["floor_A__k__m2", 14, "results"],
    ["heat_dmd__k__W_h_per_m2_a", 19, "inputs"],
    ["hot_water_dmd__k__W_h_per_m2_a", 23, "inputs"],
    ["total_heat_dmd__G__W_h_per_a", 27, "results"],
    ["elec_dmd_capita__k_W_h_per_a", 32, "inputs"],
    ["elec_dmd__G__W_h_per_a", 37, "results"],
    ["A_heat_oil__k__m2", 42, "inputs"],
    ["A_heat_oil_condensing__k__m2", 47, "inputs"],
    ["A_heat_gas__k__m2", 52, "inputs"],
    ["A_heat_heat_pump__k__m2", 57, "inputs"],
    ["A_heat_other__k__m2", 62, "inputs"],
    ["cnsmp_oil__G__W_h_per_a", 67, "results"],
    ["cnsmp_oil_condensing__G__W_h_per_a", 72, "results"],
    ["cnsmp_oil__M__L_per_a", 77, "results"],
    ["cnsmp_gas__G__W_h_per_a", 82, "results"],
    ["cnsmp_gas__M__m3_per_a", 87, "results"],
    ["cnsmp_elec_heat_pump__G__W_h_per_a", 92, "results"],
    ["cnsmp_other__G__W_h_per_a", 97, "results"],
    ["costs_oil__M__eur_per_a", 110, "results"],
    ["costs_gas__M__eur_per_a", 115, "results"],
    ["invest_heat_sources__M__eur_per_a", 141, "results"],
    ["invest_energetic_renovation__M__eur_per_a", 146, "results"],
    ["grant_heat_sources__M__eur_per_a", 162, "results"],
    ["grant_energetic_renovation__M__eur_per_a", 167, "results"],
    ["costs_heat_pump__M__eur", 122, "results"],
]:

    name = str(results.iloc[i,0]).replace('\n',' ')
    assert_lines.append(f"\t// {name}\n")
    for j, year in enumerate([2022,2023,2024,2025]):
        sector_values = ", ".join([str(val if not "nan" in str(val) else 0.0) for val in results.iloc[i:i+4,j+2].values])
        assert_lines.append("\tassert(\n")
        assert_lines.append(f"\t\tbuildings.{param_type}.{variable}.get_year_values({year}),\n")
        assert_lines.append(f"\t\t[{sector_values}],\n")
        assert_lines.append("\t);\n")
    assert_lines.append("\n")

lines = lines[:section_start+1] + assert_lines + lines[section_end:]

with open("src/buildings/tests/compare_with_excel.rs", "w") as f:
    for line in lines:
        f.write(line)










