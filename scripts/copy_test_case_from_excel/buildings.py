from .utils import find_and_replace_arguments
from .utils import convert_values
from .utils import insert_in_section

def write_test_case_buildings(inputs, measures):

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
    # # Fläche mit anderer Wärmequelle (in 1.000 qm)
    # lines = find_and_replace_arguments(
    #     lines,
    #     "A_heat_other__k__m2",
    #     convert_values(
    #         inputs.iloc[7+os, 1:5] * inputs.iloc[6+os, 1:5] * 1e-3
    #         - inputs.iloc[12+os, 1:5]
    #         - inputs.iloc[13+os, 1:5]
    #         - inputs.iloc[14+os, 1:5]
    #         - inputs.iloc[15+os, 1:5]
    #     ),
    # )
    # breakpoint()


    # # set other values for all years
    # others_input = []

    # for year in results.columns[2:-4]:
    #     others_input.append(f"\tlet mut raw_vals: SectorsRawValues;\n")
    #     others_input.append(f"\traw_vals.set({results[year][125]}, {results[year][126]}, {results[year][127]}, {results[year][128]});\n")
    #     others_input.append(f"\tbuildings.inputs.A_heat_other__k__m2.set_year_values({year}, &raw_vals);\n\n")

    # start_set_others=0
    # end_set_others=0
    # for i,line in enumerate(lines):
    #     if "[start:set_others]" in line:
    #         start_set_others=i
    #     if "[end:set_others]" in line:
    #         end_set_others=i

    # lines = lines[:start_set_others+1] + others_input + lines[end_set_others:]

    def measure_line(measures, row, varname, sector):
        return f'\t{"//" if measures.iloc[row-2, 13] == 0 else ""}buildings.inputs.{varname}.{sector}.add_measure("{varname}", {measures.iloc[row-2, 11]}, {measures.iloc[row-2, 12]}, {measures.iloc[row-2, 9] - measures.iloc[row-2, 10]});\n'

    measures_input = []
    # Set Measures
    measures_input.append("\n")
    measures_input.append("\t//Private\n")
    measures_input.append(measure_line(measures, 14, "heat_dmd__k__W_h_per_m2_a", "private"))
    measures_input.append(measure_line(measures, 15, "A_heat_oil__k__m2", "private"))
    measures_input.append(measure_line(measures, 16, "A_heat_oil_condensing__k__m2", "private"))
    measures_input.append(measure_line(measures, 17, "A_heat_gas__k__m2", "private"))
    measures_input.append(measure_line(measures, 18, "A_heat_heat_pump__k__m2", "private"))
    measures_input.append("\n")

    measures_input.append("\t//Industry\n")
    measures_input.append(measure_line(measures, 24, "heat_dmd__k__W_h_per_m2_a", "industry"))
    measures_input.append(measure_line(measures, 25, "A_heat_oil__k__m2", "industry"))
    measures_input.append(measure_line(measures, 26, "A_heat_oil_condensing__k__m2", "industry"))
    measures_input.append(measure_line(measures, 27, "A_heat_gas__k__m2", "industry"))
    measures_input.append(measure_line(measures, 28, "A_heat_heat_pump__k__m2", "industry"))
    measures_input.append("\n")

    measures_input.append("\t//Schools\n")
    measures_input.append(measure_line(measures, 36, "heat_dmd__k__W_h_per_m2_a", "schools"))
    measures_input.append(measure_line(measures, 37, "A_heat_oil__k__m2", "schools"))
    measures_input.append(measure_line(measures, 38, "A_heat_oil_condensing__k__m2", "schools"))
    measures_input.append(measure_line(measures, 39, "A_heat_gas__k__m2", "schools"))
    measures_input.append(measure_line(measures, 40, "A_heat_heat_pump__k__m2", "schools"))
    measures_input.append("\n")

    measures_input.append("\t//Public\n")
    measures_input.append(measure_line(measures, 42, "heat_dmd__k__W_h_per_m2_a", "public"))
    measures_input.append(measure_line(measures, 43, "A_heat_oil__k__m2", "public"))
    measures_input.append(measure_line(measures, 44, "A_heat_oil_condensing__k__m2", "public"))
    measures_input.append(measure_line(measures, 45, "A_heat_gas__k__m2", "public"))
    measures_input.append(measure_line(measures, 46, "A_heat_heat_pump__k__m2", "public"))
    measures_input.append("\n")

    lines = insert_in_section(lines, measures_input, "[start:measures]", "[end:measures]")


    with open("src/buildings/tests/buildings_test_case.rs", "w") as f:
        for line in lines:
            f.write(line)
