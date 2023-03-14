import sys
import pandas as pd
import re

def find_and_replace_arguments(lines, variable_name, new_arguments):

    section_reached = False
    for i, line in enumerate(lines):
        if "Define Inputs" in line:
            section_reached = True
        if section_reached:
            if variable_name in line:
                lines[i] = re.sub('\(.+\)', f'({new_arguments})', line)
                return lines

def convert_values(series):
    return ", ".join([str(float(val)) for val in series.values])

inputs = pd.read_excel(sys.argv[1], sheet_name="Eingabe Ist")
results = pd.read_excel(sys.argv[1], sheet_name="Rechenwerk")


#
# Buildings
#

# Inputs

with open("src/buildings/tests/buildings_test_case.rs", "r") as f:
    lines = f.readlines()

# EW bzw. MA bzw. SuS (in 1.000)
lines = find_and_replace_arguments(lines, "n_inhabitants__k__", convert_values(inputs.iloc[4, 1:5]))
# Anzahl Gebäude
lines = find_and_replace_arguments(lines, "n_buildings", convert_values(inputs.iloc[5, 1:5]))
# qm Geschossfläche/Gebäude
lines = find_and_replace_arguments(lines, "floor_A_building__m2", convert_values(inputs.iloc[6, 1:5]))

# Heizbedarf (in kWh/qm/a)
lines = find_and_replace_arguments(lines, "heat_dmd__k__W_h_per_m2_a", convert_values(inputs.iloc[8, 1:5]))
# WW-Bedarf je EW bzw. MA bzw. SuS (in kWh/a)
lines = find_and_replace_arguments(lines, "hot_water_dmd__k__W_h_per_m2_a", convert_values(inputs.iloc[9, 1:5]))
# Strombedarf Gebäude außer WP/ePkw je EW bzw. MA bzw. SuS (in kWh/a)
lines = find_and_replace_arguments(lines, "elec_dmd_capita__k_W_h_per_a", convert_values(inputs.iloc[17, 1:5]))

# Fläche mit Ölheizung ohne Brennwert/Niedertemperatur (in 1.000 qm)
lines = find_and_replace_arguments(lines, "A_heat_oil__k__m2", convert_values(inputs.iloc[11, 1:5]))
# Fläche mit Ölheizung mit Brennwert/Niedertemperatur (in 1.000 qm)
lines = find_and_replace_arguments(lines, "A_heat_oil_condensing__k__m2", convert_values(inputs.iloc[12, 1:5]))
# Fläche mit Gasheizung (in 1.000 qm)
lines = find_and_replace_arguments(lines, "A_heat_gas__k__m2", convert_values(inputs.iloc[13, 1:5]))
# Fläche mit Wärmepumpen-Heizung (in 1.000 qm)
lines = find_and_replace_arguments(lines, "A_heat_heat_pump__k__m2", convert_values(inputs.iloc[14, 1:5]))
# Fläche mit anderer Wärmequelle (in 1.000 qm)
lines = find_and_replace_arguments(lines, "A_heat_other__k__m2", convert_values(inputs.iloc[16, 1:5]))

with open("src/buildings/tests/buildings_test_case.rs", "w") as f:
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

for variable, i in [
    ["n_inhabitants__k__", 1]
]:
    assert_lines.append(f"\t// {results.iloc[i,0]}\n")
    for j, year in enumerate([2022,2023,2024,2025]):
        sector_values = ", ".join([str(val) for val in results.iloc[i:i+4,j+2].values])
        assert_lines.append("\tassert(\n")
        assert_lines.append(f"\t\tbuildings.inputs.{variable}.get_year_values({year}),\n")
        assert_lines.append(f"\t\t[{sector_values}],\n")
        assert_lines.append("\t);\n")
    assert_lines.append("\n")

lines = lines[:section_start+1] + assert_lines + lines[section_end:]

with open("src/buildings/tests/compare_with_excel.rs", "w") as f:
    for line in lines:
        f.write(line)










