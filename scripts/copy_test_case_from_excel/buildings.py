from .utils import (
    find_and_replace_arguments,
    convert_values,
    insert_in_section,
    sector_values_padding,
)

def write_test_case_buildings(inputs, measures):

    with open("src/buildings/tests/buildings_test_case.rs", "r") as f:
        lines = f.readlines()

    lines = set_input_variables(inputs, lines)
    lines = set_measures(measures, lines)

    with open("src/buildings/tests/buildings_test_case.rs", "w") as f:
        for line in lines:
            f.write(line)


def set_input_variables(inputs, lines):

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


    return lines


def set_measures(measures, lines):

    def measure_line(measures, row, varname, sector):
        return f'\t{"//" if measures.iloc[row-3, 13] == 0 else ""}buildings.inputs.{varname}.{sector}.add_measure("{varname}", {measures.iloc[row-2, 11]}, {measures.iloc[row-2, 12]}, {measures.iloc[row-2, 9] - measures.iloc[row-2, 10]});\n'

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

    return lines


def write_excel_comparison_buildings(results):

    with open("src/buildings/tests/compare_with_excel.rs") as f:
        lines = f.readlines()

    lines = declare_variables(results, lines)
    lines = write_assert_statements(results, lines)

    with open("src/buildings/tests/compare_with_excel.rs", "w") as f:
        for line in lines:
            f.write(line)



def declare_variables(results, lines):
    """
    This sets varibales like the price for electric energy, which is not calculated
    in the buildings case and therefore need to be set from outside.
    """

    content = []

    for year in results.columns[2:-4]:
        year_values = ','.join(
            [str(results.iloc[349][year]) for i in range(4)]
        )
        content.append(f"\traw_vals=SectorsRawValues::new();\n")
        content.append(f"\traw_vals.set({year_values});\n")
        content.append(f"\tnrg_own_mix_price__m__eur_per_W_h.set_year_values(\n")
        content.append(f"\t\t{year},\n")
        content.append(f"\t\t&raw_vals,\n")
        content.append(f"\t\t);\n\n")

    lines = insert_in_section(lines, content, "[start:declare_variables]", "[end:declare_variables]")

    return lines


def write_assert_statements(results, lines, years=[2022,2023,2024]):
    """
    This adds the assert statements for the output variables.
    """

    assert_lines = ["\n"]

    for variable, i, param_type, n_sectors in [
        ["n_inhabitants__k__", 1, "inputs", 4],
        ["n_buildings", 5, "inputs", 4],
        ["floor_A_building__m2", 10, "inputs", 4],
        ["floor_A__k__m2", 14, "results", 4],
        ["heat_dmd__k__W_h_per_m2_a", 19, "inputs", 4],
        ["hot_water_dmd__k__W_h_per_m2_a", 23, "inputs", 4],
        ["total_heat_dmd__G__W_h_per_a", 27, "results", 4],
        ["elec_dmd_capita__k_W_h_per_a", 32, "inputs", 4],
        ["elec_dmd__G__W_h_per_a", 37, "results", 4],
        ["A_heat_oil__k__m2", 42, "inputs", 4],
        ["A_heat_oil_condensing__k__m2", 47, "inputs", 4],
        ["A_heat_gas__k__m2", 52, "inputs", 4],
        ["A_heat_heat_pump__k__m2", 57, "inputs", 4],
        #["A_heat_other__k__m2", 62, "inputs", 4],
        ["cnsmp_oil__G__W_h_per_a", 67, "results", 4],
        ["cnsmp_oil_condensing__G__W_h_per_a", 72, "results", 4],
        ["cnsmp_oil__M__L_per_a", 77, "results", 4],
        ["cnsmp_gas__G__W_h_per_a", 82, "results", 4],
        ["cnsmp_gas__M__m3_per_a", 87, "results", 4],
        ["cnsmp_elec_heat_pump__G__W_h_per_a", 92, "results", 4],
        ["cnsmp_other__G__W_h_per_a", 97, "results", 4],
        ["costs_oil__M__eur_per_a", 110, "results", 4],
        ["costs_gas__M__eur_per_a", 115, "results", 4],
        ["costs_heat_pump__M__eur", 120, "results", 4],
        ["invest_heat_sources__M__eur_per_a", 141, "results", 4],
        ["invest_energetic_renovation__M__eur_per_a", 146, "results", 4],
        ["grant_heat_sources__M__eur_per_a", 162, "results", 4],
        ["grant_energetic_renovation__M__eur_per_a", 167, "results", 4],
        ["ems_oil__k__to_coe_per_a", 354, "results", 3],
        ["ems_gas__k__to_coe_per_a", 358, "results", 3],
    ]:

        name = str(results.iloc[i,0]).replace('\n',' ')
        assert_lines.append(f"\t// {name}\n")
        for j, year in enumerate(years):
            sector_values = ", ".join(
                sector_values_padding(
                    [str(val if not "nan" in str(val) else 0.0) for val in results.iloc[i:i+n_sectors,j+2].values]
                )
            )
            assert_lines.append("\tassert(\n")
            assert_lines.append(f"\t\tbuildings.{param_type}.{variable}.get_year_values({year}),\n")
            assert_lines.append(f"\t\t[{sector_values}],\n")
            assert_lines.append("\t);\n")
        assert_lines.append("\n")

    lines = insert_in_section(lines, assert_lines, "[start:assert_measures]", "[end:assert_measures]")

    return lines
